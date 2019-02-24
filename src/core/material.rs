extern crate uuid;
use self::uuid::Uuid;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::{Texture2D, SharedTexture2D, UniformItem, Uniform, ShaderProgram};
use math::*;
use std::sync::{Arc, Mutex, MutexGuard, LockResult};



#[allow(dead_code)]
#[derive(Debug)]
pub struct Material {
	pub uuid: Uuid,
	src: String,
	uniforms: Vec<UniformItem>,
}


impl ShaderProgram for Material {
	fn get_src(&self) -> &str {
		&self.src[..]
	}

	fn get_uniforms(&mut self) -> &Vec<UniformItem> {
		&mut self.uniforms
	}

	fn get_uniforms_mut(&mut self) -> &mut Vec<UniformItem> {
		&mut self.uniforms
	}
	
	fn get_uniforms_slice_mut(&mut self) -> &mut [UniformItem] {
		&mut self.uniforms
	}
}


#[allow(dead_code)]
impl Material {

	pub fn new(src: &str, new_uniforms: &[UniformItem]) -> Self {
		let uniforms = new_uniforms.iter().map(|u| u.clone()).collect();

		Self {
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			uniforms,
		}
	}


	pub fn new_basic(color: &Vector4<f32>) -> Self {
		Material::new(
			"basic",
			&[
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
			"basic-texture",
			&[
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
			"normal",
			&[]
		)
	}


	pub fn new_mat_cup() -> Self {
		Material::new(
			"mat_cup2",
			&[
				UniformItem {
					name: "texture_color".to_string(),
					uniform: Uniform::Texture2D(None),
					need_update: true,
				}
			]
		)
	}


	pub fn new_light(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		Material::new(
			"light",
			&[
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



	pub fn new_test_mat(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		Material::new(
			"test_mat1",
			&[
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
				// UniformItem {
				// 	name: "position_light".to_string(),
				// 	uniform: Uniform::Vector3(position_light.clone()),
				// 	need_update: true,
				// },
			]
		)
	}


	pub fn new_light_texture(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		Material::new(
			"light_texture",
			&[
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
		let mut m = Material::new(
			"phong",
			&[
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
		);

		m.set_uniform("colors[0]", &Uniform::Vector3(Vector3::new(0.0,1.0,0.0)));
		m.set_uniform("colors[1]", &Uniform::Vector3(Vector3::new(0.0,0.0,1.0)));

		m
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
