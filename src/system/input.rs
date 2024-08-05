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
    KeyE,
    MouseLeft,
}

/// A struct for save the key or button pressing states.
pub struct PressingInputStates {
    pub states: HashMap<PressingInput, u32>,
}
impl Default for PressingInputStates {
    fn default() -> Self {
        let mut states = HashMap::new();
        states.insert(PressingInput::KeyW, 0);
        states.insert(PressingInput::KeyA, 0);
        states.insert(PressingInput::KeyS, 0);
        states.insert(PressingInput::KeyD, 0);
        states.insert(PressingInput::KeyE, 0);
        states.insert(PressingInput::MouseLeft, 0);
        Self { states }
    }
}
impl PressingInputStates {
    pub fn get(&self, pressing_input: &PressingInput) -> u32 {
        self.states.get(pressing_input).unwrap_or(&0).clone()
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
    pub pressing: PressingInputStates,
    pub moving: MovingInputState,
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
            PhysicalKey::Code(KeyCode::KeyE) => PressingInput::KeyE,
            _ => return,
        };
        let value = if !event.state.is_pressed() {
            0
        } else if let Some(n) = self.states.pressing.states.get(&input) {
            if *n > 0 {
                *n
            } else {
                1
            }
        } else {
            1
        };
        self.states.pressing.states.insert(input, value);
    }

    pub fn update_mouse_state(&mut self, button: MouseButton, state: ElementState) {
        let input = match button {
            MouseButton::Left => PressingInput::MouseLeft,
            _ => return,
        };
        let value = if !state.is_pressed() {
            0
        } else if let Some(n) = self.states.pressing.states.get(&input) {
            if *n > 0 {
                *n
            } else {
                1
            }
        } else {
            1
        };
        self.states.pressing.states.insert(input, value);
    }

    pub fn update_cursor_state(&mut self, position: PhysicalPosition<f64>) {
        self.states.moving.x += position.x - self.cursor_position.0;
        self.states.moving.y += position.y - self.cursor_position.1;
        self.cursor_position = (position.x, position.y);
    }

    pub fn set_cursor_position(&mut self, cursor_position: (f64, f64)) {
        self.cursor_position = cursor_position;
    }

    /// A method to clean moving input state and increment pressing input states.
    ///
    /// It's should be called the end of every frame.
    pub fn go_next(&mut self) {
        self.states.moving = MovingInputState::default();
        for (_, value) in &mut self.states.pressing.states {
            if *value > 0 {
                *value += 1;
            }
        }
    }
}
