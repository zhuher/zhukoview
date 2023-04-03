#![no_std]
#![allow(unused)]

pub mod bresenham;
pub mod helpers;
pub mod rainbow;
mod types;
#[inline(always)]
pub fn normalise_rect(
    view: &CanvasView,
    corner: (isize, isize),
    w: isize,
    h: isize,
) -> Option<crate::types::NormalRect> {
    if w == 0 || h == 0 {
        return None;
    }
    let mut nr: crate::types::NormalRect = (0, 0, 0, 0);
    (nr.0, nr.1) = match w.partial_cmp(&0).unwrap() {
        core::cmp::Ordering::Less => (corner.0 + w, corner.0),
        core::cmp::Ordering::Greater => (corner.0, corner.0 + w),
        _ => unreachable!(),
    };
    if nr.1 <= 0 || nr.0 >= view.w as isize - 1 {
        return None;
    }
    (nr.2, nr.3) = match h.partial_cmp(&0).unwrap() {
        core::cmp::Ordering::Less => (corner.1 + h, corner.1),
        core::cmp::Ordering::Greater => (corner.1, corner.1 + h),
        _ => unreachable!(),
    };
    if nr.3 <= 0 || nr.2 >= view.h as isize - 1 {
        return None;
    }
    nr = (
        core::cmp::max(nr.0, 0),
        core::cmp::min(nr.1, view.w as isize),
        core::cmp::max(nr.2, 0),
        core::cmp::min(nr.3, view.h as isize),
    );
    Some(nr)
}

#[inline(always)]
fn barycentric(
    a: (isize, isize),
    b: (isize, isize),
    c: (isize, isize),
    p: (isize, isize),
) -> Option<(isize, isize, isize)> {
    let (det, wgt1, wgt2) = (
        ((a.0 - c.0) * (b.1 - c.1) - (b.0 - c.0) * (a.1 - c.1)),
        ((b.1 - c.1) * (p.0 - c.0) + (c.0 - b.0) * (p.1 - c.1)),
        ((c.1 - a.1) * (p.0 - c.0) + (a.0 - c.0) * (p.1 - c.1)),
    );
    let wgt3 = det - wgt1 - wgt2;
    if (wgt1.signum() == det.signum() || wgt1.signum() == 0)
        && (wgt2.signum() == det.signum() || wgt2.signum() == 0)
        && (wgt3.signum() == det.signum() || wgt3.signum() == 0)
    {
        let total_weight = (wgt1.abs() + wgt2.abs() + wgt3.abs());
        let wgt1 = (wgt1.abs() * 255 / total_weight);
        let wgt2 = (wgt2.abs() * 255 / total_weight);
        let wgt3 = (wgt3.abs() * 255 / total_weight);
        Some((wgt1, wgt2, wgt3))
    } else {
        None
    }
}

pub enum AntiAliasing {
    None,
    Some(u32),
}

pub enum Colour {
    Single(u32),
    Triplet(u32, u32, u32),
}

#[derive(Debug)]
pub struct CanvasView {
    pub offset: (isize, isize),
    pub w: u32,
    pub h: u32,
    pub stride: u32,
}

impl CanvasView {
    #[inline(always)]
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            offset: (0, 0),
            w,
            h,
            stride: w,
        }
    }
    #[inline(always)]
    pub fn subview(&self, offset: (isize, isize), w: u32, h: u32) -> Option<Self> {
        normalise_rect(self, offset, w as isize, h as isize).map(|nr| Self {
            offset: (offset.0 + self.offset.0, offset.1 + self.offset.1),
            w: (nr.1 - nr.0) as u32,
            h: (nr.3 - nr.2) as u32,
            ..*self
        })
    }
    #[inline(always)]
    pub fn rgba32_to_argb32(&self, src: &[u32], dst: &mut [u32]) {
        for idx in 0..dst.len() {
            dst[idx] = ((src[idx] & 0xFF) << 24) | ((src[idx] & 0xFFFFFF00) >> 8);
        }
    }
    #[inline(always)]
    pub fn argb32_to_rgba32(&self, src: &[u32], dst: &mut [u32]) {
        for idx in 0..dst.len() {
            dst[idx] = ((src[idx] & 0xFF000000) >> 24) | ((src[idx] & 0x00FFFFFF) << 8);
        }
    }
    #[inline(always)]
    pub fn fill(&self, canvas: &mut [u32], colour: u32) {
        for y in self.offset.1..self.offset.1 + self.h as isize {
            for x in self.offset.0..self.offset.0 + self.w as isize {
                canvas_pixel!(canvas, self, (x, y)) = colour;
            }
        }
    }
    #[inline(always)]
    pub fn add_rect(
        &self,
        canvas: &mut [u32],
        corner: (isize, isize),
        w: isize,
        h: isize,
        colour: u32,
    ) {
        if let Some(nr) = normalise_rect(self, corner, w, h) {
            for y in nr.2 + self.offset.1..nr.3 + self.offset.1 {
                for x in nr.0 + self.offset.0..nr.1 + self.offset.0 {
                    crate::helpers::apply_argb(&mut canvas_pixel!(canvas, self, (x, y)), colour);
                }
            }
        }
    }
    ///Blends a circle onto a canvas within the view.
    ///Maximum antialiasing is 4.
    #[inline(always)]
    pub fn add_circle(
        &self,
        canvas: &mut [u32],
        cp: (isize, isize),
        r: usize,
        colour: u32,
        aa: AntiAliasing,
    ) {
        if let Some(nr) = normalise_rect(
            self,
            (cp.0 - r as isize, cp.1 - r as isize),
            (r << 1) as isize + 1,
            (r << 1) as isize + 1,
        ) {
            let (mut dx, mut dy) = (0, 0);
            match aa {
                AntiAliasing::None => {
                    for y in nr.2 + self.offset.1..nr.3 + self.offset.1 {
                        for x in nr.0 + self.offset.0..nr.1 + self.offset.0 {
                            (dx, dy) = (
                                x.saturating_sub(cp.0 + self.offset.0),
                                y.saturating_sub(cp.1 + self.offset.1),
                            );
                            if dx * dx + dy * dy <= r as isize * r as isize {
                                crate::helpers::apply_argb(
                                    &mut canvas_pixel!(canvas, self, (x, y)),
                                    colour,
                                );
                            }
                        }
                    }
                }
                AntiAliasing::Some(mut res) => {
                    res = 4.min(res);
                    let res1 = (res + 1) as isize;
                    let maxa = (colour & 0xFF000000) >> 24;
                    let mut ca: u32 = 0x0;
                    for y in nr.2 + self.offset.1..nr.3 + self.offset.1 {
                        for x in nr.0 + self.offset.0..nr.1 + self.offset.0 {
                            ca = 0;
                            for oy in 0..res as isize {
                                for ox in 0..res as isize {
                                    let (dx, dy) = (
                                        ((((x * res1) << 1) + 2 + (ox << 1)
                                            - res1 * ((cp.0 + self.offset.0) << 1))
                                            - (res1 << 1)),
                                        ((((y * res1) << 1) + 2 + (oy << 1)
                                            - res1 * ((cp.1 + self.offset.1) << 1))
                                            - (res1 << 1)),
                                    );
                                    if dx * dx + dy * dy <= (res1 * res1 * ((r * r) << 2) as isize)
                                    {
                                        ca += maxa;
                                    }
                                }
                            }
                            crate::helpers::apply_argb(
                                &mut canvas_pixel!(canvas, self, (x, y)),
                                (colour & 0x00FFFFFF) | ((ca / (res * res)) << 24),
                            );
                        }
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn add_line(
        &self,
        canvas: &mut [u32],
        p1: (isize, isize),
        p2: (isize, isize),
        colour: u32,
    ) {
        let bruh = crate::bresenham::Bresenham::new(p1, p2);
        if bruh.point.0 >= 0
            && bruh.point.0 < self.w as isize
            && bruh.point.1 >= 0
            && bruh.point.1 < self.h as isize
        {
            crate::helpers::apply_argb(
                &mut canvas_pixel!(
                    canvas,
                    self,
                    (bruh.point.0 + self.offset.0, bruh.point.1 + self.offset.1)
                ),
                colour,
            );
        }
        for point in bruh {
            if point.0 >= 0
                && point.0 < self.w as isize
                && point.1 >= 0
                && point.1 < self.h as isize
            {
                crate::helpers::apply_argb(
                    &mut canvas_pixel!(
                        canvas,
                        self,
                        (point.0 + self.offset.0, point.1 + self.offset.1)
                    ),
                    colour,
                );
            }
        }
    }
    #[inline(always)]
    pub fn add_ruler(&self, canvas: &mut [u32], intersec: (isize, isize)) {
        if intersec.0 >= 0
            && intersec.0 < self.w as isize - 1
            && intersec.1 >= 0
            && intersec.1 < self.h as isize - 1
        {
            let (mut vpu, mut hpr, mut vpd, mut hpl) = (intersec, intersec, intersec, intersec);

            crate::helpers::apply_argb(
                &mut canvas_pixel!(
                    canvas,
                    self,
                    (intersec.0 + self.offset.0, intersec.1 + self.offset.1)
                ),
                0x80FFFFFFu32,
            );
            loop {
                if vpu.1 < self.h as isize - 1 {
                    vpu.1 += 1;
                    crate::helpers::apply_argb(
                        &mut canvas_pixel!(
                            canvas,
                            self,
                            (vpu.0 + self.offset.0, vpu.1 + self.offset.1)
                        ),
                        if vpu.1 & 1 == intersec.1 & 1 {
                            0x80FFFFFFu32
                        } else {
                            0x80000000u32
                        },
                    );
                }
                if vpd.1 > 0 {
                    vpd.1 -= 1;
                    crate::helpers::apply_argb(
                        &mut canvas_pixel!(
                            canvas,
                            self,
                            (vpd.0 + self.offset.0, vpd.1 + self.offset.1)
                        ),
                        if vpd.1 & 1 == intersec.1 & 1 {
                            0x80FFFFFFu32
                        } else {
                            0x80000000u32
                        },
                    );
                }
                if hpr.0 < self.w as isize - 1 {
                    hpr.0 += 1;
                    crate::helpers::apply_argb(
                        &mut canvas_pixel!(
                            canvas,
                            self,
                            (hpr.0 + self.offset.0, hpr.1 + self.offset.1)
                        ),
                        if hpr.0 & 1 == intersec.0 & 1 {
                            0x80FFFFFFu32
                        } else {
                            0x80000000u32
                        },
                    );
                }
                if hpl.0 > 0 {
                    hpl.0 -= 1;
                    crate::helpers::apply_argb(
                        &mut canvas_pixel!(
                            canvas,
                            self,
                            (hpl.0 + self.offset.0, hpl.1 + self.offset.1)
                        ),
                        if hpl.0 & 1 == intersec.0 & 1 {
                            0x80FFFFFFu32
                        } else {
                            0x80000000u32
                        },
                    );
                }
                if hpl.0 == 0
                    && hpr.0 == self.w as isize - 1
                    && vpd.1 == 0
                    && vpu.1 == self.h as isize - 1
                {
                    break;
                }
            }
        }
    }
    #[inline(always)]
    pub fn add_triangle(
        &self,
        canvas: &mut [u32],
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize),
        colour: Colour,
    ) {
        let corner1: (isize, isize) = (a.0.min(b.0.min(c.0)), a.1.min(b.1.min(c.1)));
        let corner2: (isize, isize) = (a.0.max(b.0.max(c.0)), a.1.max(b.1.max(c.1)));
        let (w, h) = (corner2.0 - (corner1.0 - 1), corner2.1 - (corner1.1 - 1));
        if let Some(nr) = normalise_rect(self, corner1, w, h) {
            match colour {
                Colour::Single(hexcolour) => {
                    for y in nr.2 + self.offset.1..nr.3 + self.offset.1 {
                        for x in nr.0 + self.offset.0..nr.1 + self.offset.0 {
                            if barycentric(
                                (a.0 + self.offset.0, a.1 + self.offset.1),
                                (b.0 + self.offset.0, b.1 + self.offset.1),
                                (c.0 + self.offset.0, c.1 + self.offset.1),
                                (x, y),
                            )
                            .is_some()
                            {
                                crate::helpers::apply_argb(
                                    &mut canvas_pixel!(canvas, self, (x, y)),
                                    hexcolour,
                                );
                            }
                        }
                    }
                }
                Colour::Triplet(colour1, colour2, colour3) => {
                    for y in nr.2 + self.offset.1..nr.3 + self.offset.1 {
                        for x in nr.0 + self.offset.0..nr.1 + self.offset.0 {
                            if let Some((wgt1, wgt2, wgt3)) = barycentric(
                                (a.0 + self.offset.0, a.1 + self.offset.1),
                                (b.0 + self.offset.0, b.1 + self.offset.1),
                                (c.0 + self.offset.0, c.1 + self.offset.1),
                                (x, y),
                            ) {
                                crate::helpers::apply_argb(
                                    &mut canvas_pixel!(canvas, self, (x, y)),
                                    helpers::blend_colours(
                                        &colour1,
                                        &colour2,
                                        &colour3,
                                        &(wgt1 as u32),
                                        &(wgt2 as u32),
                                        &(wgt3 as u32),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    #[cfg(feature = "rng")]
    #[inline(always)]
    pub fn add_sprite(
        &self,
        canvas: &mut [u32],
        at: (isize, isize),
        spritew: u32,
        spriteh: u32,
        cmap: &[u32],
        vsym: bool,
        hsym: bool,
    ) {
        if normalise_rect(self, at, spritew as isize, spriteh as isize).is_some() {
            let (startx, endx) = (
                (at.0 + self.offset.0) as u32,
                (at.0 + self.offset.0) as u32 + spritew - 1,
            );
            let (starty, endy) = (
                (at.1 + self.offset.1) as u32,
                (at.1 + self.offset.1) as u32 + spriteh - 1,
            );
            let (mut lx, mut rx) = (startx, endx);
            let (mut ty, mut by) = (starty, endy);
            loop {
                let c = cmap[rng_range!(0..cmap.len())];
                crate::helpers::apply_argb(&mut canvas_pixel!(canvas, self, (lx, ty)), c);
                if vsym {
                    crate::helpers::apply_argb(&mut canvas_pixel!(canvas, self, (rx, ty)), c);
                }
                if hsym {
                    crate::helpers::apply_argb(&mut canvas_pixel!(canvas, self, (lx, by)), c);
                    if vsym {
                        crate::helpers::apply_argb(&mut canvas_pixel!(canvas, self, (rx, by)), c);
                    }
                }
                if (vsym && ((spritew & 1 == 1 && lx == rx) || (spritew & 1 == 0 && lx + 1 == rx)))
                    || (!vsym && lx == rx)
                {
                    if (hsym
                        && ((spriteh & 1 == 1 && ty == by) || (spriteh & 1 == 0 && ty + 1 == by)))
                        || (!hsym && ty == by)
                    {
                        break;
                    }
                    lx = startx;
                    ty += 1;
                    if vsym {
                        rx = endx;
                    }
                    if hsym {
                        by -= 1;
                    }
                } else {
                    lx += 1;
                    if vsym {
                        rx -= 1;
                    }
                }
            }
        }
    }
}
