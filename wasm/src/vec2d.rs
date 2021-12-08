use std::f64;

#[derive(Copy, Clone, Debug)]
pub struct Vec2d(pub f64, pub f64);

impl Vec2d {
    #[inline]
    pub fn update(&mut self, x: f64, y: f64) {
        self.0 = x;
        self.1 = y;
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }

    #[inline]
    pub fn as_string(&self) -> String {
        format!("Vec2d({}, {})", self.0, self.1)
    }
}
