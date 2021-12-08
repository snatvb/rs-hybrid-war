#![allow(unused)]

mod color;
#[macro_use]
mod logger;
mod core;
mod rect;
mod size;
mod vec2d;
use logger::*;
use rect::*;
use std::cell::RefCell;
use std::rc::Rc;
use vec2d::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
enum State {
    MainMenu,
}

#[wasm_bindgen]
pub struct Game(core::Game<State>);

#[wasm_bindgen]
impl Game {
    pub fn new() -> Self {
        Game(core::Game::new(
            core::geometry::Size::new(800.0, 600.0),
            State::MainMenu,
        ))
    }

    pub fn tick(&mut self) {}

    pub fn handle_key_down(&mut self, control: core::input::Control) {
        self.0.input.add(control);
    }

    pub fn handle_key_up(&mut self, control: core::input::Control) {
        self.0.input.remove(control);
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
