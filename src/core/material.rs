extern crate uuid;
use self::uuid::Uuid;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::Texture;
use super::TextureDimensions;
use math::*;
use std::sync::{Arc, Mutex, MutexGuard, LockResult};


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
	pub uniform: Uniform,
}

#[derive(Debug, Clone)]
pub struct TextureItem {
	pub name: String,
	pub texture: Option<Arc<Mutex<Texture>>>,
	pub dimensions: TextureDimensions,
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
	textures: Vec<TextureItem>,
	uniforms: Vec<UniformItem>,
}

#[allow(dead_code)]
impl Material {
	pub fn new(src: &str, name: &str, new_uniforms: &[UniformItem], new_textures: &[TextureItem]) -> Self {
		let uniforms = new_uniforms.iter().map(|u| u.clone()).collect();
		let textures = new_textures.iter().map(|u| u.clone()).collect();

		Self {
			name: name.to_string(),
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			textures,
			uniforms,
		}
	}

	pub fn set_uniform(&mut self, name: &str, u: &Uniform) -> Option<()> {
		let res = self.uniforms.iter_mut().find(|e| *e.name == *name);

		match res {
			None => return None,
			Some(uniform_item) => {
				match (&mut uniform_item.uniform, u) {
					(Uniform::Vector2(ref mut a), Uniform::Vector2(b)) 		=> { a.copy(&b); }
					(Uniform::Vector3(ref mut a), Uniform::Vector3(b)) 		=> { a.copy(&b); }
					(Uniform::Vector4(ref mut a), Uniform::Vector4(b)) 		=> { a.copy(&b); }
					(Uniform::Matrix3f(ref mut a), Uniform::Matrix3f(b)) 	=> { a.copy(&b); }
					(Uniform::Matrix4f(ref mut a), Uniform::Matrix4f(b)) 	=> { a.copy(&b); }
					(Uniform::Float(ref mut a), Uniform::Float(b)) 			=> { *a = *b; }
					(Uniform::Int(ref mut a), Uniform::Int(b)) 				=> { *a = *b; }
					(Uniform::UInt(ref mut a), Uniform::UInt(b)) 			=> { *a = *b; }
					_ => return None,
				};
			}
		}
		Some(())
	}

	pub fn get_src(&self) -> &str {
		&self.src[..]
	}

	pub fn set_texture( &mut self, name: &str, t: Option<Arc<Mutex<Texture>>> ) -> Option<()> {
		let texture = self.textures.iter_mut().find(|e| e.name == name);

		match texture {
			Some(texture) => {
				match t {
					Some(ref data) => {
						let d = data.lock().unwrap();
						if d.dimensions != texture.dimensions {return None;}
					}
					None => {}
				}

				texture.texture = t;
				Some(())
			}
			None => {None}
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
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
				},
			],
			&[]
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
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
				},
			],
			&[
				TextureItem {
					name: "texture_color".to_string(),
					texture: None,
					dimensions: TextureDimensions::D2,
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
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					uniform: Uniform::Matrix3f(Matrix3::new()),
				},
			],
			&[]
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
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					uniform: Uniform::Matrix3f(Matrix3::new()),
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
				},
				UniformItem {
					name: "color_light".to_string(),
					uniform: Uniform::Vector3(color_light.clone()),
				},
				UniformItem {
					name: "position_light".to_string(),
					uniform: Uniform::Vector3(position_light.clone()),
				},
			],
			&[]
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
				},
				UniformItem {
					name: "matrix_view".to_string(),
					uniform: Uniform::Matrix4f(Matrix4::new()),
				},
				UniformItem {
					name: "matrix_normal".to_string(),
					uniform: Uniform::Matrix3f(Matrix3::new()),
				},
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
				},
				UniformItem {
					name: "color_light".to_string(),
					uniform: Uniform::Vector3(color_light.clone()),
				},
				UniformItem {
					name: "position_light".to_string(),
					uniform: Uniform::Vector3(position_light.clone()),
				},
			],
			&[
				TextureItem {
					name: "texture_specular".to_string(),
					texture: None,
					dimensions: TextureDimensions::D2,
				},
				TextureItem {
					name: "texture_color".to_string(),
					texture: None,
					dimensions: TextureDimensions::D2,
				}
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
