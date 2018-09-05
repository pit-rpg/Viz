extern crate uuid;
use self::uuid::Uuid;
// use math::Color;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::{Texture};
use std::sync::{Arc, Mutex};
use std::marker::Send;
use math::*;

#[derive(Debug, Clone)]
pub enum Uniform {
	Vector3f32(Vector3<f32>),
	Vector3f64(Vector3<f64>),
	Vector2f32(Vector2<f32>),
	Vector2f64(Vector2<f64>),
	Matrix4f64(Matrix4<f64>),
	Matrix4f32(Matrix4<f32>),
	// Texture(Texture),
}

#[derive(Debug, Clone)]
pub struct UniformItem {
	pub name: String,
	pub vertex: bool,
	pub fragment: bool,
	pub need_update: bool,
	pub uniform: Uniform,
}


// #[allow(dead_code)]
// pub struct Material {
// 	pub uuid: Uuid,
// 	pub name: String,
// }


// #[derive(Clone)]
// pub enum MaterialVertexColors {
// 	NoColors,
// 	FaceColors,
// 	VertexColors,
// }

#[allow(dead_code)]
#[derive(Clone)]
pub struct Material {
	// pub opacity: f32,
	pub name: String,
	pub uuid: Uuid,
	src: String,
	textures: Vec<(String, Arc<Mutex<Texture>>)>,
	uniforms: Vec<UniformItem>,
	transform: Matrix4<f32>,
	pub uniform_need_update: bool,
	// pub transparent: bool,
	// transform_need_update: bool,
}


impl Material {
	pub fn new(src: &str, name: &str, new_uniforms: &[UniformItem]) -> Self {
		let uniforms = new_uniforms
			.iter()
			.map(|u| u.clone())
			.collect();

		Self {
			name: name.to_string(),
			uuid: Uuid::new_v4(),
			src: src.to_string(),
			textures: Vec::new(),
			uniforms,
			transform: Matrix4::new(),
			uniform_need_update: true,
		}
	}


	pub fn set_transform(&mut self, m: &Matrix4<f32>) {
		self.transform.copy(m);
	}


	pub fn set_uniform(&mut self, name: &str, u: &Uniform) {
		let uniform_item = self.uniforms
			.iter_mut()
			.find(|e| *e.name == *name)
			// .as_mut()
			.unwrap();

		// let u = u.clone();

		match (&mut uniform_item.uniform, u) {
			(Uniform::Vector3f32(ref mut a), Uniform::Vector3f32(b)) => { a.copy(&b); uniform_item.need_update = true; },
			(Uniform::Vector3f64(ref mut a), Uniform::Vector3f64(b)) => { a.copy(&b); uniform_item.need_update = true; },
			(Uniform::Vector2f32(ref mut a), Uniform::Vector2f32(b)) => { a.copy(&b); uniform_item.need_update = true; },
			(Uniform::Vector2f64(ref mut a), Uniform::Vector2f64(b)) => { a.copy(&b); uniform_item.need_update = true; },
			(Uniform::Matrix4f32(ref mut a), Uniform::Matrix4f32(b)) => { a.copy(&b); uniform_item.need_update = true; },
			(Uniform::Matrix4f64(ref mut a), Uniform::Matrix4f64(b)) => { a.copy(&b); uniform_item.need_update = true; },
			_ => {return panic!();}
		};

		self.uniform_need_update = true;
	}


	pub fn get_src(&self) -> &str {
		&self.src[..]
	}


	pub fn set_texture(&mut self, name: &str, t: Option<Arc<Mutex<Texture>>>) {
		match t {
			Some (t) => {
				{
					let texture = self.textures
						.iter_mut()
						.find(|e| e.0 == name);

					if texture.is_some() {
						let texture = texture.unwrap();
						texture.1 = t;
						return;
					}
				}
				self.textures.push((name.to_string(), t));
			}
			None => {
				let textures = self.textures
					.drain(..)
					.filter(|e| e.0 != name)
					.collect();
				self.textures = textures;
			}
		}
	}


	pub fn get_textures(&self) -> &[(String, Arc<Mutex<Texture>>)] {
		&self.textures[..]
	}

	pub fn get_uniforms(&self) -> &[UniformItem] {
		&self.uniforms[..]
	}

	pub fn new_basic() -> Self {
		Material::new("basic.glsl", "Basic", &[
			UniformItem {
				name: "color".to_string(),
				vertex: false,
				fragment: true,
				need_update: true,
				uniform: Uniform::Vector3f32(Vector3::<f32>::random()),
			}
		])
	}

	pub fn new_normal() -> Self {
		Material::new("normal.glsl", "Normal", &[])
	}
}


// #[allow(dead_code)]
// pub trait Material
// where Self: Send
// {
// 	fn get_uuid(&self) -> &Uuid;
// 	fn get_name(&self) -> &String;


// 	fn get_textures(&self, names: bool) -> [Option<(Option<String>, Arc<Mutex<Texture>>)>;16] {
// 		[None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None]
// 	}
// }



// #[allow(dead_code)]
// #[derive(Clone)]
// pub struct MeshNormalMaterial {
// 	pub uuid: Uuid,
//     pub name: String,
// 	pub opacity: f32,
// 	pub transparent: bool,
// }

// #[derive(Clone)]
// pub struct MeshBasicMaterial {
// 	pub uuid: Uuid,
// 	pub name: String,
// 	// pub color: Color,
// 	// pub opacity: f32,
// 	// pub transparent: bool,
// 	// pub vertex_colors: MaterialVertexColors,

// 	pub map_color: Option<Arc<Mutex<Texture>>>,
// 	pub map_color2: Option<Arc<Mutex<Texture>>>,
// }


// impl Material for MeshBasicMaterial
// {
// 	fn get_uuid(&self) -> &Uuid{ &self.uuid }
// 	fn get_name(&self) -> &String { &self.name }


// 	fn get_textures(&self, names: bool) -> [Option<(Option<String>, Arc<Mutex<Texture>>)>;16] {
// 		let mut data = [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None];

// 		self.map_color.iter()
// 			.for_each(|map| data[0] = Some((if names {Some("map_color".to_string())} else {None}, map.clone())) );

// 		self.map_color2.iter()
// 			.for_each(|map| data[1] = Some((if names {Some("map_color2".to_string())} else {None}, map.clone())) );

// 		data
// 	}
// }


// impl Material for MeshNormalMaterial {
// 	fn get_uuid(&self) -> &Uuid{ &self.uuid }
// 	fn get_name(&self) -> &String { &self.name }
// 	// fn get_textures(&self, names: bool) -> [Option<(Option<String>, Arc<Mutex<Texture>>)>;16] { Vec::new() }
// }


// impl MeshBasicMaterial {
// 	pub fn new(color: Color) -> MeshBasicMaterial {
// 		MeshBasicMaterial {
// 			uuid: Uuid::new_v4(),
// 			name: "".to_string(),
// 			// opacity: 1.0,
// 			// transparent: false,
// 			// vertex_colors: MaterialVertexColors::NoColors,
// 			// color,

// 			map_color: None,
// 			map_color2: None,
// 		}
// 	}
// }


// impl MeshNormalMaterial {
// 	pub fn new(color: Color) -> Self {
// 		Self {
// 			uuid: Uuid::new_v4(),
// 			name: "".to_string(),
// 			opacity: 1.0,
// 			transparent: false,
// 		}
// 	}
// }

// #[derive(Clone)]
// pub enum Materials {
// 	Normal(MeshNormalMaterial),
// 	Basic(MeshBasicMaterial),
// }


impl Component for Material {
	type Storage = VecStorage<Self>;
}

// impl Materials {
// 	pub fn duplicate(&self) -> Self {
// 		let mut data = self.clone();

// 		match data {
// 			Materials::Basic(ref mut m) => { m.uuid = Uuid::new_v4(); }
// 			Materials::Normal(ref mut m) => { m.uuid = Uuid::new_v4(); }
// 			_ => {}
// 		}

// 		data
// 	}
// }
