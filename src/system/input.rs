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

/// A struct for save the key or button pressing states.
pub struct PressingInputStates {
    pub states: HashMap<PressingInput, bool>,
}
impl Default for PressingInputStates {
    fn default() -> Self {
        let mut states = HashMap::new();
        states.insert(PressingInput::KeyW, false);
        states.insert(PressingInput::KeyA, false);
        states.insert(PressingInput::KeyS, false);
        states.insert(PressingInput::KeyD, false);
        states.insert(PressingInput::MouseLeft, false);
        Self { states }
    }
}
impl PressingInputStates {
    pub fn get(&self, pressing_input: &PressingInput) -> bool {
        self.states.get(pressing_input).unwrap_or(&false).clone()
    }
}

/// A struct for save the moving amount of a cursor.
#[derive(Default)]
pub struct MovingInputState {
    pub x: f64,
    pub y: f64,
}

/// A struct for consolidating various input states.
#[derive(Default)]
pub struct InputStates {
    pub pressing_input_states: PressingInputStates,
    pub moving_input_state: MovingInputState,
}

/// An input manager.
pub struct InputManager {
    states: InputStates,
    cursor_position: (f64, f64),
}
impl InputManager {
    pub fn new(cursor_position: (f64, f64)) -> Self {
        Self {
            states: InputStates::default(),
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
            .states
            .insert(input, event.state.is_pressed());
    }

    pub fn update_mouse_state(&mut self, button: MouseButton, state: ElementState) {
        let input = match button {
            MouseButton::Left => PressingInput::MouseLeft,
            _ => return,
        };
        self.states
            .pressing_input_states
            .states
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
