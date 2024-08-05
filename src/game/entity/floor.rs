use crate::util::{cache::Cache, instance::InstanceController};
use glam::Vec3;

/// A floor entity on the world.
pub struct Floor {
    instance_controller: Cache<InstanceController>,
}

impl Floor {
    /// A constructor.
    pub fn new(width: f32, depth: f32) -> Self {
        Self {
            instance_controller: Cache::new(InstanceController {
                scale: Vec3::new(width, 1.0, depth),
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
