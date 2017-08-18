//! a drawing context
//! each canvas has exactly one

use color::{self, CanvasColor};

pub struct Context {
    fill_color: color::Rgba,
    pub(crate) window: ::pipeline::Window<::gfx_device_gl::Device>,
}

impl Context {
    pub fn new(width: u32, height: u32, ev_loop: &::glutin::EventsLoop) -> Self {
        let window = ::pipeline::init(width, height, ev_loop);
        
        Context {
            fill_color: Default::default(),
            window,
        }
    }

    /// set fill color
    pub fn fill_color<C>(&mut self, color: C) 
        where C: CanvasColor
    {
        self.fill_color = (color.as_rgb(), color.alpha()).into();
    }

    /// fill rectangle
    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {

        let data: Vec<_> = (0..width*height).map(|_| {
            self.fill_color.into_gpu(None)
        }).collect();

        self.window.update_canvas(x, y, width, height, &data);
    }
}