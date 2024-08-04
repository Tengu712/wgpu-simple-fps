use crate::util::instance::InstanceController;

/// A floor entity on the world.
pub struct Floor {
    instance_controller: InstanceController,
}

impl Floor {
    pub fn new(width: f32, depth: f32) -> Self {
        let mut instance_controller = InstanceController::default();
        instance_controller.scale.x = width;
        instance_controller.scale.z = depth;
        Self {
            instance_controller,
        }
    }

    pub fn get_instance_controller(&self) -> InstanceController {
        self.instance_controller.clone()
    }
}
