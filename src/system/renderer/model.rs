use crate::util::memory;
use std::mem;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexStepMode,
};

const SQUARE_OBJ: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/model/square.obj"));
const SPHERE_OBJ: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/model/sphere.obj"));

/// A struct for the data of a single vertex in the common model within this application’s shader.
///
/// NOTE: You can create custom vertex data to pass to `Model::from()`.
pub struct Vertex {
    pub _position: [f32; 4],
    pub _normal: [f32; 4],
    pub _tex_coord: [f32; 2],
}

/// A constant for descripting a vertex data layout to create a render pipeline.
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
            format: VertexFormat::Float32x4,
            offset: mem::size_of::<[f32; 4]>() as u64,
            shader_location: 1,
        },
        VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: mem::size_of::<[f32; 4]>() as u64 * 2,
            shader_location: 2,
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

    /// A static method to create a model from a Wavefront OBJ format text.
    ///
    /// NOTE: You can call this to create a custom model.
    ///
    /// WARN: Each parameter specified in a face element must be pre-defined in the text.
    /// 
    /// WARN: Only the parameters v, vt, and vn are supported.
    ///       Unsupported parameters will be ignored.
    pub fn from_obj(device: &Device, obj: &str) -> Result<Self, String> {
        let mut v = Vec::new();
        let mut vt = Vec::new();
        let mut vn = Vec::new();
        let mut vertex_data = Vec::new();
        let mut index_data = Vec::new();
        for n in obj.lines() {
            let n = n.trim();
            if n.is_empty() || n.starts_with("#") {
                continue;
            }
            if n.starts_with("f") {
                let elements = n
                    .split(' ')
                    .skip(1)
                    .map(|n| {
                        n.split('/')
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>()
                    })
                    .collect::<Vec<Vec<usize>>>();
                for e in elements {
                    let p: &Vec<f32> = &v[e[0] - 1];
                    let t: &Vec<f32> = &vt[e[1] - 1];
                    let n: &Vec<f32> = &vn[e[2] - 1];
                    vertex_data.push(Vertex {
                        _position: [p[0], p[1], p[2], 1.0],
                        _normal: [n[0], n[1], n[2], 1.0],
                        _tex_coord: [t[0], t[1]],
                    });
                    index_data.push(index_data.len() as u16);
                }
                continue;
            }
            let data = n
                .split(' ')
                .skip(1)
                .map(|n| n.parse::<f32>().unwrap())
                .collect::<Vec<f32>>();
            if n.starts_with("v ") {
                v.push(data);
            } else if n.starts_with("vt ") {
                vt.push(data);
            } else if n.starts_with("vn ") {
                vn.push(data);
            }
        }
        Ok(Self::from(device, &vertex_data, &index_data))
    }

    /// A static method to create a square model.
    pub fn square(device: &Device) -> Self {
        Self::from_obj(device, SQUARE_OBJ).unwrap()
    }

    /// A static method to create a sphere model.
    pub fn sphere(device: &Device) -> Self {
        Self::from_obj(device, SPHERE_OBJ).unwrap()
    }
}
