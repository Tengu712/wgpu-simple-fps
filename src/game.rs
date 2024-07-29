use glam::Vec3;

use crate::{
    system::{
        input::{InputStates, PressingInput},
        renderer::RenderRequest,
    },
    util::{camera::CameraController, instance::InstanceController},
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
        // rotate camera
        let moving_input_state = &input_states.moving_input_state;
        self.camera_controller.rotate(
            moving_input_state.x as f32 / self.camera_controller.width * 90.0f32.to_radians(),
            moving_input_state.y as f32 / self.camera_controller.height * 90.0f32.to_radians(),
        );

        // move camera
        let pressing_input_state = &input_states.pressing_input_states;
        let rl = pressing_input_state.get(&PressingInput::KeyD) as i32
            - pressing_input_state.get(&PressingInput::KeyA) as i32;
        let fb = pressing_input_state.get(&PressingInput::KeyW) as i32
            - pressing_input_state.get(&PressingInput::KeyS) as i32;
        if rl != 0 || fb != 0 {
            self.camera_controller
                .translate(Vec3::new(rl as f32, 0.0, fb as f32).normalize() * 0.1);
        }

        render_requests.push(RenderRequest::UpdateCamera(self.camera_controller.clone()));
        let mut instance_controllers = Vec::new();
        let mut instance = InstanceController::default();
        instance.position.x = -1.0;
        instance.position.z = 5.0;
        instance_controllers.push(instance);
        let mut instance = InstanceController::default();
        instance.position.x = 1.0;
        instance.position.z = 10.0;
        instance_controllers.push(instance);
        render_requests.push(RenderRequest::Draw(instance_controllers));
    }
}
