extern crate uuid;

use std::cell::{ RefCell };
// use std::cell::{Cell, RefCell};
// use std::sync::{Arc, Mutex, RwLock};

use self::uuid::Uuid;
use helpers::Nums;
use math::{Vector3, Vector};

pub trait Component {}

#[allow(dead_code)]
pub struct Node<T>
where
	T: Nums,
{
	pub uuid: Uuid,
	pub name: String,
	pub children: Vec<Node<T>>,
	pub components: Vec<RefCell<Box<Component>>>,
	pub position: Vector3<T>,
	pub rotation: Vector3<T>,
	pub scale: Vector3<T>,
	// quaternion
}


#[allow(dead_code)]
impl<T> Node<T>
where
	T: Nums,
{
	pub fn new() -> Node<T> {
		Node {
			uuid: Uuid::new_v4(),
			name: "".to_string(),
			children: Vec::new(),
			components: Vec::new(),
			position: Vector3::new_zero(),
			rotation: Vector3::new_zero(),
			scale: Vector3::new_one(),
		}
	}
}
