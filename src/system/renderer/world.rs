//! This code is an implementation of world.wgsl.

use super::model::Model;
use crate::util::{camera::CameraController, instance::InstanceController, memory};
use glam::Mat4;
use std::{borrow::Cow, mem};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferSize, BufferUsages,
    ColorTargetState, Device, FragmentState, IndexFormat, MultisampleState,
    PipelineLayoutDescriptor, PrimitiveState, Queue, RenderPass, RenderPipeline,
    RenderPipelineDescriptor, ShaderModuleDescriptor, ShaderSource, ShaderStages, VertexAttribute,
    VertexBufferLayout, VertexFormat, VertexState, VertexStepMode,
};

const SHADER: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/world.wgsl"));

struct Camera {
    _projection_matrix: Mat4,
    _view_matrix: Mat4,
}
struct Instance {
    _model_matrix: Mat4,
}
struct VertexInput {
    _position: [f32; 4],
}

const VERTEX_BUFFER_LAYOUTS: &[VertexBufferLayout] = &[VertexBufferLayout {
    array_stride: mem::size_of::<VertexInput>() as u64,
    step_mode: VertexStepMode::Vertex,
    attributes: &[VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: 0,
        shader_location: 0,
    }],
}];
const VERTEX_DATA: &[VertexInput] = &[
    VertexInput {
        _position: [-0.5, 0.5, 0.0, 1.0],
    }, // top left
    VertexInput {
        _position: [-0.5, -0.5, 0.0, 1.0],
    }, // bottom left
    VertexInput {
        _position: [0.5, -0.5, 0.0, 1.0],
    }, // bottom right
    VertexInput {
        _position: [0.5, 0.5, 0.0, 1.0],
    }, // top right
];
const INDEX_DATA: &[u16] = &[0, 1, 2, 0, 2, 3];
const MAX_INSTANCE_COUNT: u64 = 4;

/// A pipeline implementaion of world.wgsl.
pub(super) struct WorldPipeline {
    render_pipeline: RenderPipeline,
    camera_buffer: Buffer,
    instance_buffer: Buffer,
    bind_group_0: BindGroup,
    square_model: Model,
}

impl WorldPipeline {
    pub(super) fn new(device: &Device, color_target_state: ColorTargetState) -> Self {
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
                buffers: VERTEX_BUFFER_LAYOUTS,
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
        const CAMERA: Camera = Camera {
            _projection_matrix: Mat4::IDENTITY,
            _view_matrix: Mat4::IDENTITY,
        };
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::anything_to_u8slice(&CAMERA),
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

        // create a square model
        let square_model = Model::from(device, VERTEX_DATA, INDEX_DATA);

        Self {
            render_pipeline,
            camera_buffer,
            instance_buffer,
            bind_group_0,
            square_model,
        }
    }

    pub(super) fn attach(&self, render_pass: &mut RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group_0, &[]);
        render_pass.set_vertex_buffer(0, self.square_model.vertex_buffer.slice(..));
        render_pass.set_index_buffer(
            self.square_model.index_buffer.slice(..),
            IndexFormat::Uint16,
        );
    }

    /// A method to draw squares.
    ///
    /// It updates @group(0) @binding(0) instance uniform buffer and draws all at one time.
    ///
    /// WARN: if `model_controllers.len()` is larger than `MAX_INSTANCE_COUNT`,
    ///       the excess is completely ignored.
    ///
    /// WARN: If called more than once within the same render pass,
    ///       the instance buffer will be overwritten, affecting previous draws.
    ///
    /// OPTIMIZE: Not all instance information changes every frame.
    pub(super) fn draw(
        &self,
        render_pass: &mut RenderPass,
        queue: &Queue,
        instance_controllers: Vec<InstanceController>,
    ) {
        // update
        let mut count = 0;
        let mut instances = Vec::new();
        for n in instance_controllers {
            if count >= MAX_INSTANCE_COUNT {
                break;
            }
            count += 1;
            instances.push(Instance {
                _model_matrix: Mat4::from_scale_rotation_translation(
                    n.scale, n.rotation, n.position,
                ),
            });
        }
        queue.write_buffer(
            &self.instance_buffer,
            0,
            memory::slice_to_u8slice(instances.as_slice()),
        );

        // draw
        render_pass.draw_indexed(0..self.square_model.index_count as u32, 0, 0..count as u32);
    }

    pub(super) fn enqueue_update_camera(
        &self,
        queue: &Queue,
        camera_controller: &CameraController,
    ) {
        let camera = Camera {
            _projection_matrix: Mat4::perspective_lh(
                camera_controller.pov,
                camera_controller.width / camera_controller.height,
                0.1,
                1000.0,
            ),
            _view_matrix: Mat4::look_to_lh(
                camera_controller.position,
                camera_controller.direction,
                camera_controller.up,
            ),
        };
        queue.write_buffer(&self.camera_buffer, 0, memory::anything_to_u8slice(&camera));
    }
}
