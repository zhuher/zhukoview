#![no_std]
#[cfg(feature = "rng")]
#[macro_export]
macro_rules! rng_range {
    ($range:expr) => {
        rand::Rng::gen_range(&mut rand::thread_rng(), $range)
    };
}
/*
#[macro_export]
macro_rules! cmp {
    ($lhs:expr, $rhs:expr) => {
        if $lhs == $rhs {
            0i8
        } else {
            if $lhs > $rhs {
                1i8
            } else {
                -1i8
            }
        }
    };
}
*/
/*
#[macro_export]
macro_rules! min {
    ($lhs:expr, $rhs:expr) => {
        if $lhs > $rhs {
            $rhs
        } else {
            $lhs
        }
    };
}
*/
/*
#[macro_export]
macro_rules! max {
    ($lhs:expr, $rhs:expr) => {
        if $lhs < $rhs {
            $rhs
        } else {
            $lhs
        }
    };
}
*/
#[macro_export]
macro_rules! canvas_pixel {
    ($canvas:expr, $view:expr, $point:expr) => {
        $canvas[$point.0 as usize + $point.1 as usize * $view.stride as usize]
    };
} /*
          | (((fga * fga) + ((bg & 0xFF) * !(fga as u8) as u32)) / 255);
  C result=[0.01.00.00.6]*0.6+[1.00.00.01.0]*(1−0.6)
  */
/*
#[macro_export]
macro_rules! blend_colours {
    ($colour1:expr, $colour2:expr, $colour3:expr, $bias1:expr, $bias2:expr, $bias3:expr) => {
        (core::cmp::min(
            ((($colour1 >> 24) & 0xff) * $bias1
                + (($colour2 >> 24) & 0xff) * $bias2
                + (($colour3 >> 24) & 0xff) * $bias3)
                / 255,
            0xFF,
        ) << 24)
            | (core::cmp::min(
                ((($colour1 >> 16) & 0xff) * $bias1
                    + (($colour2 >> 16) & 0xff) * $bias2
                    + (($colour3 >> 16) & 0xff) * $bias3)
                    / 255,
                0xFF,
            ) << 16)
            | (0xFF.min(
                ((($colour1 >> 8) & 0xff) * $bias1
                    + (($colour2 >> 8) & 0xff) * $bias2
                    + (($colour3 >> 8) & 0xff) * $bias3)
                    / 255,
            ) << 8)
            | core::cmp::min(
                (($colour1 & 0xff) * $bias1
                    + ($colour2 & 0xff) * $bias2
                    + ($colour3 & 0xff) * $bias3)
                    / 255,
                0xFF,
            )
    };
}
*/
#[inline(always)]
pub fn blend_colours(
    colour1: &u32,
    colour2: &u32,
    colour3: &u32,
    bias1: &u32,
    bias2: &u32,
    bias3: &u32,
) -> u32 {
    (0xFF.min(
        (((colour1 >> 24) & 0xff) * bias1
            + ((colour2 >> 24) & 0xff) * bias2
            + ((colour3 >> 24) & 0xff) * bias3)
            / 255,
    ) << 24)
        | (0xFF.min(
            (((colour1 >> 16) & 0xff) * bias1
                + ((colour2 >> 16) & 0xff) * bias2
                + ((colour3 >> 16) & 0xff) * bias3)
                / 255,
        ) << 16)
        | (0xFF.min(
            (((colour1 >> 8) & 0xff) * bias1
                + ((colour2 >> 8) & 0xff) * bias2
                + ((colour3 >> 8) & 0xff) * bias3)
                / 255,
        ) << 8)
        | 0xFF.min(
            ((colour1 & 0xff) * bias1 + (colour2 & 0xff) * bias2 + (colour3 & 0xff) * bias3) / 255,
        )
}
#[inline(always)]
pub fn rotate(point: &(isize, isize), pivot: &(isize, isize), deg: &f64) -> (isize, isize) {
    (
        (libm::cos(deg * core::f64::consts::PI / 180f64) * (point.0 - pivot.0) as f64
            - libm::sin(deg * core::f64::consts::PI / 180f64) * (point.1 - pivot.1) as f64
            + pivot.0 as f64) as isize,
        (libm::sin(deg * core::f64::consts::PI / 180f64) * (point.0 - pivot.0) as f64
            + libm::cos(deg * core::f64::consts::PI / 180f64) * (point.1 - pivot.1) as f64
            + pivot.1 as f64) as isize,
    )
}
/*
#[inline(always)]
pub fn apply_rgba(mut point: &mut u32, colour: u32) {
    let fga = (colour & 0xFF);
    *point = ((((((colour >> 24) & 0xFF) * fga)
        + (((*point >> 24) & 0xFF) * !(fga as u8) as u32))
        / 255)
        << 24
        | (((((colour >> 16) & 0xFF) * fga) + (((*point >> 16) & 0xFF) * !(fga as u8) as u32))
            / 255)
            << 16
        | (((((colour >> 8) & 0xFF) * fga) + (((*point >> 8) & 0xFF) * !(fga as u8) as u32))
            / 255)
            << 8
        | std::cmp::min(255, fga + (*point & 0xFF)));
}
*/
/*
#[inline(always)]
pub fn apply_rgba(point: &mut u32, colour: u32) {
    *point = (0xFF.min(
        (((colour >> 24) & 0xFFu32) * (colour & 0xFF)
            + ((*point >> 24) & 0xFFu32) * !((colour & 0xFF) as u8) as u32)
            / 255u32,
    ) << 24)
        | (0xFF.min(
            (((colour >> 16) & 0xFFu32) * (colour & 0xFF)
                + ((*point >> 16) & 0xFFu32) * !((colour & 0xFF) as u8) as u32)
                / 255u32,
        ) << 16)
        | (0xFF.min(
            (((colour >> 8) & 0xFFu32) * (colour & 0xFF)
                + ((*point >> 8) & 0xFFu32) * !((colour & 0xFF) as u8) as u32)
                / 255u32,
        ) << 8)
        | 0xFF.min((colour & 0xFF) + (*point & 0xFF));
}
*/
#[inline(always)]
pub fn apply_argb(point: &mut u32, colour: u32) {
    *point = (0xFF.min(
        (((colour >> 16) & 0xFFu32) * ((colour >> 24) & 0xFF)
            + ((*point >> 16) & 0xFFu32) * !(((colour >> 24) & 0xFF) as u8) as u32)
            / 255u32,
    ) << 16)
        | (0xFF.min(
            (((colour >> 8) & 0xFFu32) * ((colour >> 24) & 0xFF)
                + ((*point >> 8) & 0xFFu32) * !(((colour >> 24) & 0xFF) as u8) as u32)
                / 255u32,
        ) << 8)
        | 0xFF.min(
            ((colour & 0xFFu32) * ((colour >> 24) & 0xFF)
                + (*point & 0xFFu32) * !(((colour >> 24) & 0xFF) as u8) as u32)
                / 255u32,
        )
        | (0xFF.min(((colour >> 24) & 0xFF) + ((*point >> 24) & 0xFF)) << 24)
}

/*
#[macro_export]
macro_rules! apply_rgba {
    ($point:expr, $colour:expr) => {
        $point = (0xFF.min(
            ((($colour >> 24) & 0xFFu32) * ($colour & 0xFF)
                + (($point >> 24) & 0xFFu32) * !(($colour & 0xFF) as u8) as u32)
                / 255u32,
        ) << 24)
            | (0xFF.min(
                ((($colour >> 16) & 0xFFu32) * ($colour & 0xFF)
                    + (($point >> 16) & 0xFFu32) * !(($colour & 0xFF) as u8) as u32)
                    / 255u32,
            ) << 16)
            | (0xFF.min(
                ((($colour >> 8) & 0xFFu32) * ($colour & 0xFF)
                    + (($point >> 8) & 0xFFu32) * !(($colour & 0xFF) as u8) as u32)
                    / 255u32,
            ) << 8)
            | 0xFF.min(($colour & 0xFF) + ($point & 0xFF));
    };
}
*/
