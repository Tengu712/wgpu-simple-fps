use wgpu::Device;

use super::{Model, Vertex};
use std::{f32::consts, process};

impl Model {
    /// A static method to create a sphere model.
    ///
    /// * `r` - the radius of the sphere
    /// * `latitude_count` - the number of divisions at this latitude
    /// * `longtitude_count` - the number of divisions at this longtitude
    ///
    /// When viewed from the outside, the vertices of all faces are connected in a clockwise order.
    ///
    /// WARN: `latitude_count` must be even number.
    pub fn sphere(device: &Device, r: f32, latitude_count: usize, longtitude_count: usize) -> Self {
        if latitude_count % 2 == 1 {
            error!(
                "Model.sphere",
                "tried to create a sphere whose the number of divisions at the latitude is odd number.",
            );
            process::exit(1);
        }

        // vertex data
        let mut vertex_data = Vec::new();
        // 1. north pole
        vertex_data.push(Vertex {
            _position: [0.0, r, 0.0, 1.0],
            _tex_coord: [0.25, 0.5],
        });
        // 2. northern hemisphere
        push_vertex_data(r, latitude_count, longtitude_count, true, &mut vertex_data);
        // 3. southern hemisphere
        push_vertex_data(r, latitude_count, longtitude_count, false, &mut vertex_data);
        // 4. south pole
        vertex_data.push(Vertex {
            _position: [0.0, -r, 0.0, 1.0],
            _tex_coord: [0.75, 0.5],
        });

        // index data
        let mut index_data = Vec::new();
        // 1. top
        for i in 1..(longtitude_count + 1) {
            let i = i as u16;
            index_data.push(0);
            index_data.push(i);
            index_data.push((i % longtitude_count as u16) + 1);
        }
        // 2. northern hemisphere
        push_index_data(latitude_count, longtitude_count, true, &mut index_data);
        // 3. southern hemisphere
        push_index_data(latitude_count, longtitude_count, false, &mut index_data);
        // 3. bottom
        for i in 1..(longtitude_count + 1) {
            let offset =
                longtitude_count as u16 + (latitude_count as u16 - 2) * longtitude_count as u16;
            let i = i as u16;
            index_data.push(offset + i);
            index_data.push(vertex_data.len() as u16 - 1);
            index_data.push(offset + (i % longtitude_count as u16) + 1);
        }

        // finish
        Self::from(device, &vertex_data, &index_data)
    }
}

fn push_vertex_data(
    r: f32,
    latitude_count: usize,
    longtitude_count: usize,
    is_northern: bool,
    vertex_data: &mut Vec<Vertex>,
) {
    let half_latitude_count = latitude_count / 2;
    let (start, end) = if is_northern {
        (1, half_latitude_count + 1)
    } else {
        (half_latitude_count, latitude_count)
    };
    let x_offset = 0.25 + if is_northern { 0.0 } else { 0.5 };
    let y_offset = 0.5;

    for lo in start..end {
        let theta_lo = consts::PI / latitude_count as f32 * lo as f32;
        let l = r * theta_lo.sin().abs();
        let y = r * theta_lo.cos();
        for la in 0..longtitude_count {
            let theta_la = 2.0 * consts::PI / longtitude_count as f32 * la as f32;
            let s = theta_la.sin();
            let c = theta_la.cos();
            let k = if is_northern { lo } else { latitude_count - lo };
            let k = 1.0 / half_latitude_count as f32 * k as f32;
            vertex_data.push(Vertex {
                _position: [l * s, y, l * c, 1.0],
                _tex_coord: [x_offset + s * 0.25 * k, y_offset + c * 0.5 * k],
            });
        }
    }
}

fn push_index_data(
    latitude_count: usize,
    longtitude_count: usize,
    is_northern: bool,
    index_data: &mut Vec<u16>,
) {
    let half_latitude_count = latitude_count / 2;
    let (start, end) = if is_northern {
        (0, half_latitude_count)
    } else {
        (half_latitude_count - 1, latitude_count - 2)
    };
    let offset = if is_northern {
        0
    } else {
        longtitude_count as u16
    };

    for lo in start..end {
        for la in 1..(longtitude_count + 1) {
            let la1 = la as u16;
            let la2 = (la1 % longtitude_count as u16) + 1;
            let u1 = la1 + longtitude_count as u16 * lo as u16;
            let u2 = la2 + longtitude_count as u16 * lo as u16;
            let b1 = la1 + longtitude_count as u16 * (lo + 1) as u16;
            let b2 = la2 + longtitude_count as u16 * (lo + 1) as u16;
            index_data.push(u1 + offset);
            index_data.push(b1 + offset);
            index_data.push(b2 + offset);
            index_data.push(u1 + offset);
            index_data.push(b2 + offset);
            index_data.push(u2 + offset);
        }
    }
}
