extern crate uuid;
use self::uuid::Uuid;

extern crate specs;

use std::collections::HashSet;

use super::SharedTexture2D;

use math::{Matrix3, Matrix4, Vector, Vector2, Vector3, Vector4};

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
	Normal,
	Specular,
	SpecularStrength,
	Diffuse,
	Roughness,
	Metalness,
	AmbientLightColor,
	TextureColor,
	TextureSpecular,
	TextureNormal,
	MatrixModel,
	MatrixView,
	MatrixNormal,
	Time,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ShaderTag {
	Lighting,
	VertexColor4,
	VertexColor3,
	VertexPosition,
	VertexUV,
	VertexNormal,
	MapDefuse,
	MapNormal,
	MapRoughness,
	MapMetalness,
	MapEmissive,
	Other(String),
}

#[derive(Debug, Clone)]
pub struct UniformItem {
	pub name: String,
	pub uniform: Uniform,
	pub need_update: bool,
	// pub tag: Option<String>,
}

#[allow(dead_code)]
pub trait ShaderProgram {
	fn set_uniform<T: ToUniform>(&mut self, name: &str, value: T) {
		let uniforms = self.get_uniforms_mut();
		let uniform = value.to_uniform();


		let res = uniforms.iter_mut().find(|e| e.name == name);
		if let Some(uniform_item) = res {

			if uniform_item.uniform == uniform {return}
			uniform_item.uniform = uniform;
			uniform_item.need_update = true;
		} else {
			let new_uniform = UniformItem {
				name: name.to_string(),
				uniform: uniform,
				need_update: true,
				// tag: None
			};

			uniforms.push(new_uniform);
		}
	}

	fn get_src(&self) -> &str;
	fn get_uniforms(&self) -> &Vec<UniformItem>;
	fn get_uniforms_mut(&mut self) -> &mut Vec<UniformItem>;
	fn get_uniforms_slice_mut(&mut self) -> &mut [UniformItem];
	fn get_tags(&self) -> &HashSet<ShaderTag>;
	fn get_tags_mut(&mut self) -> &mut HashSet<ShaderTag>;
	fn get_uuid(&self) -> Uuid;
	fn is_need_update(&self) -> bool;
	fn set_need_update(&mut self, bool);
}


pub trait ToUniform : PartialEq {
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




