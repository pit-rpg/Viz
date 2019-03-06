use math::{Euler, Matrix4, Quaternion, Vector, Vector3};

extern crate specs;
use self::specs::{Component, VecStorage};


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Transform {
	pub matrix_local: Matrix4<f32>,
	pub matrix_world: Matrix4<f32>,
	pub position: Vector3<f32>,
	pub scale: Vector3<f32>,
	pub rotation: Euler<f32>,
	pub quaternion: Quaternion<f32>,
}


#[allow(dead_code)]
impl Transform {
	pub fn update(&mut self) -> &mut Self {
		self.quaternion.set_from_euler(&self.rotation);
		self.matrix_local.compose(&self.position, &self.quaternion, &self.scale);
		self
	}

	pub fn look_at (&mut self, target: &Vector3<f32>, up: &Vector3<f32>) -> &mut Self {
		let mut matrix = Matrix4::new();
		// let scale_tmp = Vector3::new_one();
		// let pos_tmp = Vector3::new_zero();

		matrix.look_at(&self.position, target, up);
		self.quaternion.set_from_rotation_matrix(&matrix);
		self.matrix_local.compose(&self.position, &self.quaternion, &self.scale);
		// self.matrix_local.compose(&pos_tmp, &self.quaternion, &scale_tmp);
		self.rotation.set_from_quaternion(&self.quaternion);
		// self.update();
		self
	}


	pub fn from_matrix (matrix: Matrix4<f32>) -> Self {

		let (pos, rotQ, scale) = matrix.decompose_to_new();
		let rotE = Euler::from_quaternion(&rotQ);

		Self {
			matrix_local: matrix,
			matrix_world: Matrix4::new(),
			position: pos,
			scale: scale,
			rotation: rotE,
			quaternion: rotQ,
		}
	}
}


impl Default for Transform {
	fn default() -> Self {
		Self {
			matrix_local: Matrix4::new(),
			matrix_world: Matrix4::new(),
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
