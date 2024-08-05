use glam::{Quat, Vec3, Vec4};

#[derive(Clone)]
pub struct InstanceController {
    pub scale: Vec3,
    pub rotation: Quat,
    pub position: Vec3,
    pub uv: Vec4,
}

impl Default for InstanceController {
    fn default() -> Self {
        Self {
            scale: Vec3::new(1.0, 1.0, 1.0),
            rotation: Quat::default(),
            position: Vec3::default(),
            uv: Vec4::new(0.0, 0.0, 1.0, 1.0),
        }
    }
}
