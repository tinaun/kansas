//! event listeners
use glutin;
use std::any::Any;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EventType {
    MouseMove,
    MouseClick,
    KeyPress,
    Resize,
}

pub(crate) struct ActiveListeners {
    pub mouse_move: Option<Box<FnMut(<MouseMove as Listener>::Event)>>,
    pub mouse_click: Option<Box<FnMut(<MouseClick as Listener>::Event)>>,
    pub key_press: Option<Box<FnMut(<KeyPress as Listener>::Event)>>,
    pub resize: Option<Box<FnMut(<Resize as Listener>::Event)>>,
}

impl ActiveListeners {
    pub fn new() -> Self {
        ActiveListeners {
            mouse_move: None,
            mouse_click: None,
            key_press: None,
            resize: None,
        }
    }
    pub fn add<E: Listener>(&mut self, f: Box<FnMut(E::Event)>) {
        match E::event_id() {
            EventType::MouseMove => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Box<FnMut(<MouseMove as Listener>::Event)>>(f)
                };

                self.mouse_move = Some(f);
            },
            EventType::MouseClick => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Box<FnMut(<MouseClick as Listener>::Event)>>(f)
                };

                self.mouse_click = Some(f);
            },
            EventType::KeyPress => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Box<FnMut(<KeyPress as Listener>::Event)>>(f)
                };

                self.key_press = Some(f);
            },
            EventType::Resize => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Box<FnMut(<Resize as Listener>::Event)>>(f)
                };

                self.resize = Some(f);
            },

            _ => unreachable!(),
        }
    }

    pub fn remove(&mut self, e: EventType) {
        match e {
            EventType::MouseMove => {
                self.mouse_move = None;
            },
            EventType::MouseClick => {
                self.mouse_click = None;
            },
            EventType::KeyPress => {
                self.key_press = None;
            },
            EventType::Resize => {
                self.resize = None;
            }
        }
    }

}


pub trait Listener {
    type Event;
    fn event_id() -> EventType;
}

/// mouse motion
/// emits `(f64, f64)` deltas
pub enum MouseMove {}

impl Listener for MouseMove {
    type Event = (f64, f64);
    fn event_id() -> EventType {
        EventType::MouseMove
    }
}

/// mouse click
/// emits glutin mousebutton events
pub enum MouseClick {}

impl Listener for MouseClick {
    type Event = (glutin::ElementState, glutin::MouseButton);
    
    fn event_id() -> EventType {
        EventType::MouseClick
    }
}

/// key press
/// emits glutin keyboardinput events
pub enum KeyPress {}

impl Listener for KeyPress {
    type Event = glutin::KeyboardInput;
    
    fn event_id() -> EventType {
        EventType::KeyPress
    }
}

/// window resize
/// emits `(u32, u32)`, the new dimensions of the window
pub enum Resize {}

impl Listener for Resize {
    type Event = (u32, u32);
    
    fn event_id() -> EventType {
        EventType::Resize
    }
}