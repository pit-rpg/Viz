extern crate uuid;
use self::uuid::Uuid;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::{
	UniformItem,
	Uniform,
	ShaderProgram,
	ShaderTag,
};

use math::{
	Vector3,
	Vector4,
	Vector,
};

use std::sync::{
	Arc,
	Mutex,
	MutexGuard,
	LockResult
};

use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Material {
	pub name: String,
	uuid: Uuid,
	src: String,
	uniforms: Vec<UniformItem>,
	tags: HashSet<ShaderTag>,
	need_update: bool,
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

	fn get_tags(&self) -> &HashSet<ShaderTag> {
		&self.tags
	}

	fn get_tags_mut(&mut self) -> &mut HashSet<ShaderTag> {
		&mut self.tags
	}

	fn get_uuid(&self) -> Uuid {
		self.uuid
	}

	fn is_need_update(&self) -> bool {
		self.need_update
	}

	fn set_need_update(&mut self, update: bool) {
		self.need_update = update;
	}
}


#[allow(dead_code)]
impl Material {

	pub fn new(src: &str, tags: HashSet<ShaderTag>, new_uniforms: &[UniformItem]) -> Self {
		let uniforms = new_uniforms.iter().map(|u| u.clone()).collect();

		Self {
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			uniforms,
			tags,
			name: "".to_string(),
			need_update: true,
		}
	}


	pub fn new_basic(color: &Vector4<f32>) -> Self {
		Material::new(
			"basic",
			HashSet::new(),
			&[
				UniformItem {
					name: "color".to_string(),
					uniform: Uniform::Vector4(color.clone()),
					need_update: true,
				},
			]
		)
	}

	pub fn new_basic_texture() -> Self {
		let mut set = HashSet::new();
		set.insert(ShaderTag::Lighting);
		Material::new(
			"basic-texture",
			set,
			&[
				UniformItem {
					name: "texture_color".to_string(),
					uniform: Uniform::Texture2D(None, 0),
					need_update: true,
				}
			]
		)
	}

	pub fn new_normal() -> Self {
		Material::new(
			"normal",
			HashSet::new(),
			&[]
		)
	}


	pub fn new_mat_cup() -> Self {
		Material::new(
			"mat_cup2",
			HashSet::new(),
			&[
				UniformItem {
					name: "texture_color".to_string(),
					uniform: Uniform::Texture2D(None, 0),
					need_update: true,
				}
			]
		)
	}


	pub fn new_light(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		let mut set = HashSet::new();
		set.insert(ShaderTag::Lighting);

		Material::new(
			"light",
			set,
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



	pub fn new_mesh_phong() -> Self {
		let mut set = HashSet::new();
		set.insert(ShaderTag::Lighting);

		Material::new(
			"mesh_phong",
			set,
			&[]
		)
	}

	pub fn new_mesh_standard() -> Self {
		let mut set = HashSet::new();
		set.insert(ShaderTag::Lighting);
		set.insert(ShaderTag::VertexPosition);

		let mut mat = Material::new(
			"mesh_standard",
			set,
			&[]
		);

		mat.set_uniform("diffuse", Vector3::new_one());
		mat.set_uniform("specular", Vector3::new_one());
		mat.set_uniform("roughness", 1.0);
		mat.set_uniform("metalness", 0.0);
		mat.set_uniform("ambientLightColor", Vector3::new(0.0,0.0,0.0));

		mat
	}


	pub fn new_light_texture(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		let mut set = HashSet::new();
		set.insert(ShaderTag::Lighting);

		Material::new(
			"light_texture",
			set,
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
					uniform: Uniform::Texture2D(None, 0),
					need_update: true,
				},
				UniformItem {
					name: "texture_color".to_string(),
					uniform: Uniform::Texture2D(None, 0),
					need_update: true,
				}
			]
		)
	}


	pub fn new_phong(color: &Vector4<f32>, color_light: &Vector3<f32>, position_light: &Vector3<f32>) -> Self {
		let mut set = HashSet::new();
		set.insert(ShaderTag::Lighting);

		let mut m = Material::new(
			"phong",
			set,
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

		m.set_uniform("colors[0]", Vector3::new(0.0,1.0,0.0));
		m.set_uniform("colors[1]", Vector3::new(0.0,0.0,1.0));

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
