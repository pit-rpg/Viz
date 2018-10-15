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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgramType {
	None,
	Vertex,
	Fragment,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Material {
	pub name: String,
	pub uuid: Uuid,
	src: String,
	uniforms: Vec<UniformItem>,
}

#[allow(dead_code)]
impl Material {
	pub fn new(src: &str, name: &str, new_uniforms: &[UniformItem]) -> Self {
		let uniforms = new_uniforms.iter().map(|u| u.clone()).collect();

		Self {
			name: name.to_string(),
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			uniforms,
		}
	}

	pub fn set_uniform(&mut self, name: &str, u: &Uniform) -> Option<()> {
		let res = self.uniforms.iter_mut().find(|e| *e.name == *name);

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

					// (Uniform::ArrVector2(ref mut a), Uniform::ArrVector2(b)) => {

					// 	*a = *b;

					// }

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

	pub fn get_src(&self) -> &str {
		&self.src[..]
	}

	pub fn get_uniforms(&mut self) -> &mut [UniformItem] {
		&mut self.uniforms[..]
	}

	pub fn new_basic(color: &Vector4<f32>) -> Self {
		Material::new(
			"basic.glsl",
			"Basic",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
					need_update: true,
				},
			]
		)
	}

	pub fn new_basic_texture(color: &Vector4<f32>) -> Self {
		Material::new(
			"basic-texture.glsl",
			"Basic-Texture",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
					need_update: true,
				},
				UniformItem {
					name: "texture_color".to_string(),
					uniform: Uniform::Texture2D(None),
					need_update: true,
				}
			]
		)
	}

	pub fn new_normal() -> Self {
		Material::new(
			"normal.glsl",
			"Normal",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					uniform: Uniform::Matrix3f(Matrix3::new()),
					need_update: true,
				},
			]
		)
	}

	pub fn new_light(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		Material::new(
			"light.glsl",
			"Light",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					uniform: Uniform::Matrix3f(Matrix3::new()),
					need_update: true,
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
					need_update: true,
				},
				UniformItem {
					name: "color_light".to_string(),
					uniform: Uniform::Vector3(color_light.clone()),
					need_update: true,
				},
				UniformItem {
					name: "position_light".to_string(),
					uniform: Uniform::Vector3(position_light.clone()),
					need_update: true,
				},
			]
		)
	}


	pub fn new_light_texture(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		Material::new(
			"light_texture.glsl",
			"LightTexture",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					uniform: Uniform::Matrix3f(Matrix3::new()),
					need_update: true,
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
					need_update: true,
				},
				UniformItem {
					name: "color_light".to_string(),
					uniform: Uniform::Vector3(color_light.clone()),
					need_update: true,
				},
				UniformItem {
					name: "position_light".to_string(),
					uniform: Uniform::Vector3(position_light.clone()),
					need_update: true,
				},
				UniformItem {
					name: "texture_specular".to_string(),
					uniform: Uniform::Texture2D(None),
					need_update: true,
				},
				UniformItem {
					name: "texture_color".to_string(),
					uniform: Uniform::Texture2D(None),
					need_update: true,
				}
			]
		)
	}


	pub fn new_phong(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		Material::new(
			"phong.glsl",
			"Phong",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
					need_update: true,
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					uniform: Uniform::Matrix3f(Matrix3::new()),
					need_update: true,
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
					need_update: true,
				},
				UniformItem {
					name: "color_light".to_string(),
					uniform: Uniform::Vector3(color_light.clone()),
					need_update: true,
				},
				UniformItem {
					name: "position_light".to_string(),
					uniform: Uniform::Vector3(position_light.clone()),
					need_update: true,
				},

				UniformItem {
					name: "colors".to_string(),
					uniform: Uniform::ArrVector3(vec!(Vector3::new(0.0,1.0,0.0), Vector3::new(0.0,0.0,1.0))),
					need_update: true,
				},
			]
		)
	}

}

#[derive(Debug, Clone)]
pub struct SharedMaterial (Arc<Mutex<Material>>);

impl Component for SharedMaterial{
	type Storage = VecStorage<Self>;
}

impl SharedMaterial {
	pub fn new(m: Material) -> Self {
		SharedMaterial(Arc::new(Mutex::new(m)))
	}

	pub fn lock(&mut self) -> LockResult<MutexGuard<Material>> {
		self.0.lock()
	}
}
