use std::collections::HashMap;
use winit::{
    event::{ElementState, KeyEvent, MouseButton},
    keyboard::{KeyCode, PhysicalKey},
};

/// An enum for enumerating all the key and mouse pressing inputs used in this game.
#[derive(PartialEq, Eq, Hash)]
pub enum PressingInput {
    KeyW,
    KeyA,
    KeyS,
    KeyD,
    MouseLeft,
}

/// A struct that encapsulates the input states.
pub struct InputStates {
    pressing_input_states: HashMap<PressingInput, bool>,
}

impl InputStates {
    pub fn get_pressing_input_state(&self, pressing_input: &PressingInput) -> bool {
        self.pressing_input_states
            .get(pressing_input)
            .unwrap_or(&false)
            .clone()
    }
}

/// An input manager.
pub struct InputManager {
    states: InputStates,
}

impl InputManager {
    pub fn new() -> Self {
        let mut pressing_input_states = HashMap::new();
        pressing_input_states.insert(PressingInput::KeyW, false);
        pressing_input_states.insert(PressingInput::KeyA, false);
        pressing_input_states.insert(PressingInput::KeyS, false);
        pressing_input_states.insert(PressingInput::KeyD, false);
        pressing_input_states.insert(PressingInput::MouseLeft, false);
        info!("InputManager.new", "input manager created.");
        Self {
            states: InputStates {
                pressing_input_states,
            },
        }
    }

    pub fn get(&self) -> &InputStates {
        &self.states
    }

    pub fn update_key_state(&mut self, event: KeyEvent) {
        let input = match event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => PressingInput::KeyW,
            PhysicalKey::Code(KeyCode::KeyA) => PressingInput::KeyA,
            PhysicalKey::Code(KeyCode::KeyS) => PressingInput::KeyS,
            PhysicalKey::Code(KeyCode::KeyD) => PressingInput::KeyD,
            _ => return,
        };
        self.states
            .pressing_input_states
            .insert(input, event.state.is_pressed());
    }

    pub fn update_mouse_state(&mut self, button: MouseButton, state: ElementState) {
        let input = match button {
            MouseButton::Left => PressingInput::MouseLeft,
            _ => return,
        };
        self.states
            .pressing_input_states
            .insert(input, state.is_pressed());
    }
}
