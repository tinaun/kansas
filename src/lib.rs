// 2017 tinaun
//! a simple canvas-like drawing api for putting pixels on a screen
//! ```rust
//! extern crate wcanvas;
//! use wcanvas::Canvas;  
//!
//! let ctx = Canvas::new(640, 480);
//!
//! ctx.fill_color(0x0000FF);
//! ctx.fill_rect(40, 80, 100, 200);
//! ctx.pause();
//! ```
//!
#[macro_use]
extern crate gfx;

extern crate gfx_window_glutin;
extern crate gfx_device_gl;

extern crate glutin;

pub mod color;
pub mod events;
mod pipeline;

use color::CanvasColor;

pub struct Canvas {
    fill_color: color::Rgba,
    ev_loop: glutin::EventsLoop,
    window: pipeline::Window<gfx_device_gl::Device>,
    listeners: events::ActiveListeners,
}

impl Canvas {

    /// create a new Canvas
    pub fn new() -> Self {
        let ev_loop = glutin::EventsLoop::new();
        let window = pipeline::init(800, 600, &ev_loop);

        Canvas {
            fill_color: Default::default(),
            ev_loop,
            window,
            listeners: events::ActiveListeners::new(),
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

    /// register a function to be used as an event handler
    /// 
    /// note: as of right now, registering the same event twice will overwrite the previous handler
    /// function
    pub fn on<F, E: events::Listener>(&mut self, handler: F) 
        where F: FnMut(E::Event) + 'static
    {
        self.listeners.add::<E>(Box::new(handler));
    }

    pub fn off<E: events::Listener>(&mut self) {
        self.listeners.remove(E::event_id());
    }

    /// hold execution until user hits `Esc`
    pub fn pause(&mut self) {
        let mut running = true;
        let window = &mut self.window;

        while running {
            self.ev_loop.poll_events(|e| {
                use glutin::Event::WindowEvent;
                use glutin::VirtualKeyCode;
                use glutin::WindowEvent::*;

                fn is_break(input: glutin::KeyboardInput) -> bool {
                    input.virtual_keycode == Some(VirtualKeyCode::Escape) ||
                    (input.state == glutin::ElementState::Pressed && 
                    input.virtual_keycode == Some(VirtualKeyCode::Space))
                }

                if let WindowEvent {window_id: _, event} = e {
                    match event {
                        Closed => running = false,
                        KeyboardInput { device_id: _, input } if is_break(input) 
                            => running = false,
                        Resized(width, height) => {
                            window.update_views(gfx_window_glutin::update_views);
                            //println!("resized: ({}, {})", width, height);
                            window.resize(width, height);
                        },
                        _ => (),
                    }
                }
            });

            window.draw();
            ::std::thread::sleep(::std::time::Duration::from_millis(10));
        }
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
