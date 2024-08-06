use super::{game::GameSceneState, Scene};
use crate::{
    game::entity::message::Message,
    system::{
        input::{InputStates, PressingInput},
        renderer::{shader::ui::DrawUiDescriptor, RenderRequest},
    },
};
use glam::Vec4;

/// A states of title scene.
pub struct TitleSceneState {
    width: f32,
    height: f32,
    message: Message,
}

impl TitleSceneState {
    /// A constructor.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            message: Message::new(
                0.0,
                -height * 0.25,
                width * 0.3,
                Vec4::new(0.0, 0.125, 1.0, 0.125),
            ),
        }
    }

    /// A method to update the scene.
    pub fn update(
        &mut self,
        input_states: &InputStates,
        render_requests: &mut Vec<RenderRequest>,
    ) -> Option<Scene> {
        // update
        let next_scene = if input_states.pressing.get(&PressingInput::KeyE) == 1 {
            Some(Scene::GameScene(GameSceneState::new(
                self.width,
                self.height,
            )))
        } else {
            None
        };

        // draw
        let mut update_requests = Vec::new();
        update_requests.push(self.message.get_instance_controller());
        render_requests.push(RenderRequest::UpdateUiInstances(update_requests));
        render_requests.push(RenderRequest::DrawUi(DrawUiDescriptor {
            clear_color: Some([0.0, 0.0, 0.0]),
            instance_indices: Vec::from([(0, 1)]),
        }));

        // finish
        next_scene
    }
}
