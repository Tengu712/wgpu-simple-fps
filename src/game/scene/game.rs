use super::Scene;
use crate::{
    game::entity::{floor::Floor, wall::Wall},
    system::{
        input::{InputStates, PressingInput},
        renderer::{model::ModelId, shader::world::DrawWorldDescriptor, RenderRequest},
    },
    util::{camera::CameraController, instance::InstanceController},
};
use glam::Vec3;

/// A states of game scene.
pub struct GameSceneState {
    camera_controller: CameraController,
    floor: Floor,
    walls: [Wall; 8],
    // TODO:
    flag: bool,
}

impl GameSceneState {
    /// A constructor.
    pub fn new(width: f32, height: f32) -> Self {
        // create a camera controller
        let mut camera_controller = CameraController::default();
        camera_controller.width = width;
        camera_controller.height = height;
        camera_controller.position.y = 1.5;
        camera_controller.position.z = -35.0;

        // create entities
        let floor = Floor::new(40.0, 80.0);
        let walls = [
            // outer
            Wall::new(
                Vec3::new(0.0, 4.0, 40.0),
                0.0f32.to_radians(),
                Vec3::new(40.0, 8.0, 1.0),
            ),
            Wall::new(
                Vec3::new(0.0, 4.0, -40.0),
                180.0f32.to_radians(),
                Vec3::new(40.0, 8.0, 1.0),
            ),
            Wall::new(
                Vec3::new(20.0, 4.0, 0.0),
                90.0f32.to_radians(),
                Vec3::new(80.0, 8.0, 1.0),
            ),
            Wall::new(
                Vec3::new(-20.0, 4.0, 0.0),
                -90.0f32.to_radians(),
                Vec3::new(80.0, 8.0, 1.0),
            ),
            // inner
            Wall::new(
                Vec3::new(-2.0, 1.5, -20.0),
                0.0f32.to_radians(),
                Vec3::new(36.0, 3.0, 1.0),
            ),
            Wall::new(
                Vec3::new(2.0, 1.5, 20.0),
                0.0f32.to_radians(),
                Vec3::new(36.0, 3.0, 1.0),
            ),
            Wall::new(
                Vec3::new(11.0, 3.5, 0.0),
                0.0f32.to_radians(),
                Vec3::new(18.0, 7.0, 1.0),
            ),
            Wall::new(
                Vec3::new(-11.0, 3.5, 0.0),
                0.0f32.to_radians(),
                Vec3::new(18.0, 7.0, 1.0),
            ),
        ];

        // finish
        Self {
            camera_controller,
            floor,
            walls,
            flag: false,
        }
    }

    /// A method to resize the camera width and height.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.camera_controller.width = width;
        self.camera_controller.height = height;
    }

    pub fn update(
        &mut self,
        input_states: &InputStates,
        render_requests: &mut Vec<RenderRequest>,
    ) -> Option<Scene> {
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
                .align_to_direction(Vec3::new(rl as f32, 0.0, fb as f32).normalize() * 0.25);

            // check wall collisions
            for n in self.walls.iter() {
                velocity = n.check_collision(self.camera_controller.position, velocity);
            }

            // move
            self.camera_controller.position += velocity;
        }

        // TODO:
        if !self.flag {
            self.flag = true;
            let mut instance_controllers = self
                .walls
                .iter()
                .map(|n| Some(n.get_instance_controller()))
                .collect::<Vec<Option<InstanceController>>>();
            instance_controllers.push(Some(self.floor.get_instance_controller()));
            render_requests.push(RenderRequest::UpdateWorldInstances(instance_controllers));
            let mut i = InstanceController::default();
            i.scale.x = 10.0;
            i.scale.y = 10.0;
            render_requests.push(RenderRequest::UpdateUiInstances(Vec::from([Some(i)])));
        }

        render_requests.push(RenderRequest::UpdateCamera(self.camera_controller.clone()));
        render_requests.push(RenderRequest::DrawSkybox);
        render_requests.push(RenderRequest::DrawWorld(DrawWorldDescriptor {
            instance_indices: Vec::from([(ModelId::Cube, 0, self.walls.len() as u32 + 1)]),
        }));

        None
    }
}
