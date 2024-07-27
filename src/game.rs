use crate::{
    system::{
        input::{InputStates, PressingInput},
        renderer::RenderRequest,
    },
    util::camera::CameraController,
};

pub struct Game {
    camera_controller: CameraController,
}

impl Game {
    pub fn new(width: f32, height: f32) -> Self {
        // create a camera controller
        let mut camera_controller = CameraController::default();
        camera_controller.width = width;
        camera_controller.height = height;

        // finish
        info!("Game.new", "game created.");
        Self { camera_controller }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera_controller.width = width;
        self.camera_controller.height = height;
    }

    pub fn update(&mut self, input_states: &InputStates, render_requests: &mut Vec<RenderRequest>) {
        let f = input_states.get_pressing_input_state(&PressingInput::KeyW) as i32;
        let b = input_states.get_pressing_input_state(&PressingInput::KeyS) as i32;
        let r = input_states.get_pressing_input_state(&PressingInput::KeyD) as i32;
        let l = input_states.get_pressing_input_state(&PressingInput::KeyA) as i32;
        self.camera_controller.position.z += 0.01 * (f - b) as f32;
        self.camera_controller.position.x += 0.01 * (r - l) as f32;

        render_requests.push(RenderRequest::UpdateCamera(self.camera_controller.clone()));
    }
}
