pub mod rectangle;
pub mod sphere;

use crate::util::memory;
use std::mem;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};

/// A struct for the data of a single vertex in the common model within this application’s shader.
///
/// NOTE: You can create custom vertex data to pass to `Model::from()`.
pub struct Vertex {
    pub _position: [f32; 4],
    pub _tex_coord: [f32; 2],
}

/// A constant to define the vertex data layout for creating a render pipeline.
pub const VERTEX_BUFFER_LAYOUTS: &[VertexBufferLayout] = &[VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as u64,
    step_mode: VertexStepMode::Vertex,
    attributes: &[
        VertexAttribute {
            format: VertexFormat::Float32x4,
            offset: 0,
            shader_location: 0,
        },
        VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: mem::size_of::<[f32; 4]>() as u64,
            shader_location: 1,
        },
    ],
}];

/// A struct for a model.
pub struct Model {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_count: usize,
}

impl Model {
    /// A static method to create a model from a vertex data array and an index data array.
    ///
    /// NOTE: You can call this to create a custom model.
    pub fn from(device: &Device, vertex_data: &[Vertex], index_data: &[u16]) -> Self {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::slice_to_u8slice(vertex_data),
            usage: BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: memory::slice_to_u8slice(index_data),
            usage: BufferUsages::INDEX,
        });
        Self {
            vertex_buffer,
            index_buffer,
            index_count: index_data.len(),
        }
    }
}
