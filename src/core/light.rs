use uuid::Uuid;

use math::{
	Vector,
	Vector3,
};

#[derive(Clone, Debug)]
pub enum LightType {
	Point,
	Directional,
}

#[derive(Clone, Debug)]
pub struct Light
{
	pub uuid: Uuid,
	pub color: Vector3<f32>,
	pub distance: f32,
	pub decay: f32,
	pub power: f32,
	pub light_type: LightType,
}

impl Default for Light {
	fn default() -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color: Vector3::new_one(),
			distance: 10.0,
			decay: 1.0,
			power: 1.0,
			light_type: LightType::Point,
		}
	}
}


impl Light {
	pub fn new(color: Vector3<f32>, power: f32) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color,
			power,
			distance: 10.0,
			decay: 1.0,
			light_type: LightType::Point,
		}
	}
}
