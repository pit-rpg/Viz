extern crate uuid;

use super::BufferGeometry;
use self::uuid::Uuid;


#[allow(dead_code)]
pub struct Mesh {
	pub uuid: Uuid,
	pub name: String,
	geometry: BufferGeometry,
	// materilas
	// children
	// rotation
	// position
	// scale
	// quaternion
}