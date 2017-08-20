#![cfg_attr(not(debug_assertions), windows_subsystem = "windows" )]

extern crate wcanvas;

use wcanvas::events;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {

    /*
    let (canvas, state) = register_handlers! {
        struct State {
            painting: bool = false,
            colors: [u32; 6] = [0xFF0000, 0xFFFF00, 0x00FF00, 
                                0x00FFFF, 0x0000FF, 0xFF00FF],
            idx: usize = 0,
        };

        on events::MouseMove => |state, ctx, e| {
            if state.painting {
                ctx.fill_rect(e.0 as u32 - 5, e.1 as u32 - 5, 10, 10);
            }
        };

        on events::MouseClick => |state, ctx, (s, button)| {
            if state == events::ElementState::Pressed {
                state.painting = true;
            } else {
                state.painting = false;
            }
        };

        on events::MouseScroll => |state, ctx, (s, button)| {
            match s {
                events::ScrollEvent::Up(n) => {
                    state.2 = (state.2 + n as usize) % 6;
                },
                events::ScrollEvent::Down(n) => {
                    state.2 = (state.2 + (6 - n as usize)) % 6;
                },
            }

            ctx.fill_color(state.1[state.2]);
            ctx.fill_rect(0, 0, 10, 10);
        };
   } */

    let mut ctx = wcanvas::Canvas::new();
    let state = Rc::new(RefCell::new((
        false,
        [0xFF0000, 0xFFFF00, 0x00FF00, 
         0x00FFFF, 0x0000FF, 0xFF00FF],
        0,
    )));
    
    let c = state.clone();
    ctx.on::< events::MouseMove>(Box::new(move |ctx, e| {
        if c.borrow().0 {
            ctx.fill_rect((e.0 as u32).saturating_sub(5), (e.1 as u32).saturating_sub(5), 10, 10);
        }
    }));

    let d = state.clone();
    ctx.on::< events::MouseClick>(Box::new(move |_, (state, button)| {
        if state == events::ElementState::Pressed {
            d.borrow_mut().0 = true;
        } else {
            d.borrow_mut().0 = false;
        }
    }));

    let d = state.clone();
    ctx.on::< events::MouseScroll>(Box::new(move |ctx, s| {
        let mut state = d.borrow_mut();

        match s {
            events::ScrollEvent::Up(n) => {
                state.2 = (state.2 + n as usize) % 6;
            },
            events::ScrollEvent::Down(n) => {
                state.2 = (state.2 + (6 - n as usize)) % 6;
            },
        }

        ctx.fill_color(state.1[state.2]);
        ctx.fill_rect(0, 0, 10, 10);
    }));

    ctx.fill_color(state.borrow().1[0]);
    //ctx.fill_color([0u8, 0xFF, 0x00, 0x80]);
    ctx.pause();

}