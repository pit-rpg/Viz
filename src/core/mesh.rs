extern crate uuid;

// use std::sync::RwLock;
// use std::sync::{Arc, Mutex};

use self::uuid::Uuid;
// use super::BufferGeometry;
use super::Component;
// use super::Material;
// use math::Vector3;

// pub trait Mesh {}

#[allow(dead_code)]
pub struct Mesh {
	pub uuid: Uuid,
	pub name: String,
	// pub geometry: &'a BufferGeometry,
	// pub material: &'a M,
	// pub children: Vec<Node<T>>,
	// pub attachment: A,
	// pub children: Vec<Arc<Mutex<Node>>>,
	// pub position: Vector3<T>,
	// pub rotation: Vector3<T>,
	// pub scale: Vector3<T>,
	// quaternion
}

// impl <'a, M> Node for Mesh<'a, M>
// where
// M: Material,
// // N: Node
// {}

// impl <'a, M> Mesh<'a, M>

impl Component for Mesh {}
