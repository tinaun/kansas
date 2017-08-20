//! a drawing context
//! each canvas has exactly one

use color::{self, CanvasColor};

pub struct Context {
    width: u32,
    height: u32,
    fill_color: color::Rgba,
    pub(crate) window: ::pipeline::Window<::gfx_device_gl::Device>,
}

impl Context {
    pub fn new(width: u32, height: u32, ev_loop: &::glutin::EventsLoop) -> Self {
        let window = ::pipeline::init(width, height, ev_loop);
        
        Context {
            width,
            height,
            fill_color: Default::default(),
            window,
        }
    }

    /// dimensions
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub(crate) fn resize(&mut self, w: u32, h: u32) {
        self.window.resize(w, h);
        self.width = w;
        self.height = h;
    }

    /// set fill color
    pub fn fill_color<C>(&mut self, color: C) 
        where C: CanvasColor
    {
        self.fill_color = (color.as_rgb(), color.alpha()).into();
    }

    /// fill rectangle with preset fill color
    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let (x_extra, y_extra) = (
            (x + width).saturating_sub(self.width),
            (y + height).saturating_sub(self.height)
        );

        let width = width - x_extra;
        let height = height - y_extra;

        let data: Vec<_> = (0..width*height).map(|_| {
            self.fill_color.into_gpu(None)
        }).collect();

        self.window.update_canvas(x, y, width, height, &data);
    }

    /// fill rectangle using a fill function 
    ///
    /// 

    pub fn fill_by<C, F>(&mut self, x: u32, y: u32, width: u32, height: u32, f: F) 
        where F: Fn(u32, u32) -> C,
              C: CanvasColor
    {
        if x >= self.width || y >= self.height {
            return;
        }

        let (x_extra, y_extra) = (
            (x + width).saturating_sub(self.width),
            (x + height).saturating_sub(self.height)
        );

        let width = width - x_extra;
        let height = height - y_extra;

        let mut data: Vec<_> = Vec::new();

        for j in y .. y + height {
            for i in x .. x + width {
                let color = f(i, j).into_gpu(None);
                data.push(color);
            }
        }

        self.window.update_canvas(x, y, width, height, &data);
    }

    /// set a pixel to a specific color
    ///
    /// this does not change the internal color values of the context.
    pub fn set_pixel<C: CanvasColor>(&mut self, x: u32, y: u32, c: C) {
        if x >= self.width || y >= self.height {
            return;
        }
        
        let data = [c.into_gpu(None)];

        self.window.update_canvas(x, y, 1, 1, &data);
    }
}