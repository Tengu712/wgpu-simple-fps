//! This application is a simple FPS game with WebGPU for Windows/macOS/Linux.
//!
//! This is based on winit ApplicationHandler.
//! This creates a window with the following features:
//! - fullscreen
//! - unresizable
//! - the maximize button is disabled
//! - the cursor is invisible
//! - the cursor is confined to the window area (if possible)

#[macro_use]
mod log;
mod game;
mod system;
mod util;

use game::scene::SceneManager;
use std::{error::Error, process, sync::Arc};
use system::{input::InputManager, renderer::Renderer};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::KeyCode,
    window::{Fullscreen, Window, WindowButtons, WindowId},
};

fn set_cursor_center(window: &Arc<Window>) -> (f64, f64) {
    let x = window.inner_size().width as f64 / 2.0;
    let y = window.inner_size().height as f64 / 2.0;
    if let Err(e) = window.set_cursor_position(PhysicalPosition::new(x, y)) {
        warn!(
            "set_cursor_center",
            "failed to set cursor position center of window: {}",
            e.to_string()
        );
    }
    (x, y)
}

struct Application<'a> {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer<'a>>,
    input_manager: InputManager,
    scene_manager: SceneManager,
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // check
        if self.window.is_some() {
            return;
        }

        // get a primary monitor
        let primary_monitor = if let Some(n) = event_loop.primary_monitor() {
            n
        } else {
            error!("Application.resumed", "no primary monitor is found.");
            process::exit(1);
        };

        // create a window
        let window_attributes = Window::default_attributes()
            .with_title("WebGPU Simple FPS")
            .with_resizable(false)
            .with_inner_size(primary_monitor.size())
            .with_fullscreen(Some(Fullscreen::Borderless(Some(primary_monitor))));
        let window = match event_loop.create_window(window_attributes) {
            Ok(n) => n,
            Err(e) => {
                error!(
                    "Application.resumed",
                    "failed to create a window: {}",
                    e.to_string()
                );
                process::exit(1);
            }
        };
        let window = Arc::new(window);

        // configure the window
        window.set_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE);
        window.set_cursor_visible(false);
        info!("Application.resumed", "window created.");

        // create a renderer
        let renderer = Renderer::new(window.clone());

        // move cursor center
        self.input_manager
            .set_cursor_position(set_cursor_center(&window));

        // move on to title scene and resize
        self.scene_manager.on_window_created(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );
        self.scene_manager.resize(
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );

        // finish
        info!("Application.resumed", "initialization done.");
        self.window = Some(window);
        self.renderer = Some(renderer);
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
                self.renderer.as_mut().unwrap().resize(width, height);
                self.scene_manager.resize(width as f32, height as f32);
                info!("Application.window_event", "resized: {}x{}.", width, height);
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.physical_key == KeyCode::Escape {
                    event_loop.exit();
                }
                self.input_manager.update_key_state(event);
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                self.input_manager.update_mouse_state(button, state);
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.input_manager.update_cursor_state(position);
                let cursor_position = set_cursor_center(self.window.as_ref().unwrap());
                self.input_manager.set_cursor_position(cursor_position);
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if self.window.is_none() {
            warn!("Application.about_to_wait", "window is none.");
            return;
        }

        let mut render_requests = Vec::new();

        self.scene_manager
            .update(self.input_manager.get(), &mut render_requests);

        self.input_manager.go_next();
        self.renderer.as_ref().unwrap().render(render_requests);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut Application {
        window: None,
        renderer: None,
        input_manager: InputManager::new((0.0, 0.0)),
        scene_manager: SceneManager::new(),
    })?;
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        error!("main", "crashed: {}", e.to_string());
        process::exit(1);
    }
}
