pub mod game_objects;
pub mod geometry;
pub mod gfx;
pub mod input;
mod iter;
pub mod scene;
pub mod typography;
use game_objects::GameObject;
use geometry::Size;
use rapier2d::prelude::PhysicsPipeline;

use self::typography::Font;

pub struct FontLibrary {
    small: Font,
    medium: Font,
    large: Font,
}

pub struct Game<T> {
    pub state: T,
    pub input: input::Input,
    pub scene: scene::Scene,
    game_objects: Vec<GameObject>,
    physics_pipeline: PhysicsPipeline,
    size: Size,
    fonts: FontLibrary,
}

impl<T> Game<T> {
    pub fn new(size: Size, state: T) -> Self {
        let fonts = FontLibrary {
            small: Font::new(32.0),
            medium: Font::new(96.0),
            large: Font::new(144.0),
        };
        let high_score = 0;
        Game {
            state,
            size,
            fonts,
            game_objects: Vec::new(),
            input: input::Input::new(),
            scene: scene::Scene::new(),
            physics_pipeline: PhysicsPipeline::new(),
        }
    }

    pub fn update_fonts(&mut self, font_library: FontLibrary) -> &mut Self {
        self.fonts = font_library;
        self
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }

    pub fn render(&mut self) {
        self.scene.render(&self.game_objects);
    }
}
