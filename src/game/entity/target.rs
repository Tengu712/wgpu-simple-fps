use crate::util::instance::InstanceController;
use glam::Vec3;

/// A target entity on the world.
pub struct Target {
    instance_controller: InstanceController,
    update_function: Box<dyn FnMut(&mut Vec3, u32)>,
    count: u32,
}

impl Target {
    /// A constructor.
    pub fn new(position: Vec3, update_function: Box<dyn FnMut(&mut Vec3, u32)>) -> Self {
        Self {
            instance_controller: InstanceController {
                position,
                scale: Vec3::new(0.2, 0.2, 0.2),
                ..Default::default()
            },
            update_function,
            count: 0,
        }
    }

    /// A method to get the `InstanceController` of this.
    pub fn get_instance_controller(&mut self) -> InstanceController {
        self.instance_controller.clone()
    }

    pub fn update(&mut self) {
        (self.update_function)(&mut self.instance_controller.position, self.count);
        self.count += 1;
    }

    /// A method to check if a ray goes through the target.
    pub fn check_shot(&self, position: Vec3, direction: Vec3) -> bool {
        let r = position - self.instance_controller.position;
        let d = r - r.dot(direction) * direction;
        d.length() <= 0.2
    }
}
