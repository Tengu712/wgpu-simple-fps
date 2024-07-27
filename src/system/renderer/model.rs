use crate::util::memory;
use std::mem;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};

struct Vertex {
    _position: [f32; 4],
}

/// A vertex buffer layout on shader.wgsl.
pub(super) const VERTEX_BUFFER_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as u64,
    step_mode: VertexStepMode::Vertex,
    attributes: &[VertexAttribute {
        format: VertexFormat::Float32x4,
        offset: 0,
        shader_location: 0,
    }],
};

/// A struct for the vertex buffer and the index buffer of a square model.
pub(super) struct SquareModel {
    pub(super) vertex_buffer: Buffer,
    pub(super) index_buffer: Buffer,
    pub(super) index_count: usize,
}

impl SquareModel {
    pub(super) fn new(device: &Device) -> Self {
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

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::slice_to_u8slice(VERTEX_DATA),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::slice_to_u8slice(INDEX_DATA),
            usage: BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            index_count: INDEX_DATA.len(),
        }
    }
}
