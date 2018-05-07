extern crate uuid;

use super::BufferGeometry;
use self::uuid::Uuid;
use super::Material;


pub trait Node {}


#[allow(dead_code)]
pub struct Mesh<'a, M>
where
M: Material+'a,
// N: Node
{
	pub uuid: Uuid,
	pub name: String,
	pub geometry: &'a BufferGeometry,
	pub material: &'a M,
	// pub children: Vec<Node>,
	// rotation
	// position
	// scale
	// quaternion
}

impl <'a, M> Node for Mesh<'a, M>
where
M: Material,
// N: Node
{}


impl <'a, M> Mesh<'a, M>
where
M: Material,
// N: Node
{
	pub fn new(geometry: &'a BufferGeometry, material: &'a M) -> Mesh<'a, M> {
		Mesh {
			uuid: Uuid::new_v4(),
			name: "".to_string(),
			material,
			geometry,
			// children: Vec::new(),
		}
	}
}