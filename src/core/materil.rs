extern crate uuid;
use self::uuid::Uuid;
use math::Color;

extern crate specs;
use self::specs::{Component, VecStorage};


use std::marker::Send;
// #[allow(dead_code)]
// pub struct Material {
// 	pub uuid: Uuid,
// 	pub name: String,
// }

#[allow(dead_code)]
pub trait Material
where Self: Send
{
	fn get_uuid(&self) -> &Uuid;
	fn get_name(&self) -> &String;
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
	pub color: Color
}

impl  Material for MeshBasicMaterial
{
	fn get_uuid(&self) -> &Uuid{ &self.uuid }
	fn get_name(&self) -> &String { &self.name }
}

impl Material for MeshNormalMaterial {
	fn get_uuid(&self) -> &Uuid{ &self.uuid }
	fn get_name(&self) -> &String { &self.name }
}

impl MeshBasicMaterial {
	pub fn new(color: Color) -> MeshBasicMaterial {
		MeshBasicMaterial {
			uuid: Uuid::new_v4(),
			name: "".to_string(),
			opacity: 1.0,
			transparent: false,
			color,
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
