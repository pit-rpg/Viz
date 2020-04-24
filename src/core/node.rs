extern crate specs;
extern crate uuid;

use self::uuid::Uuid;
use super::{Context, Transform};
use specs::prelude::*;
use std::cell::*;
use std::rc::*;

#[derive(Debug)]
struct NodeBase {
	_uuid: Uuid,
	parent: Option<Node>,
	children: Vec<Node>,
	_entity: Entity,
	pub transform: Transform,
	pub name: String,
}

#[derive(Debug, Clone)]
pub struct Node(Rc<RefCell<NodeBase>>);

impl NodeBase {
	pub fn new(name: &str, transform: Transform, entity: Entity) -> Self {
		Self {
			_uuid: Uuid::new_v4(),
			parent: None,
			children: vec![],
			_entity: entity,
			transform,
			name: name.to_string(),
		}
	}

	pub fn uuid(&self) -> Uuid {
		self._uuid
	}

	pub fn entity(&self) -> Entity {
		self._entity
	}

	pub fn get_data(&self) -> (&str, &Entity, &Transform) {
		(&self.name, &self._entity, &self.transform)
	}

	pub fn get_data_mut(&mut self) -> (&str, &Entity, &mut Transform) {
		(&self.name, &self._entity, &mut self.transform)
	}
}

impl Node {
	pub fn new(name: &str, transform: Transform, entity: Entity) -> Node {
		let node_base = NodeBase::new(name, transform, entity);
		Node(Rc::new(RefCell::new(node_base)))
	}

	pub fn add_child(&self, node: &Node) {
		{
			node.0.borrow_mut().parent = Some(Node(Rc::clone(&self.0)));
		}
		let mut nb = self.0.borrow_mut();
		nb.children.push(node.clone());
	}

	pub fn remove_child(&self, node: &Node) -> bool {
		{
			node.0.borrow_mut().parent = None;
		}
		let mut nb = self.0.borrow_mut();
		let uuid = nb.uuid();
		let index = nb.children.iter().position(|e| uuid == e.uuid());
		if let Some(index) = index {
			nb.children.swap_remove(index);
			return true;
		}
		false
	}

	pub fn set_parent(&self, node: &Node) {
		node.add_child(self);
	}

	pub fn clear_children(&self) {
		self.0.borrow_mut().children.clear();
	}

	pub fn uuid(&self) -> Uuid {
		self.0.borrow().uuid()
	}

	// pub fn build_child(&mut self, name: &str, context: &mut Context) -> Node {
	// 	let entity = context.getWorld().create_entity().build();
	// 	let node = Node::new(name, Transform::default(), entity);
	// 	self.add_child(&node);
	// 	node
	// }

	pub fn traverse<F: Fn(&Node, usize)>(&self, func: F) {
		self.traverse_helper(&func, 0);
	}

	fn traverse_helper<F: Fn(&Node, usize)>(&self, func: &F, depth: usize) {
		func(&self, depth);
		let node_base = self.0.borrow_mut();
		for node in node_base.children.iter() {
			node.traverse_helper(func, depth + 1);
		}
	}
}

#[cfg(test)]
mod tests {
	extern crate specs;
	extern crate uuid;

	use core::{create_world, Node, Transform};
	// use core::{create_world, Node, Transform, create_context};
	use specs::prelude::*;
	use std::rc::*;

	#[test]
	fn node_clone_test() {
		let mut world = create_world();
		let entity = world.create_entity().build();
		let transform = Transform::default();

		let node1 = Node::new("node", transform, entity);
		{
			let node2 = node1.clone();
			let node3 = node2.clone();

			assert_eq!(Rc::strong_count(&node3.0), 3);
		}
		assert_eq!(Rc::strong_count(&node1.0), 3);
	}

	// #[test]
	// fn node_traverse_test() {
	// 	let mut context = create_context();
	// 	let entity = context.world.create_entity().build();
	// 	let transform = Transform::default();

	// 	let mut root = Node::new("root", transform, entity);
	// 	root.build_child("A", world);
	// }
}
