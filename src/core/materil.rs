extern crate uuid;
use self::uuid::Uuid;
use math::Color;


// #[allow(dead_code)]
// pub struct Material {
// 	pub uuid: Uuid,
// 	pub name: String,
// }

#[allow(dead_code)]
pub trait Material {
	fn get_uuid(&self) -> &Uuid;
	fn get_name(&self) -> &String;
}

#[allow(dead_code)]
pub struct MeshNormalMaterial {
	pub uuid: Uuid,
    pub name: String,
	pub opacity: f32,
	pub transparent: bool,
}

pub struct MeshBasicMaterial<T> {
	pub uuid: Uuid,
	pub name: String,
	pub opacity: f32,
	pub transparent: bool,
	pub color: Color<T>
}

impl <T> Material for MeshBasicMaterial<T> {
	fn get_uuid(&self) -> &Uuid{ &self.uuid }
	fn get_name(&self) -> &String { &self.name }
}

impl Material for MeshNormalMaterial {
	fn get_uuid(&self) -> &Uuid{ &self.uuid }
	fn get_name(&self) -> &String { &self.name }
}

impl <T> MeshBasicMaterial<T> {
	pub fn new(color: Color<T>) -> MeshBasicMaterial<T> {
		MeshBasicMaterial {
			uuid: Uuid::new_v4(),
			name: "".to_string(),
			opacity: 1.0,
			transparent: false,
			color,
		}
	}
}