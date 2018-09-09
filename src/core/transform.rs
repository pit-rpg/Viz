use math::{Euler, Matrix4, Quaternion, Vector, Vector3};

extern crate specs;
use self::specs::{Component, VecStorage};

#[allow(dead_code)]
pub struct Transform {
	pub matrix_local: Matrix4<f32>,
	pub matrix_global: Matrix4<f32>,
	pub matrix_view: Matrix4<f32>,
	pub position: Vector3<f32>,
	pub scale: Vector3<f32>,
	pub rotation: Euler<f32>,
	pub quaternion: Quaternion<f32>,
}

impl Transform {
	pub fn update(&mut self) {
		self.quaternion.set_from_euler(&self.rotation);
		self.matrix_local
			.compose(&self.position, &self.quaternion, &self.scale);
		self.matrix_view = self.matrix_global * self.matrix_local;
	}
}

impl Default for Transform {
	fn default() -> Self {
		Self {
			matrix_local: Matrix4::new(),
			matrix_global: Matrix4::new(),
			matrix_view: Matrix4::new(),
			position: Vector3::zero(),
			scale: Vector3::new_one(),
			rotation: Euler::default(),
			quaternion: Quaternion::new(),
		}
	}
}

impl Component for Transform {
	type Storage = VecStorage<Self>;
}
