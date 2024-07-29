use glam::{Quat, Vec2, Vec3, Vec3Swizzles};

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
    /// A method to translate the camera.
    ///
    /// The rotation angle around the Y-axis of the camera is considered.
    pub fn translate(&mut self, v: Vec3) {
        let angle = self
            .rotation
            .mul_vec3(Vec3::new(0.0, 0.0, 1.0))
            .xz()
            .normalize()
            .angle_to(Vec2::new(0.0, 1.0));
        self.position += Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), angle).mul_vec3(v);
    }
}
