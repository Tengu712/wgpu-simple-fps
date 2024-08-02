use glam::{EulerRot, Quat, Vec3};

#[derive(Clone)]
pub struct CameraController {
    pub pov: f32,
    pub width: f32,
    pub height: f32,
    pub position: Vec3,
    pub rotation: Quat,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            pov: 45.0f32.to_radians(),
            width: 1.0,
            height: 1.0,
            position: Vec3::default(),
            rotation: Quat::default(),
        }
    }
}

impl CameraController {
    /// A method to align a vector to the camera's x-z plane direction.
    pub fn align_to_direction(&self, v: Vec3) -> Vec3 {
        Quat::from_axis_angle(
            Vec3::new(0.0, 1.0, 0.0),
            self.rotation.to_euler(EulerRot::YXZ).0,
        )
        .mul_vec3(v)
    }

    /// A method to rotate the camera.
    ///
    /// It rotates the camera in the following order:
    /// 1. Y-axis around
    /// 2. X-axis around
    pub fn rotate(&mut self, y: f32, x: f32) {
        let angle = self.rotation.to_euler(EulerRot::YXZ);
        let y = angle.0 + y;
        let x = angle.1 + x;
        let x = if x < -90.0f32.to_radians() {
            -90f32.to_radians()
        } else if 90.0f32.to_radians() < x {
            90f32.to_radians()
        } else {
            x
        };
        self.rotation = Quat::from_euler(EulerRot::YXZ, y, x, 0.0);
    }
}
