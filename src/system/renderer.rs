mod model;
mod world;

use crate::util::{camera::CameraController, instance::InstanceController};
use futures::executor;
use std::{error::Error, sync::Arc};
use wgpu::{
    Backends, Color, CommandEncoder, CommandEncoderDescriptor, Device, DeviceDescriptor, Features,
    Instance, InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PowerPreference, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
    SurfaceCapabilities, SurfaceConfiguration, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};
use winit::window::Window;
use world::WorldPipeline;

const CLEAR_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

/// A enum for enumerating requests for a renderer.
pub enum RenderRequest {
    UpdateCamera(CameraController),
    Draw(Vec<InstanceController>),
}

/// A renderer on WebGPU.
///
/// It's depends on winit window.
/// The lifetime `'a` refers to the surface's lifetime, which is the same as the window's.
pub struct Renderer<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    surface_capabilities: SurfaceCapabilities,
    surface_format: TextureFormat,
    world_pipeline: WorldPipeline,
}

impl<'a> Renderer<'a> {
    /// A constructor.
    ///
    /// The arc of window is cloned in this.
    pub fn new(window: Arc<Window>) -> Self {
        // create an instance
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        // create a surface
        let surface = match instance.create_surface(Arc::clone(&window)) {
            Ok(n) => n,
            Err(e) => {
                error!(
                    "Renderer.new",
                    "failed to create a surface: {}",
                    e.to_string()
                );
                std::process::exit(1);
            }
        };

        // get an adapter
        let request = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        });
        let adapter = match executor::block_on(request) {
            Some(n) => n,
            None => {
                error!("Renderer.new", "failed to get an adapter.");
                std::process::exit(1);
            }
        };
        info!(
            "Renderer.new",
            "the adapter selected: {}.",
            adapter.get_info().name
        );
        info!(
            "Renderer.new",
            "the backend selected: {}.",
            adapter.get_info().backend.to_str()
        );

        // get a device and a queue
        let request = adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
                memory_hints: MemoryHints::MemoryUsage,
            },
            None,
        );
        let (device, queue) = match executor::block_on(request) {
            Ok(n) => n,
            Err(e) => {
                error!(
                    "Renderer.new",
                    "failed to get a device and a queue: {}",
                    e.to_string()
                );
                std::process::exit(1);
            }
        };

        // configure the surface
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_capabilities.formats[0]);
        surface.configure(
            &device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: surface_format,
                width: window.inner_size().width,
                height: window.inner_size().height,
                present_mode: surface_capabilities.present_modes[0],
                view_formats: Vec::new(),
                alpha_mode: surface_capabilities.alpha_modes[0],
                desired_maximum_frame_latency: 2,
            },
        );

        // create render pipelines
        let world_pipeline = WorldPipeline::new(&device, surface_format.into());

        // finish
        info!("Renderer.new", "renderer created.");
        Self {
            surface,
            device,
            queue,
            surface_capabilities,
            surface_format,
            world_pipeline,
        }
    }

    /// A method to resize the size of surface.
    pub fn resize(&self, width: u32, height: u32) {
        self.surface.configure(
            &self.device,
            &SurfaceConfiguration {
                usage: TextureUsages::RENDER_ATTACHMENT,
                format: self.surface_format,
                width,
                height,
                present_mode: self.surface_capabilities.present_modes[0],
                view_formats: Vec::new(),
                alpha_mode: self.surface_capabilities.alpha_modes[0],
                desired_maximum_frame_latency: 2,
            },
        );
        info!("Renderer.resize", "surface resized: {}x{}.", width, height);
    }

    /// A method to render entities.
    ///
    /// It locks the thread until a framebuffer is presented.
    ///
    /// The render pipeline of world.wgsl is attached first.
    pub fn render(&self, render_requests: Vec<RenderRequest>) -> Result<(), Box<dyn Error>> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        self.do_requests(render_requests, &mut command_encoder, &view);

        self.queue.submit(Some(command_encoder.finish()));
        frame.present();
        Ok(())
    }

    fn do_requests(
        &self,
        render_requests: Vec<RenderRequest>,
        command_encoder: &mut CommandEncoder,
        render_target_view: &TextureView,
    ) {
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: render_target_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(CLEAR_COLOR),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        self.world_pipeline.attach(&mut render_pass);

        for request in render_requests {
            match request {
                RenderRequest::UpdateCamera(camera_controller) => self
                    .world_pipeline
                    .enqueue_update_camera(&self.queue, &camera_controller),
                RenderRequest::Draw(instance_controllers) => {
                    self.world_pipeline
                        .draw(&mut render_pass, &self.queue, instance_controllers)
                }
            }
        }
    }
}
