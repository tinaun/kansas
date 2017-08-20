// 2017 tinaun
//! a simple canvas-like drawing api for putting pixels on a screen
//! 
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

pub mod context;
pub mod color;
pub mod events;
mod pipeline;

use std::thread;
use std::sync::mpsc;
use std::ops::{Deref, DerefMut};

pub struct Canvas {
    ev_loop_handle: mpsc::Receiver<glutin::Event>,
    ctx: context::Context,
    listeners: events::ActiveListeners,
}

impl Canvas {

    /// create a new Canvas
    pub fn new() -> Self {
        let ev_loop = glutin::EventsLoop::new();
        let ctx = context::Context::new(800, 600, &ev_loop);

        let (tx, ev_loop_handle) = mpsc::channel();

        thread::spawn(move ||{
            use glutin::ControlFlow;
            let mut ev_loop = ev_loop;

            ev_loop.run_forever(|e|{
                match e {
                    //Closed => ControlFlow::Break,
                    _ => {
                        match tx.send(e) {
                            Ok(_) => ControlFlow::Continue,
                            Err(_) => ControlFlow::Break,
                        }
                    }
                }
            });
        });

        Canvas {
            ev_loop_handle,
            ctx,
            listeners: events::ActiveListeners::new(),
        }
    }


    /// register a function to be used as an event handler
    /// 
    /// note: as of right now, registering the same event twice will overwrite the previous handler
    /// function
    pub fn on<E: events::Listener>(&mut self, handler: events::Callback<E>) 
    {
        self.listeners.add::<E>(handler);
    }

    pub fn off<E: events::Listener>(&mut self) {
        self.listeners.remove(E::event_id());
    }

    /// hold execution until user hits `Esc`
    pub fn pause(&mut self) {
        let mut running = true;

        while running {
            while let Ok(e) = self.ev_loop_handle.try_recv() {
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
                            self.ctx.window.update_views(gfx_window_glutin::update_views);
                            //println!("resized: ({}, {})", width, height);
                            self.ctx.resize(width, height);

                            if let Some(cb) = self.listeners.resize.as_mut() {
                                cb(&mut self.ctx, (width, height));
                            }
                        },
                        MouseMoved { device_id: _, position } => {
                            if let Some(cb) = self.listeners.mouse_move.as_mut() {
                                cb(&mut self.ctx, position);
                            }
                        },
                        MouseInput { device_id: _, state, button } => {
                            if let Some(cb) = self.listeners.mouse_click.as_mut() {
                                cb(&mut self.ctx, (state, button));
                            }
                        },
                        MouseWheel { device_id: _, delta, phase: _ } => {
                            if let Some(cb) = self.listeners.mouse_scroll.as_mut() {
                                println!("delta: {:?}", delta);
                                if let glutin::MouseScrollDelta::LineDelta(_, y) = delta {
                                    cb(&mut self.ctx, events::ScrollEvent::new(y));
                                }
                            }
                        },
                        KeyboardInput { device_id: _, input } => {
                            if let Some(cb) = self.listeners.key_press.as_mut() {
                                cb(&mut self.ctx, input);
                            }
                        },
                        _ => (),
                    }
                }
            }

            self.ctx.window.draw();
            ::std::thread::sleep(::std::time::Duration::from_millis(10));
        }
    }
}

impl Deref for Canvas {
    type Target = context::Context;
    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl DerefMut for Canvas {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ctx
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
