extern crate uuid;

use super::BufferGeometry;
use self::uuid::Uuid;
use super::Material;


pub trait Node {}


#[allow(dead_code)]
pub struct Mesh<M, N>
where
M: Material,
N: Node
{
	pub uuid: Uuid,
	pub name: String,
	pub geometry: BufferGeometry,
	pub material: M,
	pub children: Vec<N>,
	// rotation
	// position
	// scale
	// quaternion
}

impl <M, N> Node for Mesh<M, N>
where
M: Material,
N: Node
{}