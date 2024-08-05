use crate::util::instance::InstanceController;
use glam::{Vec3, Vec4};

/// A message entity.
pub struct Message {
    instance_controller: InstanceController,
    is_updated: bool,
}

impl Message {
    /// A constructor.
    ///
    /// It automatically decides the height depends on `/image/ui.png`.
    pub fn new(x: f32, y: f32, width: f32, uv: Vec4) -> Self {
        Self {
            instance_controller: InstanceController {
                position: Vec3::new(x, y, 0.0),
                scale: Vec3::new(width, width * 0.125 / uv.z, 1.0),
                uv,
                ..Default::default()
            },
            is_updated: false,
        }
    }

    /// A method to get the `InstanceController` of this.
    ///
    /// WARN: When an update is necessary, update the update flag and return the `InstanceController`.
    ///       If no update is needed, return `None`.
    pub fn get_instance_controller(&mut self) -> Option<InstanceController> {
        if !self.is_updated {
            self.is_updated = true;
            Some(self.instance_controller.clone())
        } else {
            None
        }
    }
}
