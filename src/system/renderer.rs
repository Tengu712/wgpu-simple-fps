mod model;
mod uniform;

use futures::executor;
use model::SquareModel;
use std::{borrow::Cow, error::Error, sync::Arc};
use uniform::Group0;
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, FragmentState,
    IndexFormat, Instance, InstanceDescriptor, Limits, LoadOp, MemoryHints, MultisampleState,
    Operations, PipelineLayoutDescriptor, PowerPreference, PrimitiveState, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, StoreOp, Surface,
    SurfaceCapabilities, SurfaceConfiguration, TextureFormat, TextureUsages, TextureViewDescriptor,
    VertexState,
};
use winit::window::Window;

const SHADER: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader.wgsl"));
const CLEAR_COLOR: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};

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
    render_pipeline: RenderPipeline,
    group0: Group0,
    model: SquareModel,
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

        // create a shader module
        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::from(SHADER)),
        });

        // create a pipeline layout
        let bind_group_layout = uniform::create_bind_group_layout(&device);
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // create a render pipeline
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[model::VERTEX_BUFFER_LAYOUT],
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(surface_format.into())],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // create a bind group, @group(0)
        let group0 = Group0::new(
            &device,
            &bind_group_layout,
            window.inner_size().width as f32,
            window.inner_size().height as f32,
        );

        // create a model
        let model = SquareModel::new(&device);

        // finish
        info!("Renderer.new", "renderer created.");
        Self {
            surface,
            device,
            queue,
            surface_capabilities,
            surface_format,
            render_pipeline,
            group0,
            model,
        }
    }

    /// A method to render entities.
    ///
    /// It locks the thread until a framebuffer is presented.
    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
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
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.group0.bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.model.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.model.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.model.index_count as u32, 0, 0..1);
        }
        self.queue.submit(Some(command_encoder.finish()));
        frame.present();
        Ok(())
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
        // TODO: update projection matrix
        info!("Renderer.resize", "surface resized: {}x{}.", width, height);
    }
}
