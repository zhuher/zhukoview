use pingus::*;
use zhukoview::{AntiAliasing, CanvasView, Colour};

// 4096 x 2160 4K
// 3024 x 1964 MBP 14"
const W: u32 = 32;
const H: u32 = 32;
const SIZE: usize = (W * H) as usize;
fn main() {
    let mut canvas = [0xFFFFB86C; SIZE];
    let mut imgbuf = [0; SIZE];
    let mainview = CanvasView::new(W, H);
    let subview = mainview.subview((4, 4), 24, 24).unwrap();
    let (a, b, c) = (
        (0, 0),
        ((subview.w >> 1) as isize, (subview.h - 1) as isize),
        ((subview.w - 1) as isize, (subview.h >> 1) as isize),
    );
    subview.fill(&mut canvas, 0xFF282A36);
    subview.add_triangle(
        &mut canvas,
        a,
        b,
        c,
        Colour::Triplet(0xFFFF0000, 0xFF00FF00, 0xFF0000FF),
    );
    mainview.add_rect(&mut canvas, (1, 24), 7, 7, 0x800000FF);
    mainview.add_ruler(&mut canvas, (4, 4));
    mainview.add_rect(&mut canvas, (29, 29), -7, -7, 0xFFFF0000);
    mainview.add_circle(&mut canvas, (27, 3), 4, 0x80FF00FF, AntiAliasing::Some(4));
    mainview.add_triangle(
        &mut canvas,
        (31, 31),
        (20, 25),
        (25, 20),
        Colour::Single(0x8000FF00),
    );
    subview.argb32_to_rgba32(&canvas, &mut imgbuf);
    match create(W, H, &imgbuf, "pingus.png") {
        Ok(_) => println!("Ok!"),
        Err(e) => println!("Nah: {e}"),
    };
}
