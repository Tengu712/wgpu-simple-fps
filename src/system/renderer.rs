use futures::executor;
use std::{error::Error, sync::Arc};
use wgpu::{
    Backends, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance,
    InstanceDescriptor, Limits, LoadOp, MemoryHints, Operations, PowerPreference, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
    SurfaceConfiguration, TextureUsages, TextureViewDescriptor,
};
use winit::window::Window;

const CLEAR_COLOR: Color = Color {
    r: 0.2,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};

pub struct Renderer<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
}

impl<'a> Renderer<'a> {
    pub fn new(window: Arc<Window>) -> Self {
        // create an instance
        let instance_descriptor = InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        };
        let instance = Instance::new(instance_descriptor);

        // create a surface
        let surface = match instance.create_surface(Arc::clone(&window)) {
            Ok(n) => n,
            Err(e) => {
                error!("State.new", "failed to create a surface: {}", e.to_string());
                std::process::exit(1);
            }
        };

        // get an adapter
        let request_adapter_option = RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };
        let request = instance.request_adapter(&request_adapter_option);
        let adapter = match executor::block_on(request) {
            Some(n) => n,
            None => {
                error!("State.new", "failed to get an adapter.");
                std::process::exit(1);
            }
        };
        info!(
            "State.new",
            "the adapter is selected: {}.",
            adapter.get_info().name
        );

        // get a device and a queue
        let device_descriptor = DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::default(),
            memory_hints: MemoryHints::MemoryUsage,
        };
        let request = adapter.request_device(&device_descriptor, None);
        let (device, queue) = match executor::block_on(request) {
            Ok(n) => n,
            Err(e) => {
                error!(
                    "State.new",
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
        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_capabilities.present_modes[0],
            view_formats: Vec::new(),
            alpha_mode: surface_capabilities.alpha_modes[0],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &surface_config);

        // finish
        Self {
            surface,
            device,
            queue,
        }
    }

    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        let frame = self.surface.get_current_texture()?;
        let view = frame.texture.create_view(&TextureViewDescriptor::default());
        let mut command_encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let render_pass_descriptor = RenderPassDescriptor {
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
            };
            command_encoder.begin_render_pass(&render_pass_descriptor);
        }
        self.queue.submit(Some(command_encoder.finish()));
        frame.present();
        Ok(())
    }
}
