use glam::Vec3;

#[derive(Clone)]
pub struct CameraController {
    pub pov: f32,
    pub width: f32,
    pub height: f32,
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            pov: 45.0f32.to_radians(),
            width: 1.0,
            height: 1.0,
            position: Vec3::new(0.0, 0.0, 0.0),
            direction: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, -1.0, 0.0),
        }
    }
}
