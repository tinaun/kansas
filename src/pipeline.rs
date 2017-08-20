use gfx::traits::{Factory, FactoryExt};
use gfx::format::{self, Formatted};
use gfx::{self, handle, texture, Device};
use gfx_window_glutin as gfx_glutin;
use glutin::{self, GlContext};


// backend stuff

const CANVAS: [Vertex; 4] =  [
    Vertex { pos: [1.0, -1.0], uv: [1.0, 1.0] },
    Vertex { pos: [-1.0, -1.0], uv: [0.0, 1.0] },
    Vertex { pos: [-1.0, 1.0], uv: [0.0, 0.0] },
    Vertex { pos: [1.0, 1.0], uv: [1.0, 0.0] },
];

const CANVAS_INDEX: &[u16] = &[0, 1, 2, 2, 3, 0];

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type Texture<R> = handle::Texture<R, <ColorFormat as Formatted>::Surface>;

type TexWithView<R> = (Backing, handle::Texture<R, <ColorFormat as Formatted>::Surface>, handle::ShaderResourceView<R, [f32; 4]>);

/// texture backing data
#[derive(Debug, Clone)]
struct Backing {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

fn create_texture<F, R>(factory: &mut F, width: u32, height: u32) -> TexWithView<R>
    where F: gfx::Factory<R>, R: gfx::Resources
{
    use gfx::texture::*;

    let (width, height) = (width.next_power_of_two(), height.next_power_of_two());
    let mut data: Vec<u8> = vec![];

    for i in 0..height {
        for j in 0..width {
            //let k = ((i + j) % 256) as u8;
            //data.extend(&[0xFF, 0, 0, 0xFF]);

            if (i + j) % 2 == 0 {
                data.extend(&[0x00, 0x00, 0x00, 0xFF]);
            } else {
                data.extend(&[0xFF, 0xFF, 0xFF, 0xFF]);
            }
        }
    }

    let kind = Kind::D2(width as u16, height as u16, AaMode::Single);
    // let gpu_handles = factory.create_texture_immutable_u8::<ColorFormat>(kind, &[&data])
        // .expect("error creating canvas backing texture");
    // need to set memory dynamic to update textures

    let surface = <<ColorFormat as Formatted>::Surface as format::SurfaceTyped>::get_surface_type();
    let channel = <<ColorFormat as Formatted>::Channel as format::ChannelTyped>::get_channel_type();
    let desc = texture::Info {
        kind,
        levels: 1 as texture::Level,
        format: surface,
        bind: gfx::SHADER_RESOURCE | gfx::RENDER_TARGET,
        usage: gfx::memory::Usage::Dynamic,
    };
    let raw = factory.create_texture_raw(desc, Some(channel), Some(&[&data]))
                .expect("texture creation error!");

    let levels = (0, raw.get_info().levels - 1);
    let tex = gfx::memory::Typed::new(raw);

    let view = factory.view_texture_as_shader_resource::<ColorFormat>(&tex, levels, format::Swizzle::new())
                .expect("texture view error");

    let backing = Backing { width, height, data };

    (backing, tex, view)
}

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        uv: [f32; 2] = "a_UV",
    }

    pipeline cpipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        view: gfx::Global<[f32; 2]> = "i_View",
        canvas: gfx::TextureSampler<[f32; 4]> = "t_Canvas",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
        depth: gfx::DepthTarget<DepthFormat> = Default::default(),
    }
}

pub type GlWindow = Window<::gfx_device_gl::Device, ::gfx_device_gl::Factory>;
pub struct Window<D: Device, F: Factory<D::Resources>> {
    window: glutin::GlWindow,
    device: D,
    factory: F,
    backing: Backing,
    _texture: Texture<D::Resources>,
    encoder: gfx::Encoder<D::Resources, D::CommandBuffer>,
    pipeline: gfx::PipelineState<D::Resources, cpipe::Meta>,
    data: cpipe::Data<D::Resources>,
    slice: gfx::Slice<D::Resources>,
}

pub fn init(width: u32, height: u32, ev_loop: &glutin::EventsLoop) 
    -> GlWindow
{
    
    let builder = glutin::WindowBuilder::new()
        .with_title("canvas")
        .with_dimensions(width, height);
    let ctx = glutin::ContextBuilder::new();
    let (window, device, mut factory, color, depth) = gfx_glutin::init::<ColorFormat, DepthFormat>(builder, ctx, ev_loop);
    
    // lets see what we've got
    //println!("{:?}", device.get_info());

    //create data on the gpu
    let (tex_backing, tex_handle, tex_view) = create_texture(&mut factory, width, height);
    let tex_sampler = factory.create_sampler({
        let mut s = texture::SamplerInfo::new(texture::FilterMethod::Scale, texture::WrapMode::Border);
        s.border = [0.0, 1.0, 0.0, 1.0].into();
        s
    });

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&CANVAS, CANVAS_INDEX);

    let pipeline = factory.create_pipeline_simple(
        include_bytes!("../static/vert.glsl"),
        include_bytes!("../static/frag.glsl"),
        cpipe::new()
    ).expect("error creating shader pipeline");

    let view = [
        width as f32 / tex_backing.width as f32,
        height as f32 / tex_backing.height as f32
    ];

    let data = cpipe::Data {
        vbuf: vertex_buffer,
        view,
        canvas: (tex_view, tex_sampler),
        out: color,
        depth,
    };

    let encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();

    Window {
        window,
        device,
        factory,
        backing: tex_backing,
        _texture: tex_handle,
        encoder,
        pipeline,
        data,
        slice,
    }
}

impl<D: gfx::Device, F: Factory<D::Resources>> Window<D, F> {
    pub fn draw(&mut self) {
        self.encoder.clear(&self.data.out, [0.0, 0.0, 0.0, 1.0]);
        self.encoder.draw(&self.slice, &self.pipeline, &self.data);

        self.encoder.flush(&mut self.device);

        self.window.swap_buffers().unwrap();
        self.device.cleanup();
    }

    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width > self.backing.width || new_height > self.backing.height {
            let (tb, th, tv) = create_texture(&mut self.factory, new_width, new_height);
            self.data.canvas.0 = tv;
            self._texture = th;
            self.backing = tb;
            println!("created larger texture!");
        }
        
        
        self.data.view = [
            new_width as f32 / self.backing.width as f32,
            new_height as f32 / self.backing.height as f32
        ];

        //println!("{:?}", self.data.view);
    }

    pub fn update_canvas(&mut self, x: u32, y: u32, width: u32, height: u32, data: &[[u8; 4]]) {
        let bounds = texture::NewImageInfo {
            xoffset: x as u16,
            yoffset: y as u16,
            zoffset: 0,
            width: width as u16,
            height: height as u16,
            depth: 0,
            format: (),
            mipmap: 0,
        };

        self.encoder.update_texture::<
            <ColorFormat as Formatted>::Surface,
            ColorFormat
        >(&self._texture, None, bounds, data).expect("painting error");
    }

    pub fn update_views<C>(&mut self, f: C) 
        where C: Fn( &glutin::GlWindow,
                    &mut handle::RenderTargetView<D::Resources, ColorFormat>,
                    &mut handle::DepthStencilView<D::Resources, DepthFormat> ) -> ()
    {
        f(&self.window, &mut self.data.out, &mut self.data.depth);
    }
}