use uuid::Uuid;

use std::collections::{HashMap};

use super::SharedTexture2D;

use math::{Matrix3, Matrix4, Vector2, Vector3, Vector4};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Uniform {
	Vector2(Vector2<f32>),
	Vector3(Vector3<f32>),
	Vector4(Vector4<f32>),
	Matrix4(Matrix4<f32>),
	Matrix3(Matrix3<f32>),
	Float(f32),
	Int(i32),
	UInt(u32),
	Texture2D(Option<SharedTexture2D>, u32),
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum UniformName {
	Color,
	Alpha,
	Normal,
	NormalScale,
	Emissive,
	SpecularStrength,
	Specular,
	Roughness,
	Metalness,
	AmbientLight,
	Shininess,

	MapColor,
	MapSpecular,
	MapRoughness,
	MapNormal,
	MapMetalness,
	MapAlpha,
	MapEmissive,

	MatrixModel,
	MatrixView,
	MatrixNormal,
	MapOcclusion,

	Time,
	Other(String),
}

impl UniformName {
	pub fn get_name(&self) -> String {
		match self {
			UniformName::Color => "color".to_string(),
			UniformName::Normal => "normal".to_string(),
			UniformName::NormalScale => "normal_scale".to_string(),
			UniformName::Specular => "specular".to_string(),
			UniformName::SpecularStrength => "specular_strength".to_string(),
			UniformName::Emissive => "emissive".to_string(),
			UniformName::Time => "time".to_string(),
			UniformName::Alpha => "alpha".to_string(),
			UniformName::Roughness => "roughness".to_string(),
			UniformName::Metalness => "metalness".to_string(),
			UniformName::AmbientLight => "ambient_light".to_string(),

			UniformName::MapColor => "map_color".to_string(),
			UniformName::MapSpecular => "map_specular".to_string(),
			UniformName::MapRoughness => "map_roughness".to_string(),
			UniformName::MapNormal => "map_normal".to_string(),
			UniformName::MapMetalness => "map_metalness".to_string(),
			UniformName::MapEmissive => "map_emissive".to_string(),
			UniformName::MapOcclusion => "map_occlusion".to_string(),
			UniformName::MapAlpha => "map_alpha".to_string(),

			UniformName::MatrixModel => "matrix_model".to_string(),
			UniformName::MatrixView => "matrix_view".to_string(),
			UniformName::MatrixNormal => "matrix_normal".to_string(),
			UniformName::Shininess => "shininess".to_string(),

			UniformName::Other(value) => value.clone(),
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Blending {
	None,
	Mix,
	Additive,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ShaderDef {
	Lighting,
	Metalness,
	AmbientLight,
	Transparent,
	Emissive,
	Additive,
	Shadeless,
	ReceiveShadows,
	CastShadows,

	Other(String),
}

#[derive(Debug, Clone)]
pub struct UniformItem {
	pub name: UniformName,
	pub uniform: Uniform,
	pub need_update: bool,
}

// #[allow(dead_code)]
// pub trait ShaderProgram {
// 	fn set_uniform<T: ToUniform>(&mut self, name: UniformName, value: T) {
// 		let uniforms = self.get_uniforms_mut();
// 		let uniform = value.to_uniform();

// 		let res = uniforms.iter_mut().find(|e| e.name == name);
// 		if let Some(uniform_item) = res {
// 			if uniform_item.uniform == uniform {
// 				return;
// 			}
// 			uniform_item.uniform = uniform;
// 			uniform_item.need_update = true;
// 		} else {
// 			let new_uniform = UniformItem {
// 				name,
// 				uniform: uniform,
// 				need_update: true,
// 			};

// 			uniforms.push(new_uniform);
// 			self.set_need_update(true);
// 		}
// 	}

// 	fn get_src(&self) -> &str;

// 	fn get_uniforms(&self) -> &Vec<UniformItem>;
// 	fn get_uniforms_mut(&mut self) -> &mut Vec<UniformItem>;
// 	fn get_uniforms_slice_mut(&mut self) -> &mut [UniformItem];

// 	fn remove_tag(&mut self, tag: ShaderDef);
// 	fn add_tag(&mut self, tag: ShaderDef);
// 	fn has_tag(&self, tag: ShaderDef) -> bool;
// 	fn get_tags(&self) -> &HashSet<ShaderDef>;
// 	fn get_tags_mut(&mut self) -> &mut HashSet<ShaderDef>;

// 	fn get_uuid(&self) -> Uuid;

// 	fn is_need_update(&self) -> bool;
// 	fn set_need_update(&mut self, bool);

// 	fn blending(&self) -> Blending;
// 	fn set_blending(&mut self, blending: Blending);
// }

pub trait ToUniform: PartialEq {
	fn to_uniform(self) -> Uniform;
}

impl ToUniform for Vector2<f32> {
	fn to_uniform(self) -> Uniform {
		Uniform::Vector2(self)
	}
}
impl ToUniform for Vector3<f32> {
	fn to_uniform(self) -> Uniform {
		Uniform::Vector3(self)
	}
}
impl ToUniform for Vector4<f32> {
	fn to_uniform(self) -> Uniform {
		Uniform::Vector4(self)
	}
}
impl ToUniform for Matrix3<f32> {
	fn to_uniform(self) -> Uniform {
		Uniform::Matrix3(self)
	}
}
impl ToUniform for Matrix4<f32> {
	fn to_uniform(self) -> Uniform {
		Uniform::Matrix4(self)
	}
}
impl ToUniform for f32 {
	fn to_uniform(self) -> Uniform {
		Uniform::Float(self)
	}
}
impl ToUniform for i32 {
	fn to_uniform(self) -> Uniform {
		Uniform::Int(self)
	}
}
impl ToUniform for u32 {
	fn to_uniform(self) -> Uniform {
		Uniform::UInt(self)
	}
}
impl ToUniform for (Option<SharedTexture2D>, u32) {
	fn to_uniform(self) -> Uniform {
		Uniform::Texture2D(self.0, self.1)
	}
}
impl ToUniform for Option<SharedTexture2D> {
	fn to_uniform(self) -> Uniform {
		Uniform::Texture2D(self, 0)
	}
}
impl ToUniform for SharedTexture2D {
	fn to_uniform(self) -> Uniform {
		Uniform::Texture2D(Some(self), 0)
	}
}


#[derive(Debug)]
pub struct ShaderProgram {
	uuid: Uuid,
	src: String,
	uniforms: HashMap<UniformName, Uniform>,
	definitions: HashMap<ShaderDef, String>,
	need_update: bool,
	// blending_mode: Blending,
}

impl ShaderProgram {
	pub fn new(src: String) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			uniforms: HashMap::default(),
			definitions: HashMap::new(),
			need_update: true,
			// blending_mode: Blending::None,
		}
	}

	pub fn set_uniform(&mut self, key: &UniformName, val: Uniform) -> bool {
		if let Some(uniform) = self.uniforms.get(key) {
			if *uniform == val {
				return false;
			}
		}
		self.uniforms.insert(key.clone(), val);
		true
	}

	pub fn add_definition(&mut self, key: ShaderDef, val: String) {
		if matches!(self.definitions.get(&key), Some(def_val) if def_val == &val) {
			return;
		}

		self.definitions.insert(key, val);
		self.set_need_update(true);
	}

	pub fn remove_definition(&mut self, key: ShaderDef) {
		if self.definitions.get(&key).is_none() {
			return
		}

		self.definitions.remove(&key);
		self.set_need_update(true);
	}

	pub fn has_definition(&self, key: ShaderDef) -> bool {
		self.definitions.get(&key).is_some()
	}

	pub fn get_definitions(&self) -> &HashMap<ShaderDef, String> {
		&self.definitions
	}

	pub fn get_definitions_mut(&mut self) -> &mut HashMap<ShaderDef, String> {
		&mut self.definitions
	}

	pub fn get_uuid(&self) -> Uuid {
		self.uuid
	}

	pub fn is_need_update(&self) -> bool {
		self.need_update
	}

	pub fn set_need_update(&mut self, update: bool) {
		self.need_update = update;
	}

	// pub fn blending(&self) -> Blending {
	// 	self.blending_mode
	// }

	// pub fn set_blending(&mut self, blending: Blending) {
	// 	if self.blending_mode != blending {
	// 		self.blending_mode = blending;
	// 		self.set_need_update(true);
	// 	}
	// }

	pub fn get_src(&self) -> &str {
		&self.src[..]
	}

	pub fn get_uniforms(&self) -> &HashMap<UniformName, Uniform> {
		&self.uniforms
	}
}
