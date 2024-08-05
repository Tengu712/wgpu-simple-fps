use crate::util::{cache::Cache, instance::InstanceController};
use glam::{Vec3, Vec4};

fn new(x: f32, y: f32, width: f32, number: u32) -> Vec<Cache<InstanceController>> {
    let mut number = number;
    let mut digits = Vec::new();
    if number > 0 {
        while number != 0 {
            digits.push(number % 10);
            number /= 10;
        }
    } else {
        digits.push(0);
    }

    let mut result = Vec::new();
    let mut x = x - width / 2.0;
    for n in digits {
        let uv = Vec4::new(n as f32 * 0.1, 0.0, 0.1, 0.125);
        let height = width * 0.125 / uv.z;
        result.push(Cache::new(InstanceController {
            scale: Vec3::new(width, height, 1.0),
            position: Vec3::new(x, y - height / 2.0, 0.0),
            uv,
            ..Default::default()
        }));
        x -= width;
    }
    result
}

/// A digits entity.
pub struct Digits {
    instance_controllers: Vec<Cache<InstanceController>>,
    x: f32,
    y: f32,
    width: f32,
}

impl Digits {
    /// A constructor.
    ///
    /// * `x` - the right of the digits
    /// * `y` - the top of the digits
    /// * `width` - the width of a character
    pub fn new(x: f32, y: f32, width: f32, number: u32) -> Self {
        Self {
            instance_controllers: new(x, y, width, number),
            x,
            y,
            width,
        }
    }

    /// A method to get the vector of `InstanceController` of this.
    ///
    /// WARN: If no update is needed, return `None`.
    pub fn get_instance_controllers(&mut self) -> Vec<Option<InstanceController>> {
        self.instance_controllers
            .iter_mut()
            .map(|n| n.cache())
            .collect()
    }

    /// A method to set number.
    pub fn set_number(&mut self, number: u32) {
        self.instance_controllers = new(self.x, self.y, self.width, number);
    }
}
