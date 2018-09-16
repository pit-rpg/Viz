extern crate uuid;
use self::uuid::Uuid;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::Texture;
use math::*;
use std::sync::{Arc, Mutex};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Uniform {
	Vector4(Vector4<f32>),
	Vector3(Vector3<f32>),
	Vector2(Vector2<f32>),
	Matrix4f(Matrix4<f32>),
	Matrix3f(Matrix3<f32>),
	Float(f32),
	Int(i32),
	UInt(u32),
}

#[derive(Debug, Clone)]
pub struct UniformItem {
	pub name: String,
	pub program_type: ProgramType,
	pub need_update: bool,
	pub uniform: Uniform,
}

#[derive(Debug, Clone)]
pub struct TextureItem {
	pub name: String,
	pub program_type: ProgramType,
	pub texture: Arc<Mutex<Texture>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgramType {
	None,
	Vertex,
	Fragment,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Material {
	pub name: String,
	pub uuid: Uuid,
	src: String,
	textures: Vec<TextureItem>,
	uniforms: Vec<UniformItem>,
	pub uniform_need_update: bool,
}

#[allow(dead_code)]
impl Material {
	pub fn new(src: &str, name: &str, new_uniforms: &[UniformItem]) -> Self {
		let uniforms = new_uniforms.iter().map(|u| u.clone()).collect();

		Self {
			name: name.to_string(),
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			textures: Vec::new(),
			uniforms,
			uniform_need_update: true,
		}
	}

	pub fn set_uniform(&mut self, name: &str, u: &Uniform) -> Option<()> {
		let res = self.uniforms.iter_mut().find(|e| *e.name == *name);

		match res {
			None => return None,
			Some(uniform_item) => {
				match (&mut uniform_item.uniform, u) {
					(Uniform::Vector2(ref mut a), Uniform::Vector2(b)) => {
						a.copy(&b);
						uniform_item.need_update = true;
					}
					(Uniform::Vector3(ref mut a), Uniform::Vector3(b)) => {
						a.copy(&b);
						uniform_item.need_update = true;
					}
					(Uniform::Vector4(ref mut a), Uniform::Vector4(b)) => {
						a.copy(&b);
						uniform_item.need_update = true;
					}
					(Uniform::Matrix3f(ref mut a), Uniform::Matrix3f(b)) => {
						a.copy(&b);
						uniform_item.need_update = true;
					}
					(Uniform::Matrix4f(ref mut a), Uniform::Matrix4f(b)) => {
						a.copy(&b);
						uniform_item.need_update = true;
					}
					(Uniform::Float(ref mut a), Uniform::Float(b)) => {
						*a = *b;
						uniform_item.need_update = true;
					}
					(Uniform::Int(ref mut a), Uniform::Int(b)) => {
						*a = *b;
						uniform_item.need_update = true;
					}
					(Uniform::UInt(ref mut a), Uniform::UInt(b)) => {
						*a = *b;
						uniform_item.need_update = true;
					}
					_ => return None,
				};
			}
		}
		self.uniform_need_update = true;
		Some(())
	}

	pub fn get_src(&self) -> &str {
		&self.src[..]
	}

	pub fn set_texture(
		&mut self,
		name: &str,
		t: Option<Arc<Mutex<Texture>>>,
		program_type: ProgramType,
	) {
		match t {
			Some(t) => {
				{
					let texture = self.textures.iter_mut().find(|e| e.name == name);

					if texture.is_some() {
						let texture = texture.unwrap();
						texture.texture = t;
						texture.program_type = program_type;
						return;
					}
				}

				self.textures.push(TextureItem {
					name: name.to_string(),
					texture: t,
					program_type,
				});
			}

			None => {
				let textures = self.textures.drain(..).filter(|e| e.name != name).collect();
				self.textures = textures;
			}
		}
	}

	pub fn get_textures(&self) -> &[TextureItem] {
		&self.textures[..]
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
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_view".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "color".to_string(),
					program_type: ProgramType::Fragment,
					need_update: true,
					uniform: Uniform::Vector4(color.clone()),
				},
			],
		)
	}

	pub fn new_basic_texture(color: &Vector4<f32>) -> Self {
		Material::new(
			"basic-texture.glsl",
			"Basic-Texture",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_view".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "color".to_string(),
					program_type: ProgramType::Fragment,
					need_update: true,
					uniform: Uniform::Vector4(color.clone()),
				},
			],
		)
	}

	pub fn new_normal() -> Self {
		Material::new(
			"normal.glsl",
			"Normal",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_view".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix3f(Matrix3::new()),
				},
			],
		)
	}

	pub fn new_light(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		Material::new(
			"light.glsl",
			"Light",
			&[
				UniformItem {
					name: "matrix_model".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_view".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Matrix3f(Matrix3::new()),
				},
				UniformItem {
					name: "color".to_string(),
					program_type: ProgramType::Fragment,
					need_update: true,
					uniform: Uniform::Vector4(color.clone()),
				},
				UniformItem {
					name: "color_light".to_string(),
					program_type: ProgramType::Fragment,
					need_update: true,
					uniform: Uniform::Vector3(color_light.clone()),
				},
				UniformItem {
					name: "position_light".to_string(),
					program_type: ProgramType::Vertex,
					need_update: true,
					uniform: Uniform::Vector3(position_light.clone()),
				},
			],
		)
	}
}

impl Component for Material {
	type Storage = VecStorage<Self>;
}
