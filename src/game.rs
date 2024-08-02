mod wall;

use crate::{
    system::{
        input::{InputStates, PressingInput},
        renderer::RenderRequest,
    },
    util::camera::CameraController,
};
use glam::Vec3;
use wall::Wall;

pub struct Game {
    camera_controller: CameraController,
    walls: [Wall; 4],
}

impl Game {
    pub fn new(width: f32, height: f32) -> Self {
        // create a camera controller
        let mut camera_controller = CameraController::default();
        camera_controller.width = width;
        camera_controller.height = height;
        camera_controller.position.y = 1.0;

        // create walls
        let walls = [
            Wall::new(0.0, 20.0, 0.0, 40.0),
            Wall::new(0.0, -20.0, 180.0f32.to_radians(), 40.0),
            Wall::new(20.0, 0.0, 90.0f32.to_radians(), 40.0),
            Wall::new(-20.0, 0.0, -90.0f32.to_radians(), 40.0),
        ];

        // finish
        info!("Game.new", "game created.");
        Self {
            camera_controller,
            walls,
        }
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
            // create a correct velocity
            let mut velocity = self
                .camera_controller
                .align_to_direction(Vec3::new(rl as f32, 0.0, fb as f32).normalize() * 0.2);

            // check wall collisions
            for n in self.walls.iter() {
                velocity = n.check_collision(self.camera_controller.position, velocity);
            }

            // move
            self.camera_controller.position += velocity;
        }

        render_requests.push(RenderRequest::UpdateCamera(self.camera_controller.clone()));
        render_requests.push(RenderRequest::DrawSkybox);
        let mut instance_controllers = Vec::new();
        for n in self.walls.iter() {
            instance_controllers.push(n.get_instance_controller());
        }
        render_requests.push(RenderRequest::DrawWorld(instance_controllers));
    }
}
