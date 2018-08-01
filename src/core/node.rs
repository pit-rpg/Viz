extern crate uuid;

use std::cell::{ RefCell, RefMut };
// use std::cell::{Cell, RefCell};
// use std::sync::{Arc, Mutex, RwLock};

use self::uuid::Uuid;
use helpers::Nums;
use math::{Vector3, Vector};

pub trait Component
// where Self: Sized
{}

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
	// Node<T>: 'a
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

	pub fn add_component<C> (&mut self, component: C)
	where
		C: Component+'static,
		Node<T>: 'static
	{
		self.components.push(RefCell::new(Box::new(component)));
	}

	pub fn traverse(&mut self, func: fn(node: &mut Self) ) {
		func(self);

		for node in &mut self.children {
			node.traverse(func);
		}
	}
}
