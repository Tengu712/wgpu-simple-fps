mod game;
mod title;

use crate::system::{input::InputStates, renderer::RenderRequest};
use game::GameSceneState;
use title::TitleSceneState;

enum Scene {
    /// A dummy scene for waiting for a window to be created.
    ///
    /// NOTE: Since winit 0.30 does not allow retrieving the window size at the entry point,
    ///       I need to create this scene.
    PrepareScene,
    TitleScene(TitleSceneState),
    GameScene(GameSceneState),
}

/// A scene manager.
pub struct SceneManager {
    scene: Scene,
}

impl SceneManager {
    /// A constructor.
    ///
    /// The first scene is prepare scene.
    pub fn new() -> Self {
        Self {
            scene: Scene::PrepareScene,
        }
    }

    /// A method to update the scene.
    pub fn update(&mut self, input_states: &InputStates, render_requests: &mut Vec<RenderRequest>) {
        let new_scene = match &mut self.scene {
            Scene::PrepareScene => return,
            Scene::TitleScene(n) => n.update(input_states, render_requests),
            Scene::GameScene(n) => n.update(input_states, render_requests),
        };
        if let Some(n) = new_scene {
            self.scene = n;
        }
    }

    /// A method to move on to title scene from prepare scene.
    ///
    /// WARN: If the scene isn't prepare scene, it does nothing.
    pub fn on_window_created(&mut self, width: f32, height: f32) {
        match self.scene {
            Scene::PrepareScene => {
                self.scene = Scene::TitleScene(TitleSceneState::new(width, height))
            }
            _ => return,
        }
    }

    /// A method to resize something that depends on the window size.
    pub fn resize(&mut self, width: f32, height: f32) {
        match &mut self.scene {
            Scene::GameScene(n) => n.resize(width, height),
            _ => (),
        }
    }
}
