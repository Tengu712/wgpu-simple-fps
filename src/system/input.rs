use std::collections::HashMap;
use winit::{
    event::{ElementState, KeyEvent, MouseButton},
    keyboard::{KeyCode, PhysicalKey},
};

/// An enum for enumerating all the inputs used in this game.
#[derive(PartialEq, Eq, Hash)]
pub enum Input {
    W,
    A,
    S,
    D,
    LeftButton,
}

/// An input manager.
///
/// It manages key and mouse input states as `bool`.
/// Call `get_states()` to get one of the state.
pub struct InputManager {
    states: HashMap<Input, bool>,
}

impl InputManager {
    pub fn new() -> Self {
        let mut states = HashMap::new();
        states.insert(Input::W, false);
        states.insert(Input::A, false);
        states.insert(Input::S, false);
        states.insert(Input::D, false);
        states.insert(Input::LeftButton, false);
        info!("InputManager.new", "input manager created.");
        Self { states }
    }

    pub fn update_key_state(&mut self, event: KeyEvent) {
        let input = match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => Input::W,
            PhysicalKey::Code(KeyCode::KeyA) => Input::A,
            PhysicalKey::Code(KeyCode::KeyS) => Input::S,
            PhysicalKey::Code(KeyCode::KeyD) => Input::D,
            _ => return,
        };
        self.states.insert(input, event.state.is_pressed());
    }

    pub fn update_mouse_state(&mut self, button: MouseButton, state: ElementState) {
        let input = match button {
            MouseButton::Left => Input::LeftButton,
            _ => return,
        };
        self.states.insert(input, state.is_pressed());
    }

    pub fn get_state(&self, input: &Input) -> bool {
        if let Some(n) = self.states.get(input) {
            n.clone()
        } else {
            false
        }
    }
}
