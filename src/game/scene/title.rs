use super::{game::GameSceneState, Scene};
use crate::system::{
    input::{InputStates, PressingInput},
    renderer::RenderRequest,
};

/// A states of title scene.
pub struct TitleSceneState {
    width: f32,
    height: f32,
}

impl TitleSceneState {
    /// A constructor.
    ///
    /// It needs screen width and screen height for create `GameSceneState`.
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// A method to update the scene.
    ///
    /// If a player presses a mouse left button, it moves on to a game scene.
    pub fn update(
        &self,
        input_states: &InputStates,
        render_requests: &mut Vec<RenderRequest>,
    ) -> Option<Scene> {
        let next_scene = if input_states
            .pressing_input_states
            .get(&PressingInput::MouseLeft)
        {
            Some(Scene::GameScene(GameSceneState::new(
                self.width,
                self.height,
            )))
        } else {
            None
        };

        next_scene
    }
}
