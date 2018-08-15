extern crate uuid;
use self::uuid::Uuid;
use math::Color;

extern crate specs;
use self::specs::{Component, VecStorage};
use super::{Texture};
use std::sync::{Arc, Mutex};


use std::marker::Send;
// #[allow(dead_code)]
// pub struct Material {
// 	pub uuid: Uuid,
// 	pub name: String,
// }


#[derive(Clone)]
pub enum MaterialVertexColors {
	NoColors,
	FaceColors,
	VertexColors,
}


#[allow(dead_code)]
pub trait Material
where Self: Send
{
	fn get_uuid(&self) -> &Uuid;
	fn get_name(&self) -> &String;
	fn get_textures(&self) -> Vec<Arc<Mutex<Texture>>> ;
}



#[allow(dead_code)]
#[derive(Clone)]
pub struct MeshNormalMaterial {
	pub uuid: Uuid,
    pub name: String,
	pub opacity: f32,
	pub transparent: bool,
}

#[derive(Clone)]
pub struct MeshBasicMaterial {
	pub uuid: Uuid,
	pub name: String,
	pub opacity: f32,
	pub transparent: bool,
	pub color: Color,
	pub vertex_colors: MaterialVertexColors,

	pub map_color: Option<Arc<Mutex<Texture>>>,
}

impl  Material for MeshBasicMaterial
{
	fn get_uuid(&self) -> &Uuid{ &self.uuid }
	fn get_name(&self) -> &String { &self.name }

	fn get_textures(&self) -> Vec<Arc<Mutex<Texture>>> {
		let mut data = Vec::new();

		// self.map_color.map(|map| data.push(map.clone()) );
		self.map_color.as_ref().map(|map| data.push(map.clone()));

		data
	}
}

impl Material for MeshNormalMaterial {
	fn get_uuid(&self) -> &Uuid{ &self.uuid }
	fn get_name(&self) -> &String { &self.name }
	fn get_textures(&self) -> Vec<Arc<Mutex<Texture>>> { Vec::new() }
}

impl MeshBasicMaterial {
	pub fn new(color: Color) -> MeshBasicMaterial {
		MeshBasicMaterial {
			uuid: Uuid::new_v4(),
			name: "".to_string(),
			opacity: 1.0,
			transparent: false,
			vertex_colors: MaterialVertexColors::NoColors,
			color,

			map_color: None,
		}
	}
}

#[derive(Clone)]
pub enum Materials {
	Normal(MeshNormalMaterial),
	Basic(MeshBasicMaterial),
}


impl Component for Materials {
	type Storage = VecStorage<Self>;
}

impl Materials {
	pub fn duplicate(&self) -> Self {
		let mut data = self.clone();

		match data {
			Materials::Basic(ref mut m) => { m.uuid = Uuid::new_v4(); }
			Materials::Normal(ref mut m) => { m.uuid = Uuid::new_v4(); }
			_ => {}
		}

		data
	}
}
