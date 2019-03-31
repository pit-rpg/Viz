extern crate uuid;
use self::uuid::Uuid;

extern crate specs;

use std::collections::HashSet;

use super::SharedTexture2D;

use math::{Matrix3, Matrix4, Vector, Vector2, Vector3, Vector4};

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
	Texture2D(Option<SharedTexture2D>, u32),
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ShaderTag {
	Lighting,
	B_Color_4,
	B_Color_3,
	B_Position,
	B_UV,
	B_Normal,
	E_Map_Defuse,
	E_Map_Normal,
	E_Map_Roughness,
	E_Map_Metalness,
	E_Map_Emissive,
	Other(String),
}

#[derive(Debug, Clone)]
pub struct UniformItem {
	pub name: String,
	pub uniform: Uniform,
	pub need_update: bool,
}

#[allow(dead_code)]
pub trait ShaderProgram {

	fn set_uniform(&mut self, name: &str, u: &Uniform) {
		let uniforms = self.get_uniforms_mut();
		let mut material_need_update = false;
		let mut updated = false;
		{
			let res = uniforms.iter_mut().find(|e| e.name == name);
			if let Some(uniform_item) = res {
				match (&mut uniform_item.uniform, u) {
					(Uniform::Vector2(ref mut a), Uniform::Vector2(b)) => {
						if !a.equals(b) {
							a.copy(&b);
							uniform_item.need_update = true;
							updated = true;
						}
					}
					(Uniform::Vector3(ref mut a), Uniform::Vector3(b)) => {
						if !a.equals(b) {
							a.copy(&b);
							uniform_item.need_update = true;
							updated = true;
						}
					}
					(Uniform::Vector4(ref mut a), Uniform::Vector4(b)) => {
						if !a.equals(b) {
							a.copy(&b);
							uniform_item.need_update = true;
							updated = true;
						}
					}
					(Uniform::Matrix3f(ref mut a), Uniform::Matrix3f(b)) => {
						if !a.equals(b) {
							a.copy(&b);
							uniform_item.need_update = true;
							updated = true;
						}
					}
					(Uniform::Matrix4f(ref mut a), Uniform::Matrix4f(b)) => {
						if !a.equals(b) {
							a.copy(&b);
							uniform_item.need_update = true;
							updated = true;
						}
					}
					(Uniform::Float(ref mut a), Uniform::Float(b)) => {
						if a != b {
							*a = *b;
							uniform_item.need_update = true;
							updated = true;
						}
					}
					(Uniform::Int(ref mut a), Uniform::Int(b)) => {
						if a != b {
							*a = *b;
							uniform_item.need_update = true;
							updated = true;
						}
					}
					(Uniform::UInt(ref mut a), Uniform::UInt(b)) => {
						if a != b {
							*a = *b;
							uniform_item.need_update = true;
							updated = true;
						}
					}
					(Uniform::Texture2D(ref mut a, n1), Uniform::Texture2D(b, n2)) => {
						match (&a, &b) {
							(None, None) => {},
							(Some(t1), Some(t2)) => {
								if t1.get_uuid() != t1.get_uuid() {
									uniform_item.need_update = true;
									updated = true;
								}
							},
							(_, _) => {
								uniform_item.need_update = true;
								updated = true;
							}
						}

						match b {
							None => *a = None,
							Some(t) => *a = Some(t.clone()),
						}

						if n1 != n2 {
							material_need_update = true;
						}
					}

					_ => {}
				};

				return;
			}
		}

		let new_uniform = UniformItem {
			name: name.to_string(),
			uniform: u.clone(),
			need_update: true,
		};

		uniforms.push(new_uniform);
	}

	fn get_src(&self) -> &str;
	fn get_uniforms(&self) -> &Vec<UniformItem>;
	fn get_uniforms_mut(&mut self) -> &mut Vec<UniformItem>;
	fn get_uniforms_slice_mut(&mut self) -> &mut [UniformItem];
	fn get_tags(&self) -> &HashSet<ShaderTag>;
	fn get_tags_mut(&mut self) -> &mut HashSet<ShaderTag>;
	fn get_uuid(&self) -> Uuid;
	fn is_need_update(&self) -> bool;
	fn set_need_update(&mut self, bool);
}
