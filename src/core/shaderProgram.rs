extern crate uuid;
use self::uuid::Uuid;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::{Texture2D, SharedTexture2D};
use math::*;
use std::sync::{Arc, Mutex, MutexGuard, LockResult};


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Uniform {
	Vector2(Vector2<f32>),
	Vector3(Vector3<f32>),
	Vector4(Vector4<f32>),
	Matrix4f(Matrix4<f32>),
	Matrix3f(Matrix3<f32>),
	Float(f32),
	Int(i32),
	UInt(u32),

	ArrVector2(Vec<Vector2<f32>>),
	ArrVector3(Vec<Vector3<f32>>),
	ArrVector4(Vec<Vector4<f32>>),
	ArrMatrix4f(Vec<Matrix4<f32>>),
	ArrMatrix3f(Vec<Matrix3<f32>>),
	ArrFloat(Vec<f32>),
	ArrInt(Vec<i32>),
	ArrUInt(Vec<u32>),

	Texture2D(Option<SharedTexture2D>),
}


#[derive(Debug, Clone)]
pub struct UniformItem {
	pub name: String,
	pub uniform: Uniform,
	pub need_update: bool
}


pub fn set_uniform(uniforms: &mut [UniformItem], name: &str, u: &Uniform) -> Option<()> {
	let res = uniforms.iter_mut().find(|e| *e.name == *name);

	match res {
		None => return None,
		Some(uniform_item) => {
			match (&mut uniform_item.uniform, u) {
				(Uniform::Vector2(ref mut a), Uniform::Vector2(b)) 		=> {
					if !a.equals(b) {
						a.copy(&b);
						uniform_item.need_update = true;
					}
				}
				(Uniform::Vector3(ref mut a), Uniform::Vector3(b)) 		=> {
					if !a.equals(b) {
						a.copy(&b);
						uniform_item.need_update = true;
					}
				}
				(Uniform::Vector4(ref mut a), Uniform::Vector4(b)) 		=> {
					if !a.equals(b) {
						a.copy(&b);
						uniform_item.need_update = true;
					}
				}
				(Uniform::Matrix3f(ref mut a), Uniform::Matrix3f(b)) 	=> {
					if !a.equals(b) {
						a.copy(&b);
						uniform_item.need_update = true;
					}
				}
				(Uniform::Matrix4f(ref mut a), Uniform::Matrix4f(b)) 	=> {
					if !a.equals(b) {
						a.copy(&b);
						uniform_item.need_update = true;
					}
				}
				(Uniform::Float(ref mut a), Uniform::Float(b)) 			=> {
					if a != b {
						*a = *b;
						uniform_item.need_update = true;
					}
				}
				(Uniform::Int(ref mut a), Uniform::Int(b)) 				=> {
					if a != b {
						*a = *b;
						uniform_item.need_update = true;
					}
				}
				(Uniform::UInt(ref mut a), Uniform::UInt(b)) 			=> {
					if a != b {
						*a = *b;
						uniform_item.need_update = true;
					}
				}

				(Uniform::ArrVector2(ref mut a), Uniform::ArrVector2(b)) => {
					if a.len() != b.len() {return None}
					let mut need_update = false;

					for (v_a, v_b) in a.iter_mut().zip(b.iter()) {
						if !v_a.equals(v_b) {
							v_a.copy(&v_b);
							need_update = true;
						}
					}
					uniform_item.need_update = need_update;
				}

				(Uniform::ArrVector3(ref mut a), Uniform::ArrVector3(b)) => {
					if a.len() != b.len() {return None}
					let mut need_update = false;

					for (v_a, v_b) in a.iter_mut().zip(b.iter()) {
						if !v_a.equals(v_b) {
							v_a.copy(&v_b);
							need_update = true;
						}
					}
					uniform_item.need_update = need_update;
				}

				(Uniform::ArrVector4(ref mut a), Uniform::ArrVector4(b)) => {
					if a.len() != b.len() {return None}
					let mut need_update = false;

					for (v_a, v_b) in a.iter_mut().zip(b.iter()) {
						if !v_a.equals(v_b) {
							v_a.copy(&v_b);
							need_update = true;
						}
					}
					uniform_item.need_update = need_update;
				}

				(Uniform::ArrMatrix3f(ref mut a), Uniform::ArrMatrix3f(b)) => {
					if a.len() != b.len() {return None}
					let mut need_update = false;

					for (v_a, v_b) in a.iter_mut().zip(b.iter()) {
						if !v_a.equals(v_b) {
							v_a.copy(&v_b);
							need_update = true;
						}
					}
					uniform_item.need_update = need_update;
				}

				(Uniform::ArrMatrix4f(ref mut a), Uniform::ArrMatrix4f(b)) => {
					if a.len() != b.len() {return None}
					let mut need_update = false;

					for (v_a, v_b) in a.iter_mut().zip(b.iter()) {
						if !v_a.equals(v_b) {
							v_a.copy(&v_b);
							need_update = true;
						}
					}
					uniform_item.need_update = need_update;
				}

				(Uniform::ArrFloat(ref mut a), Uniform::ArrFloat(b)) => {
					if a.len() != b.len() {return None}
					let mut need_update = false;

					for (v_a, v_b) in a.iter_mut().zip(b.iter()) {
						if v_a != v_b {
							*v_a = *v_b;
							need_update = true;
						}
					}
					uniform_item.need_update = need_update;
				}

				(Uniform::ArrInt(ref mut a), Uniform::ArrInt(b)) => {
					if a.len() != b.len() {return None}
					let mut need_update = false;

					for (v_a, v_b) in a.iter_mut().zip(b.iter()) {
						if v_a != v_b {
							*v_a = *v_b;
							need_update = true;
						}
					}
					uniform_item.need_update = need_update;
				}

				(Uniform::ArrUInt(ref mut a), Uniform::ArrUInt(b)) => {
					if a.len() != b.len() {return None}
					let mut need_update = false;

					for (v_a, v_b) in a.iter_mut().zip(b.iter()) {
						if v_a != v_b {
							*v_a = *v_b;
							need_update = true;
						}
					}
					uniform_item.need_update = need_update;
				}

				(Uniform::Texture2D(ref mut a), Uniform::Texture2D(b)) 	=> {
					match b {
						None => {*a = None}
						Some(t) => {*a = Some(t.clone())}
					}
					uniform_item.need_update = true;
				}

				_ => return None,
			};
		}
	}
	Some(())
}


#[allow(dead_code)]
pub trait ShaderProgram {
	fn set_uniform(&mut self, name: &str, u: &Uniform) -> Option<()> {
		set_uniform(&mut self.get_uniforms(), name, u)
	}

	fn get_src(&self) -> &str;
	fn get_uniforms(&mut self) -> &mut [UniformItem];
}

