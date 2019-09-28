extern crate uuid;
use self::uuid::Uuid;

extern crate specs;

use std::collections::HashSet;

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

#[derive(Debug, Clone, PartialEq)]
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
	// pub fn get_tag(&self) -> Option<ShaderTag> {
	// 	match self {
	// 		UniformName::Metalness => Some(ShaderTag::Metalness),
	// 		UniformName::AmbientLight => Some(ShaderTag::AmbientLight),
	// 		UniformName::Emissive => Some(ShaderTag::Emissive),
	// 		UniformName::Alpha => Some(ShaderTag::Transparent),
	// 		UniformName::MapAlpha => Some(ShaderTag::Transparent),
	// 		_ => None,
	// 	}
	// }

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
	Transparent,
	Additive,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ShaderTag {
	Lighting,
	Metalness,
	AmbientLight,
	Transparent,
	Emissive,
	Additive,
	Shadeless,

	Other(String),
}

#[derive(Debug, Clone)]
pub struct UniformItem {
	pub name: UniformName,
	pub uniform: Uniform,
	pub need_update: bool,
}

#[allow(dead_code)]
pub trait ShaderProgram {
	fn set_uniform<T: ToUniform>(&mut self, name: UniformName, value: T) {
		let uniforms = self.get_uniforms_mut();
		let uniform = value.to_uniform();

		let res = uniforms.iter_mut().find(|e| e.name == name);
		if let Some(uniform_item) = res {
			if uniform_item.uniform == uniform {
				return;
			}
			uniform_item.uniform = uniform;
			uniform_item.need_update = true;
		} else {
			let new_uniform = UniformItem {
				name,
				uniform: uniform,
				need_update: true,
			};

			uniforms.push(new_uniform);
			self.set_need_update(true);
		}
	}

	fn get_src(&self) -> &str;

	fn get_uniforms(&self) -> &Vec<UniformItem>;
	fn get_uniforms_mut(&mut self) -> &mut Vec<UniformItem>;
	fn get_uniforms_slice_mut(&mut self) -> &mut [UniformItem];

	fn remove_tag(&mut self, tag: ShaderTag);
	fn add_tag(&mut self, tag: ShaderTag);
	fn has_tag(&self, tag: ShaderTag) -> bool;
	fn get_tags(&self) -> &HashSet<ShaderTag>;
	fn get_tags_mut(&mut self) -> &mut HashSet<ShaderTag>;

	fn get_uuid(&self) -> Uuid;

	fn is_need_update(&self) -> bool;
	fn set_need_update(&mut self, bool);

	fn blending(&self) -> Blending;
	fn set_blending(&mut self, blending: Blending);
}

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
