#[cfg(feature = "rng")]
fn main() {
    let paddingc = 0x808080FF;
    let (spritew, spriteh) = (10u32, 7u32);
    let (cols, rows) = (10, 10);
    let padding = true;
    let vsym = true;
    let hsym = false;
    let paddingw = 2;
    let cmap = [
        0x1D2B53FF, 0x7E2553FF, 0x008751FF, 0xAB5236FF, 0x5F574FFF, 0xC2C3C7FF, 0xFFF1E8FF,
        0xFF004DFF, 0xFFA300FF, 0xFFEC27FF, 0x00E436FF, 0x29ADFFFF, 0x83769CFF, 0xFF77A8FF,
        0xFFCCAAFF,
    ];
    let totalw = spritew * cols + if padding { (cols + 1) * paddingw } else { 0 };
    let totalh = spriteh * rows + if padding { (rows + 1) * paddingw } else { 0 };
    let mut c: zhucanvas::CanvasView =
        zhucanvas::CanvasView::new(paddingc, totalw, totalh, zhucanvas::RGBType::RGBA32);
    for row in 0..rows {
        for col in 0..cols {
            c.add_sprite(
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
    #[cfg(feature = "png")]
    match pingus::create(totalw, totalh, &c.data, "img/spritemap100.png") {
        Ok(_) => println!("Spritemap created successfully."),
        Err(e) => println!("Failed to create da spritemap: {e}"),
    };
}
