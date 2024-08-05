use crate::util::instance::InstanceController;
use glam::Vec3;

/// A target entity on the world.
pub struct Target {
    instance_controller: InstanceController,
}

impl Target {
    /// A constructor.
    pub fn new(position: Vec3) -> Self {
        Self {
            instance_controller: InstanceController {
                position,
                scale: Vec3::new(0.2, 0.2, 0.2),
                ..Default::default()
            },
        }
    }

    /// A method to get the `InstanceController` of this.
    pub fn get_instance_controller(&mut self) -> InstanceController {
        self.instance_controller.clone()
    }

    /// A method to check if a ray goes through the target.
    pub fn check_shot(&self, position: Vec3, direction: Vec3) -> bool {
        let r = position - self.instance_controller.position;
        let d = r - r.dot(direction) * direction;
        d.length() <= 0.2
    }
}
