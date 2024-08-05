use crate::util::{cache::Cache, instance::InstanceController};
use glam::{Vec3, Vec4};

/// A message entity.
pub struct Message {
    instance_controller: Cache<InstanceController>,
}

impl Message {
    /// A constructor.
    ///
    /// It automatically decides the height depends on `/image/ui.png`.
    pub fn new(x: f32, y: f32, width: f32, uv: Vec4) -> Self {
        Self {
            instance_controller: Cache::new(InstanceController {
                position: Vec3::new(x, y, 0.0),
                scale: Vec3::new(width, width * 0.125 / uv.z, 1.0),
                uv,
                ..Default::default()
            }),
        }
    }

    /// A method to get the `InstanceController` of this.
    ///
    /// WARN: If no update is needed, return `None`.
    pub fn get_instance_controller(&mut self) -> Option<InstanceController> {
        self.instance_controller.cache()
    }
}
