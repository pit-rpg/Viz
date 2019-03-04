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
}


impl Default for PointLight {
	fn default() -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color: Vector3::new_one(),
			distance: 10.0,
			decay: 1.0,
		}
	}
}


impl PointLight {
	pub fn new(color: &Vector3<f32>, distance: f32, decay: f32) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color: color.clone(),
			distance,
			decay,
		}
	}
}


impl Component for PointLight{
	type Storage = VecStorage<Self>;
}

// #[allow(dead_code)]
// #[derive(Clone)]
// pub struct DirectionalLight<T>
// where T: Nums
// {
// 	pub uuid: Uuid,
// 	pub color: Vector3<T>,
// 	pub power: T,
// }


// #[allow(dead_code)]
// #[derive(Clone)]
// pub struct AmbientLight<T>
// where T: Nums
// {
// 	pub uuid: Uuid,
// 	pub color: Vector3<T>,
// 	pub power: T,
// }


// impl <T> PointLight<T>
// where T:Nums
// {
// 	pub fn new (color: Vector3<T>, power: T) -> Self {
// 		Self {
// 			uuid: Uuid::new_v4(),
// 			color,
// 			power,
// 			constant: T::from_f32(1.0),
// 			linear: T::from_f32(0.7),
// 			quadratic: T::from_f32(1.8),
// 		}
// 	}
// }

// impl <T> AmbientLight<T>
// where T:Nums
// {
// 	pub fn new (color: Vector3<T>, power: T) -> Self {
// 		Self {
// 			uuid: Uuid::new_v4(),
// 			color,
// 			power,
// 		}
// 	}
// }


// #[allow(dead_code)]
// impl <T> Light<T>
// where T: Nums
// {
// 	pub fn new_point(color: Vector3<T>, power: T) -> Self {
// 		Light::Point(PointLight::new(color, power))
// 	}

// 	pub fn new_ambient_light(color: Vector3<T>, power: T) -> Self {
// 		Light::Ambient(AmbientLight::new(color, power))
// 	}
// }