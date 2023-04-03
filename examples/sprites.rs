// #[cfg(all(feature = "rng", feature = "png"))]
fn main() {
    let paddingc = 0xFF808080_u32; // padding colour in ARGB hex
    let (spritew, spriteh) = (10_u32, 7_u32); // single sprite dimensions
    let (cols, rows) = (4_u32, 3_u32); // amount of columns and rows of sprites
    let padding = true; // to pad or not to pad that is the question...
    let vsym = true; // vertical sprite symmetry (qp)
    let hsym = false; // horisontal (qd)
    let paddingw = 1; // padding width
    let cmap = [
        0xFF000000, 0xFF1D2B53, 0xFF7E2553, 0xFF008751, 0xFFAB5236, 0xFF5F574F, 0xFFC2C3C7,
        0xFFFFF1E8, 0xFFFF004D, 0xFFFFA300, 0xFFFFEC27, 0xFF00E436, 0xFF29ADFF, 0xFF83769C,
        0xFFFF77A8, 0xFFFFCCAA,
    ]; // colour palette
    let totalw = spritew * cols + if padding { (cols + 1) * paddingw } else { 0 };
    let totalh = spriteh * rows + if padding { (rows + 1) * paddingw } else { 0 };
    let view: zhukoview::CanvasView = zhukoview::CanvasView::new(totalw, totalh);
    let mut canvas = vec![paddingc; (totalw * totalh) as usize];
    for row in 0..rows {
        for col in 0..cols {
            view.add_sprite(
                &mut canvas,
                (
                    (spritew * col + (u32::from(padding) + col * u32::from(padding)) * paddingw)
                        as isize,
                    (spriteh * row + (u32::from(padding) + row * u32::from(padding)) * paddingw)
                        as isize,
                ),
                spritew,
                spriteh,
                &cmap,
                vsym,
                hsym,
            );
        }
    }
    let mut imgbuf = vec![0; (totalw * totalh) as usize];
    view.argb32_to_rgba32(&canvas, &mut imgbuf);
    match pingus::create(totalw, totalh, &imgbuf, "spritemap.png") {
        Ok(_) => println!("Spritemap created successfully."),
        Err(e) => println!("Failed to create da spritemap: {e}"),
    };
}
