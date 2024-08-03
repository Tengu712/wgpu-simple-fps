pub mod model;
mod shader;
mod texture;

use crate::util::{camera::CameraController, instance::InstanceController};
use futures::executor;
use model::{Model, ModelId};
use shader::{skybox::SkyboxPipeline, world::WorldPipeline};
use std::{collections::HashMap, sync::Arc};
use wgpu::{
    Backends, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, MemoryHints, PowerPreference, Queue, RequestAdapterOptions,
    Surface, SurfaceCapabilities, SurfaceConfiguration, TextureFormat, TextureUsages,
    TextureViewDescriptor,
};
use winit::window::Window;

/// A enum for enumerating requests for a renderer.
pub enum RenderRequest {
    UpdateCamera(CameraController),
    DrawSkybox,
    /// List the instance information.
    /// The indices in this array correspond to the indices in the instance buffer.
    /// If no update is needed, store `None`.
    UpdateWorldInstances(Vec<Option<InstanceController>>),
    /// Specify the model id to attach and the start and end index of instances.
    /// To reduce draw calls, you should group the same models together whenever possible.
    DrawWorld(Vec<(ModelId, u32, u32)>),
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
    skybox_pipeline: SkyboxPipeline,
    world_pipeline: WorldPipeline,
    models: HashMap<ModelId, Model>,
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
        let skybox_pipeline = SkyboxPipeline::new(
            &device,
            &queue,
            surface_format.into(),
            window.inner_size().width,
            window.inner_size().height,
        );
        let world_pipeline = WorldPipeline::new(
            &device,
            surface_format.into(),
            window.inner_size().width,
            window.inner_size().height,
        );

        // create models
        let mut models = HashMap::new();
        models.insert(ModelId::Square, Model::square(&device));
        models.insert(ModelId::Sphere, Model::sphere(&device));

        // finish
        info!("Renderer.new", "renderer created.");
        Self {
            surface,
            device,
            queue,
            surface_capabilities,
            surface_format,
            skybox_pipeline,
            world_pipeline,
            models,
        }
    }

    /// A method to resize the size of textures.
    pub fn resize(&mut self, width: u32, height: u32) {
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
        self.skybox_pipeline.resize(&self.device, width, height);
        self.world_pipeline.resize(&self.device, width, height);
        info!("Renderer.resize", "textures resized: {}x{}.", width, height);
    }

    /// A method to render entities.
    ///
    /// It locks the thread until a framebuffer is presented.
    pub fn render(&self, render_requests: Vec<RenderRequest>) {
        let surface_texture = match self.surface.get_current_texture() {
            Ok(n) => n,
            Err(e) => {
                warn!(
                    "Renderer.render",
                    "failed to get surface current texture: {}",
                    e.to_string()
                );
                return;
            }
        };
        let render_target_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());
        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        for request in render_requests {
            match request {
                RenderRequest::UpdateCamera(camera_controller) => {
                    self.world_pipeline
                        .enqueue_update_camera(&self.queue, &camera_controller);
                    self.skybox_pipeline
                        .enqueue_update_camera(&self.queue, &camera_controller);
                }
                RenderRequest::DrawSkybox => {
                    self.skybox_pipeline.draw(
                        &mut command_encoder,
                        &render_target_view,
                        &self.models[&ModelId::Sphere],
                    );
                }
                RenderRequest::UpdateWorldInstances(instance_controllers) => {
                    self.world_pipeline
                        .update_instances(&self.queue, instance_controllers);
                }
                RenderRequest::DrawWorld(instances) => {
                    self.world_pipeline.draw(
                        &mut command_encoder,
                        &render_target_view,
                        &self.models,
                        instances,
                    );
                }
            }
        }

        self.queue.submit(Some(command_encoder.finish()));
        surface_texture.present();
    }
}
