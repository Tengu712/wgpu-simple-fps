//! This application is a simple FPS game with WebGPU for Windows/macOS/Linux.
//!
//! This is based on winit ApplicationHandler.
//! This creates a window with the following features:
//! - fullscreen
//! - unresizable
//! - the maximize button is disabled

#[macro_use]
mod log;
mod game;
mod system;
mod util;

use game::Game;
use std::{error::Error, process, sync::Arc};
use system::{input::InputManager, renderer::Renderer};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Fullscreen, Window, WindowButtons, WindowId},
};

#[derive(Default)]
struct Application<'a> {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer<'a>>,
    input_manager: Option<InputManager>,
    game: Option<Game>,
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // check
        if self.window.is_some() {
            return;
        }

        // create a window
        let window_attributes = Window::default_attributes()
            .with_title("WebGPU Simple FPS")
            .with_resizable(false)
            .with_fullscreen(Some(Fullscreen::Borderless(None)));
        let window = match event_loop.create_window(window_attributes) {
            Ok(n) => n,
            Err(e) => {
                error!(
                    "Application.resumed",
                    "failed to create a window: {}",
                    e.to_string()
                );
                std::process::exit(1);
            }
        };
        window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);
        info!("Application.resumed", "window created.");

        // create an arc of the window
        let window = Arc::new(window);

        // create a renderer
        let renderer = Renderer::new(window.clone());

        // create an input manager
        let input_manager = InputManager::new();

        // create a game
        let game = Game::new(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );

        // finish
        info!("Application.resumed", "initialization done.");
        self.window = Some(window);
        self.renderer = Some(renderer);
        self.input_manager = Some(input_manager);
        self.game = Some(game);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        if self.window.is_none() {
            warn!("Application.window_event", "window is none.");
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Destroyed => event_loop.exit(),
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                self.renderer.as_ref().unwrap().resize(width, height);
                self.game
                    .as_mut()
                    .unwrap()
                    .resize(width as f32, height as f32);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.input_manager.as_mut().unwrap().update_key_state(event);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                self.input_manager
                    .as_mut()
                    .unwrap()
                    .update_mouse_state(button, state);
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if self.window.is_none() {
            warn!("Application.about_to_wait", "window is none.");
            return;
        }

        let input_states = self.input_manager.as_ref().unwrap().get();
        let mut render_requests = Vec::new();

        self.game
            .as_mut()
            .unwrap()
            .update(input_states, &mut render_requests);

        if let Err(e) = self.renderer.as_ref().unwrap().render(render_requests) {
            warn!(
                "Application.about_to_wait",
                "failed to render: {}",
                e.to_string()
            );
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut Application::default())?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        error!("main", "crashed: {}", e.to_string());
        process::exit(1);
    }
}
