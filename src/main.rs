//! This application is a simple FPS game with WebGPU for Windows/macOS/Linux.
//!
//! This is based on winit ApplicationHandler.
//! This creates a window with the following features:
//! - fullscreen
//! - unresizable
//! - the maximize button is disabled

#[macro_use]
mod log;
mod system;

use std::{error::Error, process, sync::Arc};
use system::renderer::Renderer;
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
        let enabled_buttons_bits = WindowButtons::CLOSE.bits() | WindowButtons::MINIMIZE.bits();
        let enabled_buttons = WindowButtons::from_bits(enabled_buttons_bits).unwrap();
        window.set_enabled_buttons(enabled_buttons);
        info!("Application.resumed", "a window has been created.");

        // create an arc of the window
        let window = Arc::new(window);

        // create a renderer
        let renderer = Renderer::new(window.clone());

        // finish
        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Destroyed => event_loop.exit(),
            WindowEvent::Resized(PhysicalSize { width, height }) => {
                if let Some(renderer) = &self.renderer {
                    renderer.resize(width, height);
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        if let Some(renderer) = &self.renderer {
            if let Err(e) = renderer.render() {
                warn!(
                    "Application.about_to_wait",
                    "failed to render: {}",
                    e.to_string()
                );
            }
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
