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
	tags: Vec<String>,
}


impl ShaderProgram for Material {
	fn get_src(&self) -> &str {
		&self.src[..]
	}

	fn get_uniforms(&self) -> &Vec<UniformItem> {
		&self.uniforms
	}

	fn get_uniforms_mut(&mut self) -> &mut Vec<UniformItem> {
		&mut self.uniforms
	}

	fn get_uniforms_slice_mut(&mut self) -> &mut [UniformItem] {
		&mut self.uniforms
	}

	fn get_tags(&self) -> &Vec<String> {
		&self.tags
	}
}


#[allow(dead_code)]
impl Material {

	pub fn new(src: &str, tags: Vec<String>, new_uniforms: &[UniformItem]) -> Self {
		let uniforms = new_uniforms.iter().map(|u| u.clone()).collect();

		Self {
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			uniforms,
			tags,
		}
	}


	pub fn new_basic(color: &Vector4<f32>) -> Self {
		Material::new(
			"basic",
			vec![],
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
			vec!["LIGHTING".to_string()],
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
			vec![],
			&[]
		)
	}


	pub fn new_mat_cup() -> Self {
		Material::new(
			"mat_cup2",
			vec![],
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
			vec!["LIGHTING".to_string()],
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



	pub fn new_test_mat() -> Self {
		Material::new(
			"test_mat1",
			vec!["LIGHTING".to_string()],
			&[]
		)
	}


	pub fn new_light_texture(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		Material::new(
			"light_texture",
			vec!["LIGHTING".to_string()],
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
			vec!["LIGHTING".to_string()],
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
