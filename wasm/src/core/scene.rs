use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use super::{
    game_objects::GameObject,
    gfx::{Gfx, Layer, RGBA},
};

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Rectangle {
    pub color: RGBA,
}

type Elements = (Vec<Rectangle>);

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct Scene {
    layers: HashMap<u32, Elements>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
        }
    }

    pub fn render(&mut self, game_objects: &[GameObject]) {
        for game_object in game_objects {
            let elements = self.layers.get_mut(&(game_object.layer as u32)).unwrap();
            match &game_object.gfx {
                Gfx::Rect(rect) => elements.push(Rectangle {
                    color: RGBA(0u8, 0u8, 0u8, 255u8),
                }),
                _ => {}
            }
        }
    }
}

#[wasm_bindgen]
impl Scene {
    pub fn get_layers(&self) -> JsValue {
        JsValue::from_serde(&self).unwrap()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self::new()
    }
}
