use super::{title::TitleSceneState, Scene};
use crate::{
    game::entity::{
        digits::Digits, floor::Floor, message::Message, reticle::Reticle, target::Target,
        wall::Wall,
    },
    system::{
        input::{InputStates, PressingInput},
        renderer::{
            model::ModelId,
            shader::{ui::DrawUiDescriptor, world::DrawWorldDescriptor},
            RenderRequest,
        },
    },
    util::camera::CameraController,
};
use glam::{Vec3, Vec4};

enum State {
    Game,
    End,
}

/// A states of game scene.
pub struct GameSceneState {
    state: State,
    width: f32,
    height: f32,
    camera_controller: CameraController,
    floor: Floor,
    walls: [Wall; 8],
    targets: Vec<Target>,
    reticle: Reticle,
    message: Option<Message>,
    indication: Option<Message>,
    score_ui: Digits,
    score: u32,
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

        // create targets
        let targets = Vec::from([
            Target::new(Vec3::new(15.0, 2.5, -15.0), Box::new(|_, _| ())),
            Target::new(
                Vec3::new(0.0, 2.0, 0.0),
                Box::new(|n: &mut Vec3, i: u32| n.y = (i as f32).to_radians().sin() * 2.0 + 3.0),
            ),
            Target::new(Vec3::new(15.0, 3.0, 15.0), Box::new(|_, _| ())),
            Target::new(
                Vec3::new(0.0, 2.0, 35.0),
                Box::new(|n: &mut Vec3, i: u32| {
                    *n = Vec3::new(
                        ((i / 40) as f32 * 51.0).to_radians().sin(),
                        ((i / 40) as f32 * 79.0).to_radians().sin() + 2.5,
                        35.0,
                    )
                }),
            ),
            Target::new(
                Vec3::new(25.0, 30.0, -20.0),
                Box::new(|n: &mut Vec3, i: u32| n.z = (i as f32).to_radians().sin() * 2.0 + -20.0),
            ),
        ]);

        // create uis
        let reticle = Reticle::new();
        let score_ui = Digits::new(width / 2.0, height / 2.0, 100.0, 1200);

        // finish
        Self {
            state: State::Game,
            width,
            height,
            camera_controller,
            floor,
            walls,
            targets,
            reticle,
            message: None,
            indication: None,
            score_ui,
            score: 1380,
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
        let moving = &input_states.moving;
        self.camera_controller.rotate(
            moving.x as f32 / self.camera_controller.width * 90.0f32.to_radians(),
            moving.y as f32 / self.camera_controller.height * 90.0f32.to_radians(),
        );

        // move camera
        let pressing = &input_states.pressing;
        let r = pressing.get(&PressingInput::KeyD) > 0;
        let l = pressing.get(&PressingInput::KeyA) > 0;
        let f = pressing.get(&PressingInput::KeyW) > 0;
        let b = pressing.get(&PressingInput::KeyS) > 0;
        let rl = r as i32 - l as i32;
        let fb = f as i32 - b as i32;
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

        // do depends on state
        let next_scene = match self.state {
            State::Game => self.update_game(input_states),
            State::End => self.update_end(input_states),
        };

        // update targets
        // NOTE: Update targets after player's shooting is processed.
        for n in &mut self.targets {
            n.update();
        }

        // collect update requests
        let mut update_world_requests = Vec::new();
        for n in &mut self.walls {
            update_world_requests.push(n.get_instance_controller());
        }
        update_world_requests.push(self.floor.get_instance_controller());
        for n in &mut self.targets {
            update_world_requests.push(Some(n.get_instance_controller()));
        }
        let mut update_ui_requests = Vec::new();
        update_ui_requests.push(self.reticle.get_instance_controller());
        if let Some(n) = &mut self.message {
            update_ui_requests.push(n.get_instance_controller());
        }
        if let Some(n) = &mut self.indication {
            update_ui_requests.push(n.get_instance_controller());
        }
        for n in self.score_ui.get_instance_controllers() {
            update_ui_requests.push(n);
        }

        // define entities count on the world
        let static_entities_count = self.walls.len() as u32 + 1;
        let all_entities_count = static_entities_count + self.targets.len() as u32;

        // get uis count
        let ui_entities_count = update_ui_requests.len();

        // draw
        render_requests.push(RenderRequest::UpdateCamera(self.camera_controller.clone()));
        render_requests.push(RenderRequest::DrawSkybox);
        render_requests.push(RenderRequest::UpdateWorldInstances(update_world_requests));
        render_requests.push(RenderRequest::DrawWorld(DrawWorldDescriptor {
            instance_indices: Vec::from([
                (ModelId::Cube, 0, static_entities_count),
                (ModelId::Sphere, static_entities_count, all_entities_count),
            ]),
        }));
        render_requests.push(RenderRequest::UpdateUiInstances(update_ui_requests));
        render_requests.push(RenderRequest::DrawUi(DrawUiDescriptor {
            clear_color: None,
            instance_indices: Vec::from([(0, ui_entities_count as u32)]),
        }));

        next_scene
    }

    fn update_game(&mut self, input_states: &InputStates) -> Option<Scene> {
        // decrement score
        self.score = if self.score == 0 { 0 } else { self.score - 1 };
        self.score_ui.set_number(self.score);

        // shoot
        if input_states.pressing.get(&PressingInput::MouseLeft) == 1 {
            let direction = self
                .camera_controller
                .rotation
                .mul_vec3(Vec3::new(0.0, 0.0, 1.0));
            self.targets
                .retain(|n| !n.check_shot(self.camera_controller.position, direction));
        }

        // check game clear or over
        if self.targets.is_empty() || self.score == 0 {
            let uv = if self.targets.is_empty() {
                Vec4::new(0.0, 0.375, 0.8, 0.125)
            } else {
                Vec4::new(0.0, 0.5, 0.7, 0.125)
            };
            let y = self.height * 0.25;
            let message = Message::new(0.0, y, self.width * 0.3, uv);
            let mut indication = Message::new(
                0.0,
                0.0,
                self.width * 0.35,
                Vec4::new(0.0, 0.25, 1.0, 0.125),
            );
            indication.set_position(
                0.0,
                y - message.get_height() / 2.0 - indication.get_height() / 2.0,
            );
            self.message = Some(message);
            self.indication = Some(indication);
            self.state = State::End;
        }

        None
    }

    fn update_end(&mut self, input_states: &InputStates) -> Option<Scene> {
        if input_states.pressing.get(&PressingInput::KeyE) == 1 {
            Some(Scene::TitleScene(TitleSceneState::new(
                self.width,
                self.height,
            )))
        } else {
            None
        }
    }
}
