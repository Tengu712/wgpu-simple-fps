use wgpu::Device;

use super::{Model, Vertex};

impl Model {
    /// A static method to create a rectangle model.
    ///
    /// The vertices of this model are connected in a clockwise order
    /// when viewed along the positive z-axis in the local coordinate system.
    pub fn rectangle(device: &Device, width: f32, height: f32) -> Self {
        let hw = width / 2.0;
        let hh = height / 2.0;
        let vertex_data = Vec::from([
            // bottom left
            Vertex {
                _position: [-hw, -hh, 0.0, 1.0],
                _tex_coord: [0.0, 1.0],
            },
            // top left
            Vertex {
                _position: [-hw, hh, 0.0, 1.0],
                _tex_coord: [0.0, 0.0],
            },
            // top right
            Vertex {
                _position: [hw, hh, 0.0, 1.0],
                _tex_coord: [1.0, 0.0],
            },
            // bottom right
            Vertex {
                _position: [hw, -hh, 0.0, 1.0],
                _tex_coord: [1.0, 1.0],
            },
        ]);
        let index_data = Vec::from([0, 1, 2, 0, 2, 3]);
        Self::from(device, &vertex_data, &index_data)
    }
}
