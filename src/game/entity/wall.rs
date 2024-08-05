use crate::util::{cache::Cache, instance::InstanceController};
use glam::{Quat, Vec2, Vec3, Vec3Swizzles};

fn cross(v1: Vec2, v2: Vec2) -> f32 {
    v1.x * v2.y - v1.y * v2.x
}
fn is_left(v1: Vec2, v2: Vec2) -> bool {
    cross(v1, v2) > 0.0
}
fn get_intersection(p11: Vec2, p12: Vec2, p21: Vec2, p22: Vec2) -> Option<Vec2> {
    let v1 = p12 - p11;
    let v2 = p22 - p21;
    let c = cross(v1, v2);
    if c.abs() <= f32::EPSILON {
        return None;
    }
    let s = cross(p21 - p11, v2) / c;
    let t = cross(v1, p11 - p21) / c;
    if s < 0.0 || s > 1.0 || t < 0.0 || t > 1.0 {
        return None;
    }
    Some(p11 + s * v1)
}

/// A wall entity on the world.
pub struct Wall {
    instance_controller: Cache<InstanceController>,
    vertices: [Vec2; 5],
    edges: [Vec2; 4],
}

impl Wall {
    /// A constructor.
    ///
    /// * `position` - the position of the center point
    /// * `rotation` - the rotation angle (rad) around y-axis
    /// * `scale` - the scale in the x-axis direction
    pub fn new(position: Vec3, rotation: f32, scale: Vec3) -> Self {
        let rotation = Quat::from_rotation_y(rotation);
        let hw = (scale.x / 2.0) + 1.0;
        let hd = (scale.z / 2.0) + 1.0;
        let a = rotation.mul_vec3(Vec3::new(-hw, 0.0, -hd)) + position;
        let b = rotation.mul_vec3(Vec3::new(hw, 0.0, -hd)) + position;
        let c = rotation.mul_vec3(Vec3::new(hw, 0.0, hd)) + position;
        let d = rotation.mul_vec3(Vec3::new(-hw, 0.0, hd)) + position;
        let vertices = [a.xz(), b.xz(), c.xz(), d.xz(), a.xz()];
        let edges = [
            b.xz() - a.xz(),
            c.xz() - b.xz(),
            d.xz() - c.xz(),
            a.xz() - d.xz(),
        ];

        Self {
            instance_controller: Cache::new(InstanceController {
                position,
                scale,
                rotation,
                ..Default::default()
            }),
            vertices,
            edges,
        }
    }

    /// A method to get the `InstanceController` of the wall entity.
    ///
    /// WARN: If no update is needed, return `None`.
    pub fn get_instance_controller(&mut self) -> Option<InstanceController> {
        self.instance_controller.cache()
    }

    /// A method to check if the wall entity and `position + velocity` is collided.
    ///
    /// If a collision occurs, it returns new velocity.
    pub fn check_collision(&self, position: Vec3, velocity: Vec3) -> Vec3 {
        // calculate new position
        let position = position.xz();
        let new_position = position + velocity.xz();

        // check collision
        let is_out = (0..4)
            .into_iter()
            .any(|i| !is_left(self.edges[i], new_position - self.vertices[i]));
        if is_out {
            return velocity;
        }

        // get intersections
        let mut intersections = Vec::new();
        for i in 0..4 {
            let intersection = get_intersection(
                self.vertices[i],
                self.vertices[i + 1],
                position,
                new_position,
            );
            if let Some(n) = intersection {
                intersections.push((i, n));
            }
        }
        if intersections.is_empty() {
            return velocity;
        }

        // get the closest intersection index
        let mut index = 0;
        let mut min_distance = f32::MAX;
        for (i, n) in intersections {
            let distance = n.distance(position);
            if distance < min_distance {
                index = i;
                min_distance = distance;
            }
        }

        // project onto a normal vector
        let normal = Vec2::new(self.edges[index].y, -self.edges[index].x).normalize();
        let projected = velocity.project_onto(Vec3::new(normal.x, 0.0, normal.y));

        // return
        velocity - projected
    }
}
