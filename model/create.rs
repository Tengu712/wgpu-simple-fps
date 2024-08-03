//! A program to create follwing models as Wavefront OBJ:
//! - sphere
//! - rectangle
//!
//! The 3d model has follwing parameters:
//! - geometric vertex
//! - face elements
//!   - index
//!   - texture coordinates
//!   - vertex normals

use std::{f32::consts, fs::File, io::Write};

/// A function to normalize a 3d vector.
///
/// WARN: The parameter `v` must not be a zero vector.
fn normalize(v: [f32; 3]) -> [f32; 3] {
    if v[0].abs() <= f32::EPSILON && v[1].abs() <= f32::EPSILON && v[2].abs() <= f32::EPSILON {
        panic!("tried to normalize a zero vector.");
    }
    let magnitude = (v[0].powf(2.0) + v[1].powf(2.0) + v[2].powf(2.0)).powf(0.5);
    [v[0] / magnitude, v[1] / magnitude, v[2] / magnitude]
}

/// A struct for the data of a single vertex.
struct Vertex {
    p: [f32; 3],
    n: [f32; 3],
    t: [f32; 2],
}

fn push_vertex_for_sphere(
    r: f32,
    latitude_count: usize,
    longtitude_count: usize,
    is_northern: bool,
    vertex_data: &mut Vec<Vertex>,
) {
    let half_latitude_count = latitude_count / 2;
    let (start, end, x_offset, y_offset) = if is_northern {
        (1, half_latitude_count + 1, 0.25, 0.5)
    } else {
        (half_latitude_count, latitude_count, 0.75, 0.5)
    };

    for lo in start..end {
        let theta_lo = consts::PI / latitude_count as f32 * lo as f32;
        let l = r * theta_lo.sin().abs();
        let y = r * theta_lo.cos();
        for la in 0..longtitude_count {
            let theta_la = 2.0 * consts::PI / longtitude_count as f32 * la as f32;
            let s = theta_la.sin();
            let c = theta_la.cos();
            let k = if is_northern { lo } else { latitude_count - lo };
            let k = 0.98 / half_latitude_count as f32 * k as f32;
            let position = [l * s, y, l * c];
            vertex_data.push(Vertex {
                p: position,
                n: normalize(position),
                t: [x_offset + s * 0.25 * k, y_offset + c * 0.5 * k],
            });
        }
    }
}

fn push_index_for_sphere(
    latitude_count: u16,
    longtitude_count: u16,
    is_northern: bool,
    index_data: &mut Vec<u16>,
) {
    let half_latitude_count = latitude_count / 2;
    let (start, end, offset) = if is_northern {
        (0, half_latitude_count - 1, 0)
    } else {
        (
            half_latitude_count - 1,
            latitude_count - 2,
            longtitude_count,
        )
    };

    for lo in start..end {
        for la in 1..(longtitude_count + 1) {
            let la1 = la;
            let la2 = (la % longtitude_count) + 1;
            let u1 = la1 + longtitude_count * lo;
            let u2 = la2 + longtitude_count * lo;
            let b1 = la1 + longtitude_count * (lo + 1);
            let b2 = la2 + longtitude_count * (lo + 1);
            index_data.push(u1 + offset);
            index_data.push(b1 + offset);
            index_data.push(b2 + offset);
            index_data.push(u1 + offset);
            index_data.push(b2 + offset);
            index_data.push(u2 + offset);
        }
    }
}

/// A function to create a sphere model.
///
/// * `r` - the radius of the sphere
/// * `latitude_count` - the number of divisions at this latitude
/// * `longtitude_count` - the number of divisions at this longtitude
///
/// When viewed from the outside, the vertices of all faces are connected in a clockwise order.
///
/// WARN: `latitude_count` must be even number.
fn create_sphere(
    r: f32,
    latitude_count: usize,
    longtitude_count: usize,
) -> (Vec<Vertex>, Vec<u16>) {
    if latitude_count % 2 == 1 {
        panic!("tried to divide a sphere into an even number of sections along the latitude.");
    }

    // vertex data
    let mut vertex_data = Vec::new();
    // 1. north pole
    vertex_data.push(Vertex {
        p: [0.0, r, 0.0],
        n: [0.0, 1.0, 0.0],
        t: [0.25, 0.5],
    });
    // 2. northern hemisphere
    push_vertex_for_sphere(r, latitude_count, longtitude_count, true, &mut vertex_data);
    // 3. southern hemisphere
    push_vertex_for_sphere(r, latitude_count, longtitude_count, false, &mut vertex_data);
    // 4. south pole
    vertex_data.push(Vertex {
        p: [0.0, -r, 0.0],
        n: [0.0, -1.0, 0.0],
        t: [0.75, 0.5],
    });

    // cast
    let latitude_count = latitude_count as u16;
    let longtitude_count = longtitude_count as u16;

    // index data
    let mut index_data = Vec::new();
    // 1. top
    for i in 1..(longtitude_count + 1) {
        index_data.push(0);
        index_data.push(i);
        index_data.push((i % longtitude_count) + 1);
    }
    // 2. northern hemisphere
    push_index_for_sphere(latitude_count, longtitude_count, true, &mut index_data);
    // 3. southern hemisphere
    push_index_for_sphere(latitude_count, longtitude_count, false, &mut index_data);
    // 3. bottom
    for i in 1..(longtitude_count + 1) {
        let offset = longtitude_count + (latitude_count - 2) * longtitude_count;
        index_data.push(i + offset);
        index_data.push(vertex_data.len() as u16 - 1);
        index_data.push((i % longtitude_count) + 1 + offset);
    }

    (vertex_data, index_data)
}

/// A function to create a rectangle model.
///
/// The vertices of this model are connected in a clockwise order
/// when viewed along the positive z-axis in the local coordinate system.
fn create_rectangle(width: f32, height: f32) -> (Vec<Vertex>, Vec<u16>) {
    let hw = width / 2.0;
    let hh = height / 2.0;
    let vertex_data = Vec::from([
        // bottom left
        Vertex {
            p: [-hw, -hh, 0.0],
            n: [0.0, 0.0, -1.0],
            t: [0.0, 1.0],
        },
        // top left
        Vertex {
            p: [-hw, hh, 0.0],
            n: [0.0, 0.0, -1.0],
            t: [0.0, 0.0],
        },
        // top right
        Vertex {
            p: [hw, hh, 0.0],
            n: [0.0, 0.0, -1.0],
            t: [1.0, 0.0],
        },
        // bottom right
        Vertex {
            p: [hw, -hh, 0.0],
            n: [0.0, 0.0, -1.0],
            t: [1.0, 1.0],
        },
    ]);
    let index_data = Vec::from([0, 1, 2, 0, 2, 3]);
    (vertex_data, index_data)
}

/// A function to create a Wavefront OBJ file.
fn create_file(name: &str, data: (Vec<Vertex>, Vec<u16>)) {
    let mut file = File::create(name).unwrap();
    let (vertex_data, index_data) = data;
    for n in vertex_data {
        file.write_all(format!("v {} {} {}\n", n.p[0], n.p[1], n.p[2]).as_bytes())
            .unwrap();
        file.write_all(format!("vt {} {}\n", n.t[0], n.t[1]).as_bytes())
            .unwrap();
        file.write_all(format!("vn {} {} {}\n", n.n[0], n.n[1], n.n[2]).as_bytes())
            .unwrap();
    }
    for n in index_data.chunks(3) {
        file.write_all(
            format!(
                "f {0}/{0}/{0} {1}/{1}/{1} {2}/{2}/{2}\n",
                n[0] + 1,
                n[1] + 1,
                n[2] + 1
            )
            .as_bytes(),
        )
        .unwrap();
    }
}

fn main() {
    create_file("square.obj", create_rectangle(1.0, 1.0));
    create_file("sphere.obj", create_sphere(50.0, 20, 20));
}
