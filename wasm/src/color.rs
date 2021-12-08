pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn as_rgb_str(&self) -> String {
        format!("rgb({}, {}, {})", self.0, self.1, self.2)
    }
}
