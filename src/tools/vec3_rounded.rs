use bevy::prelude::Vec3;

const SCALE: f32 = 1000.0; //num of decimal places

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Vec3Rounded {
    x: i32,
    y: i32,
    z: i32,
}

impl From<[f32; 3]> for Vec3Rounded {
    fn from(value: [f32; 3]) -> Self {
        Self { 
            x: (value[0] * SCALE).round() as i32,
            y: (value[1] * SCALE).round() as i32,
            z: (value[2] * SCALE).round() as i32, 
        }
    }
}

impl Vec3Rounded {
    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.x as f32 / SCALE,
            self.y as f32 / SCALE,
            self.z as f32 / SCALE,
        )
    }
}