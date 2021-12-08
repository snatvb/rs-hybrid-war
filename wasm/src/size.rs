use std::f64;

#[derive(Copy, Clone, Debug)]
pub struct Size(pub f64, pub f64);

impl Size {
    pub fn width(&self) -> f64 {
        self.0
    }

    pub fn height(&self) -> f64 {
        self.1
    }

    pub fn update(&mut self, w: f64, h: f64) {
        self.0 = w;
        self.1 = h;
    }
}
