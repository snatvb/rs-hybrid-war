use std::collections::HashMap;

use bevy::prelude::{Handle, Texture};

pub struct Materials {
    pub player: Handle<Texture>,
    pub environment: HashMap<&'static str, Handle<Texture>>,
}
