use std::collections::HashMap;
use winit::{
    dpi::PhysicalPosition,
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

/// A struct for save the moving amount of a cursor.
#[derive(Clone)]
pub struct MovingInputState {
    pub x: f64,
    pub y: f64,
}
impl Default for MovingInputState {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

/// An object for save input states.
pub struct InputStates {
    pressing_input_states: HashMap<PressingInput, bool>,
    moving_input_state: MovingInputState,
}
impl InputStates {
    pub fn get_pressing_input_state(&self, pressing_input: &PressingInput) -> bool {
        self.pressing_input_states
            .get(pressing_input)
            .unwrap_or(&false)
            .clone()
    }
    pub fn get_moving_input_state(&self) -> MovingInputState {
        self.moving_input_state.clone()
    }
}

/// An input manager.
pub struct InputManager {
    states: InputStates,
    cursor_position: (f64, f64),
}
impl InputManager {
    pub fn new(cursor_position: (f64, f64)) -> Self {
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
                moving_input_state: MovingInputState::default(),
            },
            cursor_position,
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

    pub fn update_cursor_state(&mut self, position: PhysicalPosition<f64>) {
        self.states.moving_input_state.x += position.x - self.cursor_position.0;
        self.states.moving_input_state.y += position.y - self.cursor_position.1;
        self.cursor_position = (position.x, position.y);
    }

    pub fn set_cursor_position(&mut self, cursor_position: (f64, f64)) {
        self.cursor_position = cursor_position;
    }

    /// A method to clean moving input state.
    ///
    /// It's should be called the end of every frame.
    pub fn clean(&mut self) {
        self.states.moving_input_state = MovingInputState::default();
    }
}
