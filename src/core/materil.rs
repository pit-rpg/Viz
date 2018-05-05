extern crate uuid;
use self::uuid::Uuid;


// #[allow(dead_code)]
// pub struct Material {
// 	pub uuid: Uuid,
// 	pub name: String,
// }

#[allow(dead_code)]
pub trait Material {

}


pub struct MeshNormalMaterial {}
pub struct MeshBasicMaterial {}

impl Material for MeshBasicMaterial {}
impl Material for MeshNormalMaterial {}