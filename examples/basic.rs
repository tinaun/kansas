#![cfg_attr(not(debug_assertions), windows_subsystem = "windows" )]

extern crate wcanvas;

fn main() {
    let mut ctx = wcanvas::Canvas::new();
    let wheel: [u32; 6] = [0xFF0000, 0xFFFF00, 0x00FF00, 
                           0x00FFFF, 0x0000FF, 0xFF00FF];

    ctx.pause();

    for (i, color) in wheel.iter().enumerate() {
        let i = (i + 1) as u32;

        ctx.fill_color(*color);
        ctx.fill_rect(50 * i, 50 * i, 100, 100);
        ctx.pause();
    }

}