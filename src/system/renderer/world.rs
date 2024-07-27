//! This code is an implementation of world.wgsl.

use super::model::Model;
use crate::util::{camera::CameraController, memory};
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
    _world_matrix: Mat4,
}
struct Vertex {
    _position: [f32; 4],
}

const VERTEX_BUFFER_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as u64,
    step_mode: VertexStepMode::Vertex,
    attributes: &[VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: 0,
        shader_location: 0,
    }],
};
const VERTEX_DATA: &[Vertex] = &[
    Vertex {
        _position: [-0.5, 0.5, 0.0, 1.0],
    }, // top left
    Vertex {
        _position: [-0.5, -0.5, 0.0, 1.0],
    }, // bottom left
    Vertex {
        _position: [0.5, -0.5, 0.0, 1.0],
    }, // bottom right
    Vertex {
        _position: [0.5, 0.5, 0.0, 1.0],
    }, // top right
];
const INDEX_DATA: &[u16] = &[0, 1, 2, 0, 2, 3];

/// A pipeline implementaion of world.wgsl.
pub(super) struct WorldPipeline {
    render_pipeline: RenderPipeline,
    camera_buffer: Buffer,
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
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(mem::size_of::<Camera>() as u64),
                },
                count: None,
            }],
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
                buffers: &[VERTEX_BUFFER_LAYOUT],
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
            _world_matrix: Mat4::IDENTITY,
        };
        let camera_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::anything_to_u8slice(&CAMERA),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        // create a bind group, @group(0)
        let bind_group_0 = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bind_group_0_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        // create a square model
        let square_model = Model::from(device, VERTEX_DATA, INDEX_DATA);

        Self {
            render_pipeline,
            camera_buffer,
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

    pub(super) fn draw(&self, render_pass: &mut RenderPass) {
        render_pass.draw_indexed(0..self.square_model.index_count as u32, 0, 0..1);
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
                0.0,
                1000.0,
            ),
            _view_matrix: Mat4::look_to_lh(
                camera_controller.position,
                camera_controller.direction,
                camera_controller.up,
            ),
            _world_matrix: Mat4::IDENTITY,
        };
        queue.write_buffer(&self.camera_buffer, 0, memory::anything_to_u8slice(&camera));
    }
}
