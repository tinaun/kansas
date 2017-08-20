#![cfg_attr(not(debug_assertions), windows_subsystem = "windows" )]

extern crate wcanvas;

use wcanvas::events;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {

    let mut ctx = wcanvas::Canvas::new();
    let state = Rc::new(RefCell::new((
        200.0, // scale
        [0.0, 0.0], // offset
        [0.0, 0.0] // scaled mouse pos
    )));

    let d = state.clone();
    ctx.on::< events::MouseMove>(Box::new(move |ctx, offset|{
        let mut state = d.borrow_mut();
        let (w, h) = ctx.dimensions();

        state.2[0] = (offset.0 - w as f64 / 2.0) / state.0;
        state.2[1] = (offset.1 - h as f64 / 2.0) / state.0;

        println!("mouse: {:?}", state.2);

    }));

    let d = state.clone();
    ctx.on::< events::MouseScroll>(Box::new(move |ctx, s| {
        let mut state = d.borrow_mut();

        match s {
            events::ScrollEvent::Up(_) => {
                state.0 *= 0.5;
            },
            events::ScrollEvent::Down(_) => {
                state.0 *= 2.0;
                state.1[0] += state.2[0];
                state.1[1] += state.2[1];
            },
        }

        println!("{:?}", state);

        let (w, h) = ctx.dimensions();
        ctx.fill_by(0, 0, w, h, |x, y| {
            let x = (x as f64 - w as f64 / 2.0 ) / state.0 + state.1[0];
            let y = (y as f64 - h as f64 / 2.0 ) / state.0 + state.1[1];

            let mut a = 0.0;
            let mut b = 0.0;

            let mut n = 0;
            while a*a + b*b < 4.0 && n < 1000 {
                let a2 = a*a - b*b + x;
                b = 2.0*a*b + y;
                a = a2;
                n += 1;
            }

            if n == 1000 {
                (0.0, 0.0, 0.0)
            } else if n <= 1 {
                (0.0, 0.0, 1.0)
            } else if n < 10 {
                let n = n as f64 / 10.0;
                (n, 1.0 - n, n)
            } else if n < 100 {
                let n = n as f64 / 100.0;
                (1.0, n, 1.0)
            } else {
                let n = n as f64 / 1000.0;
                (1.0, 1.0 - n.sqrt(), 1.0 - n.sqrt())
            }
        });
    }));

    ctx.pause();

}