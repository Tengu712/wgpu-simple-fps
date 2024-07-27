use crate::util::memory;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

/// A struct for a model.
pub struct Model {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_count: usize,
}

impl Model {
    pub fn from<TVertex, TIndex>(
        device: &Device,
        vertex_data: &[TVertex],
        index_data: &[TIndex],
    ) -> Self {
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
