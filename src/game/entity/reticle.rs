use crate::util::{cache::Cache, instance::InstanceController};
use glam::{Vec3, Vec4};

/// A reticle entity.
pub struct Reticle {
    instance_controller: Cache<InstanceController>,
}

impl Reticle {
    /// A constructor.
    pub fn new() -> Self {
        Self {
            instance_controller: Cache::new(InstanceController {
                scale: Vec3::new(200.0, 200.0, 1.0),
                uv: Vec4::new(0.0, 0.75, 0.25, 0.25),
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
