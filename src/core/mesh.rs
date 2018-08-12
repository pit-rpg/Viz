extern crate uuid;
extern crate specs;

use self::specs::{Component, VecStorage};

// use std::sync::RwLock;
use std::sync::{Arc, Mutex};

use self::uuid::Uuid;
use super::BufferGeometry;
// use super::Component;
use super::Material;
// use math::Vector3;

// pub trait Mesh {}

#[allow(dead_code)]
pub struct Mesh {
	pub uuid: Uuid,
	pub name: String,
	pub geometry: BufferGeometry,
	// pub material: Box<Material>,
	// pub geometry: &'a BufferGeometry,
	// pub material: &'a M,
	// pub children: Vec<Node<T>>,
	// pub attachment: A,
	pub material: Arc<Mutex<Material>>,
	// pub position: Vector3<T>,
	// pub rotation: Vector3<T>,
	// pub scale: Vector3<T>,
	// quaternion
}


impl Mesh {
	pub fn new(geometry: BufferGeometry, material: Arc<Mutex<Material>>) -> Mesh
	// where Material: 'static
	{
		Mesh {
			uuid: Uuid::new_v4(),
			name: "".to_string(),
			geometry,
			material,
		}
	}
}

impl Component for Mesh {
    type Storage = VecStorage<Self>;
}

// impl <'a, M> Node for Mesh<'a, M>
// where
// M: Material,
// // N: Node
// {}

// impl <'a, M> Mesh<'a, M>

// impl Component for Mesh {
// 	fn test(&self){
// 		println!("1")
// 	}
// }
