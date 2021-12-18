use bevy::{
    input::Input,
    prelude::{KeyCode, Res},
};

pub struct KeyDirection {
    pub x: i8,
    pub y: i8,
}

impl KeyDirection {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    pub fn has_direction(&self) -> bool {
        self.x != 0 || self.y != 0
    }
}

impl ToString for KeyDirection {
    fn to_string(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

pub fn pressed_pair(input: &Res<Input<KeyCode>>, (positive, negative): (KeyCode, KeyCode)) -> i8 {
    input.pressed(positive) as i8 - input.pressed(negative) as i8
}
