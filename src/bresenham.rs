#![no_std]
pub struct Bresenham {
    pub point: (isize, isize),
    dx: usize,
    dy: usize,
    sx: isize,
    sy: isize,
    err: isize,
    end: (isize, isize),
}
impl Bresenham {
    pub fn new(p1: (isize, isize), p2: (isize, isize)) -> Bresenham {
        Bresenham {
            point: p1,
            dx: (p2.0 - p1.0).unsigned_abs(),
            dy: (p2.1 - p1.1).unsigned_abs(),
            sx: match (p2.0 - p1.0).partial_cmp(&0).unwrap() {
                core::cmp::Ordering::Less => -1,
                core::cmp::Ordering::Greater => 1,
                core::cmp::Ordering::Equal => 0,
            },
            sy: match (p2.1 - p1.1).partial_cmp(&0).unwrap() {
                core::cmp::Ordering::Less => -1,
                core::cmp::Ordering::Greater => 1,
                core::cmp::Ordering::Equal => 0,
            },
            err: (p2.0 - p1.0).abs() - (p2.1 - p1.1).abs(),
            end: p2,
        }
    }
}
impl Iterator for Bresenham {
    type Item = (isize, isize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.point.0 == self.end.0 && self.point.1 == self.end.1 {
            return None;
        }
        let e2 = self.err << 1;
        if e2 > -(self.dy as isize) {
            self.err -= self.dy as isize;
            self.point.0 += self.sx;
        }
        if e2 < self.dx as isize {
            self.err += self.dx as isize;
            self.point.1 += self.sy;
        }
        Some(self.point)
    }
}
