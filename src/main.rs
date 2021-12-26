use bevy::prelude::*;

mod core;

fn main() {
    App::new()
        .add_plugin(core::Core)
        .run();
}