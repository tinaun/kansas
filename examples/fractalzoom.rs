#![cfg_attr(not(debug_assertions), windows_subsystem = "windows" )]

extern crate wcanvas;

use wcanvas::events;
use std::cell::RefCell;
use std::rc::Rc;


fn render_mandelbrot(ctx: &mut wcanvas::Context, scale: f64, center: [f64; 2], limit: u32) {
    let (w, h) = ctx.dimensions();
    ctx.fill_by(0, 0, w, h, |x, y| {
        let x = (x as f64 - w as f64 / 2.0 ) / scale + center[0];
        let y = (y as f64 - h as f64 / 2.0 ) / scale + center[1];

        let mut a = 0.0;
        let mut b = 0.0;

        let mut n = 0;
        while a*a + b*b < 4.0 && n < limit {
            let a2 = a*a - b*b + x;
            b = 2.0*a*b + y;
            a = a2;
            n += 1;
        }

        if n == limit {
            (0.0, 0.0, 0.0)
        } else if n <= 1 {
            (0.0, 0.0, 1.0)
        } else if n % 100 < 20 {
            let n = (n % 100) as f64 / 20.0;
            (n, 1.0 - n, n)
        } else if n % 100 < 60 {
            let n = ((n % 100) as f64 - 60.0) / 40.0;
            (1.0, n, 1.0)
        } else {
            let n = ((n % 100) as f64 - 60.0) / 40.0;
            (1.0, 1.0 - n.sqrt(), 1.0 - n.sqrt())
        }
    });
}

fn main() {

    let mut ctx = wcanvas::Canvas::new(600, 600);
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

        render_mandelbrot(ctx, state.0, state.1, 1000);
    }));


    render_mandelbrot(ctx.context_mut(), 200.0, [0.0, 0.0], 1000);
    ctx.pause();

}