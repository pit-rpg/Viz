use helpers::Nums;
use math::{Euler, Matrix4, Quaternion, Vector3, Vector};

extern crate specs;
use self::specs::{Component, VecStorage};

#[allow(dead_code)]
pub struct Transform
// where
	// T: Nums,
{
	pub matrix_local: Matrix4<f32>,
	pub matrix_global: Matrix4<f32>,
	pub position: Vector3<f32>,
	pub scale: Vector3<f32>,
	pub rotation: Euler<f32>,
}



impl Default for Transform
// where
// 	T: Nums,
{
	fn default() -> Self {
		Self {
			matrix_local: Matrix4::new(),
			matrix_global: Matrix4::new(),
			position: Vector3::zero(),
			scale: Vector3::zero(),
			rotation: Euler::default(),
		}
	}
}


impl Component for Transform
// where
// 	T: Nums+'static, Self: Sized,
{
	type Storage = VecStorage<Self>;
}
