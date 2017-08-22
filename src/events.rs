//! event listeners
use glutin;

pub use glutin::{ElementState, MouseButton, KeyboardInput, VirtualKeyCode};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EventType {
    MouseMove,
    MouseClick,
    MouseScroll,
    KeyPress,
    Resize,
}


pub type Callback<L> = Box<FnMut(&mut ::context::Context, <L as Listener>::Event)>;

pub(crate) struct ActiveListeners {
    pub mouse_move: Option<Callback<MouseMove>>,
    pub mouse_click: Option<Callback<MouseClick>>,
    pub mouse_scroll: Option<Callback<MouseScroll>>,
    pub key_press: Option<Callback<KeyPress>>,
    pub resize: Option<Callback<Resize>>,
}

impl ActiveListeners {
    pub fn new() -> Self {
        ActiveListeners {
            mouse_move: None,
            mouse_click: None,
            mouse_scroll: None,
            key_press: None,
            resize: None,
        }
    }
    pub fn add<E: Listener>(&mut self, f: Callback<E>) {
        match E::event_id() {
            EventType::MouseMove => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Callback<MouseMove>>(f)
                };

                self.mouse_move = Some(f);
            },
            EventType::MouseClick => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Callback<MouseClick>>(f)
                };

                self.mouse_click = Some(f);
            },
            EventType::MouseScroll => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Callback<MouseScroll>>(f)
                };

                self.mouse_scroll = Some(f);
            },
            EventType::KeyPress => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Callback<KeyPress>>(f)
                };

                self.key_press = Some(f);
            },
            EventType::Resize => {
                let f = unsafe { // evil unsafe downcasts evil
                    ::std::mem::transmute::<_, Callback<Resize>>(f)
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
            },
            EventType::MouseScroll => {
                self.mouse_scroll = None;
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

/// mouse scroll
/// emits 
pub enum MouseScroll {}

impl Listener for MouseScroll {
    type Event = ScrollEvent;
    
    fn event_id() -> EventType {
        EventType::MouseScroll
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ScrollEvent {
    Up(f32),
    Down(f32),
}

impl ScrollEvent {
    pub(crate) fn new(amount: f32) -> Self {
        if amount > 0.0 {
            ScrollEvent::Up(amount)
        } else {
            ScrollEvent::Down(-amount)
        }
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