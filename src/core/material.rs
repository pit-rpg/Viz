extern crate uuid;
use self::uuid::Uuid;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::{
	UniformItem,
	ShaderProgram,
	ShaderTag,
	UniformName,
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

	fn add_tag(&mut self, tag: ShaderTag) {
		self.tags.insert(tag);
	}


	fn has_tag(&self, tag: ShaderTag) -> bool {
		self.tags.get(&tag).is_some()
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

	pub fn new(src: &str) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			uniforms: vec![],
			tags: HashSet::new(),
			name: "".to_string(),
			need_update: true,
		}
	}


	pub fn new_basic(color: Vector4<f32>) -> Self {
		let mut mat = Material::new("basic");
		mat.set_uniform(UniformName::Color, color);
		mat
	}

	pub fn new_basic_texture() -> Self {
		let mut mat = Material::new("basic-texture");
		mat.set_uniform(UniformName::MapColor, None);
		mat.add_tag(ShaderTag::Lighting);
		mat
	}

	pub fn new_normal() -> Self {
		Material::new("normal")
	}

	pub fn new_mat_cup() -> Self {
		let mut mat = Material::new("mat_cup2");
		mat.set_uniform(UniformName::MapColor, None);
		mat
	}


	pub fn new_light(color: Vector4<f32>, color_light: Vector3<f32>, position_light: Vector3<f32>) -> Self {
		let mut mat = Material::new("light");
		mat.add_tag(ShaderTag::Lighting);
		mat.set_uniform(UniformName::Color, color);
		mat.set_uniform(UniformName::Other("color_light".to_string()), color_light);
		mat.set_uniform(UniformName::Other("position_light".to_string()), position_light);
		mat
	}



	pub fn new_mesh_phong() -> Self {
		let mut mat = Material::new("mesh_phong");
		mat.add_tag(ShaderTag::Lighting);
		mat
	}

	pub fn new_mesh_standard() -> Self {
		let mut mat = Material::new("mesh_standard");
		mat.add_tag(ShaderTag::Lighting);

		mat.set_uniform(UniformName::Color, Vector3::new_one());
		mat.set_uniform(UniformName::Specular, Vector3::new_one());
		mat.set_uniform(UniformName::Roughness, 1.0);
		mat.set_uniform(UniformName::Metalness, 0.0);
		mat.set_uniform(UniformName::AmbientLight, Vector3::new(0.0,0.0,0.0));

		mat
	}


	pub fn new_light_texture(color: Vector4<f32>, color_light: Vector3<f32>, position_light: Vector3<f32>) -> Self {
		let mut mat = Material::new("light_texture");
		mat.add_tag(ShaderTag::Lighting);

		mat.set_uniform(UniformName::Color, color);
		mat.set_uniform(UniformName::Other("color_light".to_string()), color_light);
		mat.set_uniform(UniformName::Other("position_light".to_string()), position_light);
		mat.set_uniform(UniformName::MapSpecular, None);
		mat.set_uniform(UniformName::MapColor, None);
		mat
	}


	pub fn new_phong(color: Vector4<f32>, color_light: Vector3<f32>, position_light: Vector3<f32>) -> Self {
		let mut mat = Material::new("phong");
		mat.add_tag(ShaderTag::Lighting);

		mat.set_uniform(UniformName::Color, color);
		mat.set_uniform(UniformName::Other("color_light".to_string()), color_light);
		mat.set_uniform(UniformName::Other("position_light".to_string()), position_light);
		mat.set_uniform(UniformName::Other("colors[0]".to_string()), Vector3::new(0.0,1.0,0.0));
		mat.set_uniform(UniformName::Other("colors[1]".to_string()), Vector3::new(0.0,0.0,1.0));

		mat
	}

}



#[derive(Debug, Clone)]
pub struct SharedMaterials (Vec<Arc<Mutex<Material>>>);

impl Component for SharedMaterials{
	type Storage = VecStorage<Self>;
}

impl SharedMaterials {
	pub fn new(material: Material) -> Self {
		SharedMaterials(vec![Arc::new(Mutex::new(material))])
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn new_collection(mut materials: Vec<Material>) -> Self {
        let materials = materials
            .drain(..)
            .map(|mat| Arc::new(Mutex::new(mat)))
            .collect();

		SharedMaterials(materials)
	}

	pub fn lock(&mut self, index: usize) -> LockResult<MutexGuard<Material>> {
        self.0[index].lock()
	}

	pub fn iter(&self) -> std::slice::Iter<'_, Arc<Mutex<Material>>> {
        self.0.iter()
	}

	pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Arc<Mutex<Material>>> {
        self.0.iter_mut()
	}

	pub fn clone_material(&self, index: usize) -> Arc<Mutex<Material>> {
        self.0[index].clone()
	}
}
