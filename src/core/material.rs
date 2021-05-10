use super::{Blending, ShaderProgram, ShaderDef, ToUniform, Uniform, UniformName};
use math::{Vector, Vector3, Vector4};
use std::collections::{HashMap};
use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Material {
	pub name: String,
	pub blending: Blending,
	pub uniforms: HashMap<UniformName, Uniform>,
	shader_program: Arc<Mutex<ShaderProgram>>,
	_uuid: Uuid,
}

#[allow(dead_code)]
impl Material {
	pub fn get_shader_program(&self) -> LockResult<MutexGuard<ShaderProgram>> {
		self.shader_program.lock()
	}

	pub fn need_update(&mut self) {
		self.shader_program.lock().unwrap().set_need_update(true)
	}

	pub fn add_definition(&mut self, key: ShaderDef, val: String) {
		self.shader_program.lock().unwrap().add_definition(key, val);
	}

	pub fn has_definition(&mut self, key: ShaderDef) -> bool {
		self.shader_program.lock().unwrap().has_definition(key)
	}

	pub fn remove_definition(&mut self, key: ShaderDef) {
		self.shader_program.lock().unwrap().remove_definition(key);
	}

	pub fn set_uniform<T: ToUniform>(&mut self, name: UniformName, value: T) {
		self.uniforms.insert(name, value.to_uniform());
	}

	pub fn uuid(&self) -> Uuid {
		self._uuid
	}

	pub fn new(src: &str) -> Self {
		Self {
			uniforms: HashMap::new(),
			name: "".to_string(),
			blending: Blending::None,
			shader_program: Arc::new(Mutex::new(ShaderProgram::new(src.to_string()))),
			_uuid: Uuid::new_v4(),
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
		mat.add_definition(ShaderDef::Lighting, "".to_string());
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

	pub fn new_light(
		color: Vector4<f32>,
		color_light: Vector3<f32>,
		position_light: Vector3<f32>,
	) -> Self {
		let mut mat = Material::new("light");
		mat.add_definition(ShaderDef::Lighting, "".to_string());
		mat.set_uniform(UniformName::Color, color);
		mat.set_uniform(UniformName::Other("color_light".to_string()), color_light);
		mat.set_uniform(
			UniformName::Other("position_light".to_string()),
			position_light,
		);
		mat
	}

	pub fn new_mesh_phong() -> Self {
		let mut mat = Material::new("mesh_phong");
		mat.add_definition(ShaderDef::Lighting, "".to_string());
		mat
	}

	pub fn new_mesh_standard() -> Self {
		let mut mat = Material::new("mesh_standard");
		mat.add_definition(ShaderDef::Lighting, "".to_string());

		mat.set_uniform(UniformName::Color, Vector3::new_one());
		mat.set_uniform(UniformName::Specular, Vector3::new_one());
		mat.set_uniform(UniformName::Roughness, 1.0);
		mat.set_uniform(UniformName::Metalness, 0.0);
		mat.set_uniform(UniformName::AmbientLight, Vector3::new(0.0, 0.0, 0.0));

		mat
	}

	pub fn new_light_texture(
		color: Vector4<f32>,
		color_light: Vector3<f32>,
		position_light: Vector3<f32>,
	) -> Self {
		let mut mat = Material::new("light_texture");
		mat.add_definition(ShaderDef::Lighting, "".to_string());

		mat.set_uniform(UniformName::Color, color);
		mat.set_uniform(UniformName::Other("color_light".to_string()), color_light);
		mat.set_uniform(
			UniformName::Other("position_light".to_string()),
			position_light,
		);
		mat.set_uniform(UniformName::MapSpecular, None);
		mat.set_uniform(UniformName::MapColor, None);
		mat
	}

	pub fn new_phong(
		color: Vector4<f32>,
		color_light: Vector3<f32>,
		position_light: Vector3<f32>,
	) -> Self {
		let mut mat = Material::new("phong");
		mat.add_definition(ShaderDef::Lighting, "".to_string());

		mat.set_uniform(UniformName::Color, color);
		mat.set_uniform(UniformName::Other("color_light".to_string()), color_light);
		mat.set_uniform(
			UniformName::Other("position_light".to_string()),
			position_light,
		);
		mat.set_uniform(
			UniformName::Other("colors[0]".to_string()),
			Vector3::new(0.0, 1.0, 0.0),
		);
		mat.set_uniform(
			UniformName::Other("colors[1]".to_string()),
			Vector3::new(0.0, 0.0, 1.0),
		);

		mat
	}

	pub fn new_frame_buffer() -> Self {
		Material::new("frame_buffer")
	}

	pub fn to_shared(self) -> SharedMaterial {
		SharedMaterial::new(self)
	}
}

#[derive(Debug, Clone)]
pub struct SharedMaterial(Uuid, Arc<Mutex<Material>>);

impl SharedMaterial {
	pub fn new(material: Material) -> Self {
		let uuid = material.uuid();
		SharedMaterial(uuid, Arc::new(Mutex::new(material)))
	}

	pub fn lock(&mut self) -> LockResult<MutexGuard<Material>> {
		self.1.lock()
	}

	pub fn uuid(&mut self) -> Uuid {
		self.0
	}
}
