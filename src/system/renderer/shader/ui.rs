use crate::{
    system::renderer::model::{self, Model},
    util::{instance::InstanceController, memory},
};
use glam::Mat4;
use std::{borrow::Cow, cmp, mem};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferSize, BufferUsages, Color,
    ColorTargetState, CommandEncoder, Device, FragmentState, IndexFormat, LoadOp, MultisampleState,
    Operations, PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, ShaderStages, StoreOp, TextureView, VertexState,
};

const SHADER: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/shader/ui.wgsl"));

struct Camera {
    _projection_matrix: Mat4,
}
#[derive(Clone)]
struct Instance {
    _model_matrix: Mat4,
}
const MAX_INSTANCE_COUNT: u64 = 16;

/// A struct for descripting the detail of a draw request on an ui pipeline.
pub struct DrawUiDescriptor {
    /// Specify the color that the pipeline clears the render target view with.
    /// If it's `None`, the pipeline doesn't clear the render target view.
    pub clear_color: Option<[f64; 3]>,
    /// Specify the start and end index of instances.
    /// To reduce draw calls, you should group the same models together whenever possible.
    pub instance_indices: Vec<(u32, u32)>,
}

/// A pipeline implementaion of ui.wgsl.
pub struct UiPipeline {
    render_pipeline: RenderPipeline,
    camera_buffer: Buffer,
    instance_buffer: Buffer,
    bind_group_0: BindGroup,
}

impl UiPipeline {
    /// A constructor.
    pub fn new(
        device: &Device,
        color_target_state: ColorTargetState,
        width: u32,
        height: u32,
    ) -> Self {
        // create a shader module
        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::from(SHADER)),
        });

        // create a bind group layout, @group(0)
        let bind_group_0_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(mem::size_of::<Camera>() as u64),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(
                            mem::size_of::<Instance>() as u64 * MAX_INSTANCE_COUNT,
                        ),
                    },
                    count: None,
                },
            ],
        });

        // create a pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_0_layout],
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
                buffers: model::VERTEX_BUFFER_LAYOUTS,
            },
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[Some(color_target_state)],
            }),
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // create a camera uniform buffer
        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;
        let camera = Camera {
            _projection_matrix: Mat4::orthographic_lh(
                -half_width,
                half_width,
                -half_height,
                half_height,
                0.0,
                1000.0,
            ),
        };
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::anything_to_u8slice(&camera),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // create a instance uniform buffer
        let instances = (0..MAX_INSTANCE_COUNT)
            .into_iter()
            .map(|_| Instance {
                _model_matrix: Mat4::IDENTITY,
            })
            .collect::<Vec<Instance>>();
        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::slice_to_u8slice(instances.as_slice()),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // create a bind group, @group(0)
        let bind_group_0 = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_0_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: instance_buffer.as_entire_binding(),
                },
            ],
        });

        Self {
            render_pipeline,
            camera_buffer,
            instance_buffer,
            bind_group_0,
        }
    }

    /// A method to update instance buffer.
    ///
    /// WARN: Indices exceeding `MAX_INSTANCES_COUNT` will be completely ignored.
    pub fn update_instances(
        &self,
        queue: &Queue,
        instance_controllers: Vec<Option<InstanceController>>,
    ) {
        let mut instances_group = Vec::new();
        let mut instances = Vec::new();
        let mut current_i = 0;
        for (i, n) in instance_controllers.into_iter().enumerate() {
            if i >= MAX_INSTANCE_COUNT as usize {
                break;
            }
            if let Some(n) = n {
                if instances.is_empty() {
                    current_i = i;
                }
                instances.push(Instance {
                    _model_matrix: Mat4::from_scale_rotation_translation(
                        n.scale, n.rotation, n.position,
                    ),
                });
            } else if !instances.is_empty() {
                instances_group.push((current_i, instances.clone()));
                instances.clear();
            }
        }
        if !instances.is_empty() {
            instances_group.push((current_i, instances.clone()));
        }
        for (i, n) in instances_group {
            queue.write_buffer(
                &self.instance_buffer,
                mem::size_of::<Instance>() as u64 * i as u64,
                memory::slice_to_u8slice(n.as_slice()),
            );
        }
    }

    /// A method to draw models.
    ///
    /// WARN: Indices exceeding `MAX_INSTANCES_COUNT` will be completely ignored.
    pub fn draw<'a>(
        &self,
        command_encoder: &'a mut CommandEncoder,
        render_target_view: &TextureView,
        square: &Model,
        descriptor: DrawUiDescriptor,
    ) {
        // begin render pass
        let load = if let Some(n) = descriptor.clear_color {
            LoadOp::Clear(Color {
                r: n[0],
                g: n[1],
                b: n[2],
                a: 1.0,
            })
        } else {
            LoadOp::Load
        };
        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(RenderPassColorAttachment {
                view: render_target_view,
                resolve_target: None,
                ops: Operations {
                    load,
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // prepare
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group_0, &[]);
        render_pass.set_vertex_buffer(0, square.vertex_buffer.slice(..));
        render_pass.set_index_buffer(square.index_buffer.slice(..), IndexFormat::Uint16);

        // draw
        for (start, end) in descriptor.instance_indices {
            if start >= MAX_INSTANCE_COUNT as u32 {
                continue;
            }
            let end = cmp::min(MAX_INSTANCE_COUNT as u32, end);
            render_pass.draw_indexed(0..square.index_count as u32, 0, start..end);
        }
    }

    /// A method to resize the camera size.
    ///
    /// It enqueues a `write_buffer` queue to `queue`.
    pub fn resize(&mut self, queue: &Queue, width: u32, height: u32) {
        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;
        let camera = Camera {
            _projection_matrix: Mat4::orthographic_lh(
                -half_width,
                half_width,
                -half_height,
                half_height,
                0.0,
                1000.0,
            ),
        };
        queue.write_buffer(&self.camera_buffer, 0, memory::anything_to_u8slice(&camera));
    }
}
