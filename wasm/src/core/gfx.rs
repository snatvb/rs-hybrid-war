use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use super::geometry;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

pub struct Rect {
    position: geometry::Point,
}

pub struct Polygon {}

#[wasm_bindgen]
#[repr(i32)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Layer {
    Background,
    Decoration,
    Foreground,
    Effect,
    UI,
}

pub enum Gfx {
    None,
    Rect(Rect),
    Polygon(Polygon),
}
