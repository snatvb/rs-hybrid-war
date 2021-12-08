use crate::color::*;
use crate::size::*;
use crate::vec2d::*;

pub struct Rect {
    pub position: Vec2d,
    pub size: Size,
    pub color: Color,
}

impl Rect {
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}
