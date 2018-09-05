extern crate uuid;
use self::uuid::Uuid;
use math::Color;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::{Texture};
use std::sync::{Arc, Mutex};
use std::marker::Send;
use math::*;

#[derive(Debug, Copy)]
pub enum Uniform {
	Vector3f32(Vector3<f32>),
	Vector3f64(Vector3<f64>),
	Vector2f32(Vector2<f32>),
	Vector2f64(Vector2<f64>),
	Matrix4f64(Matrix4<f64>),
	Matrix4f32(Matrix4<f32>),
	// Texture(Texture),
}

#[derive(Debug)]
struct UniformItem {
	name: String,
	vertex: bool,
	fragment: bool,
	need_update: bool,
	uniform: Uniform,
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
	name: String,
	pub uuid: Uuid,
	pub transparent: bool,
	textures: Vec<(String, Arc<Mutex<Texture>>)>,
	uniforms: Vec<(String, UniformItem)>,
	pub uniforms_need_update: bool,
	pub uniform_need_update: bool,
	transform: Matrix4<f32>,
	// transform_need_update: bool,
}


impl Material {
	pub fn new(name: &str) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			name: name.to_string(),
			transparent: false,
			textures: Vec::new(),
			uniforms: Vec::new(),
			transform: Matrix4::new(),
		}
	}


	pub fn set_transform(&mut self, m: &Matrix4<f32>) {
		self.transform.copy(m);
	}


	pub fn add_uniform(&mut self, name: &str, vertex:bool, fragment:bool, u: &Uniform) -> Result<(),()> {
		let uniform = self.uniforms
			.iter()
			.find(|e| e.0 == name);

		if uniform.is_some() { return Err(()); }

		self.uniforms.push(UniformItem {
			name: name.to_string(),
			vertex,
			fragment,
			uniform: u.copy(),
		});

		self.uniforms_need_update = true;

		Ok(())
	}


	pub fn set_uniform(&mut self, name: &str, u: &Uniform) -> Result<(),()> {
		let uniform_item = self.uniforms
			.iter()
			.find(|e| e.0 == name)?;

		match (uniform_item.uniform, u) {
			(Uniform::Vector3f32(a), Uniform::Vector3f32(b)) => { uniform_item.uniform = b },
			(Uniform::Vector3f64(a), Uniform::Vector3f64(b)) => { uniform_item.uniform = b },
			(Uniform::Vector2f32(a), Uniform::Vector2f32(b)) => { uniform_item.uniform = b },
			(Uniform::Vector2f64(a), Uniform::Vector2f64(b)) => { uniform_item.uniform = b },
			(Uniform::Matrix4f32(a), Uniform::Matrix4f32(b)) => { uniform_item.uniform = b },
			(Uniform::Matrix4f64(a), Uniform::Matrix4f64(b)) => { uniform_item.uniform = b },
			_ => {return Err(());}
		};

		self.uniform_need_update = true;

		Ok(())
	}


	pub fn remove_uniform(&mut self, name: &str) {
		self.uniforms = self.uniforms
			.iter()
			.filter(|e| e.0 != name)
			.collect();

		self.uniforms_need_update = true;
	}


	pub fn get_name(&self) -> &str {
		self.name[..]
	}


	pub fn add_texture(&mut self, name: &str, t: Arc<Mutex<Texture>>) -> Result<(),()> {
		let texture = self.textures
			.iter()
			.find(|e| e.0 == name);

		if texture.is_some() { return Err(()); }

		self.textures.push((name.to_string(), t));
		Ok(())
	}

	pub fn remove_texture(&mut self, name: &str) {
		self.textures = self.textures
			.iter()
			.filter(|e| e.0 != name)
			.collect();
	}

	pub fn set_texture(&mut self, name: &str, t: Arc<Mutex<Texture>>) {
		let texture = self.textures
			.iter()
			.find(|e| e.0 == name);

		if texture.is_some() {
			let texture = texture.unwrap();
			texture.1 = t;
		} else {
			self.add_texture(name, t);
		}
	}

	pub fn get_textures(&self) -> &[(String, Arc<Mutex<Texture>>)] {
		self.textures[..]
	}

	pub fn get_uniforms(&self) -> &[(String, UniformItem)] {
		self.uniforms[..]
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


// impl Component for Materials {
// 	type Storage = VecStorage<Self>;
// }

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
