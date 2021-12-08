use std::collections::HashSet;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, std::hash::Hash)]
pub enum Control {
    Left,
    Right,
    Top,
    Bottom,
}

pub struct Input {
    buffer: HashSet<Control>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            buffer: HashSet::new(),
        }
    }

    pub fn add(&mut self, control: Control) {
        self.buffer.insert(control);
    }

    pub fn remove(&mut self, control: Control) {
        self.buffer.remove(&control);
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn is_pressed(&self, control: Control) {
        self.buffer.contains(&control);
    }
}
