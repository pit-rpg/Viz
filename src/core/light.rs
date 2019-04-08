extern crate uuid;
extern crate specs;

use self::uuid::Uuid;

use math::{
	Vector,
	Vector3,
};

use self::specs::{
	Component,
	VecStorage
};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PointLight
{
	pub uuid: Uuid,
	pub color: Vector3<f32>,
	pub distance: f32,
	pub decay: f32,
	pub power: f32,
}

impl Default for PointLight {
	fn default() -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color: Vector3::new_one(),
			distance: 10.0,
			decay: 1.0,
			power: 1.0,
		}
	}
}


impl PointLight {
	pub fn new(color: Vector3<f32>, power: f32, distance: f32, decay: f32) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color,
			distance,
			decay,
			power,
		}
	}
}


impl Component for PointLight{
	type Storage = VecStorage<Self>;
}


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DirectionalLight
{
	pub uuid: Uuid,
	pub color: Vector3<f32>,
	pub direction: Vector3<f32>,
	pub power: f32,
}



impl Default for DirectionalLight {
	fn default() -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color: Vector3::new_one(),
			direction: Vector3::new(0.0, -1.0, 0.0),
			power: 1.0,
		}
	}
}


impl DirectionalLight {
	pub fn new(color: Vector3<f32>, direction: Vector3<f32>, power: f32) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color: color.clone(),
			direction,
			power,
		}
	}
}


impl Component for DirectionalLight {
	type Storage = VecStorage<Self>;
}



// #[allow(dead_code)]
// #[derive(Clone)]
// pub struct AmbientLight<T>
// where T: Nums
// {
// 	pub uuid: Uuid,
// 	pub color: Vector3<T>,
// 	pub power: T,
// }


