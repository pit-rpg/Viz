use uuid::Uuid;
use super::{Transform, SharedMaterial, SharedGeometry, Light, PerspectiveCamera};
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub struct NodeData {
	_uuid: Uuid,
	parent: Option<Node>,
	children: Vec<Node>,
	pub transform: Transform,
	pub name: String,
	pub materials: Vec<SharedMaterial>,
	pub geometry: Option<SharedGeometry>,
	pub light: Option<Light>,
	pub camera: Option<PerspectiveCamera>,
}

#[derive(Debug, Clone)]
pub struct Node (Uuid, Arc<Mutex<NodeData>>);

impl NodeData {
	pub fn new(name: &str) -> Self {
		Self {
			_uuid: Uuid::new_v4(),
			parent: None,
			children: vec![],
			transform: Transform::default(),
			name: name.to_string(),
			materials: vec![],
			geometry: None,
			light: None,
			camera: None,
		}
	}

	pub fn set_name(mut self, name: &str) -> Self {
		self.name = name.to_string();
		self
	}

	pub fn set_transform(mut self, transform: Transform) -> Self {
		self.transform = transform;
		self
	}

	pub fn set_materials(mut self, materials: Vec<SharedMaterial>) -> Self {
		self.materials = materials;
		self
	}

	pub fn set_material(mut self, materials: SharedMaterial) -> Self {
		self.materials.clear();
		self.materials.push(materials);
		self
	}

	pub fn set_geometry(mut self, geometry: SharedGeometry) -> Self {
		self.geometry = Some(geometry);
		self
	}

	pub fn set_light(mut self, light: Light) -> Self {
		self.light = Some(light);
		self
	}

	pub fn set_camera(mut self, camera: PerspectiveCamera) -> Self {
		self.camera = Some(camera);
		self
	}

	pub fn uuid(&self) -> Uuid {
		self._uuid
	}

	pub fn to_shared(self) -> Node {
		Node::from_node_data(self)
	}
}

impl Node {

	pub fn from_node_data(data: NodeData) -> Node {
		Node(data._uuid, Arc::new(Mutex::new(data)))
	}

	pub fn new(name: &str) -> Node {
		let data = NodeData::new(name);
		let uuid = data._uuid;

		Node(uuid, Arc::new(Mutex::new(data)))
	}

	pub fn add_child(&self, node: Node) -> Node {
		{
			let mut base = node.lock();
			if let Some(parent) = &base.parent {
				parent.remove_child(self);
			}
			base.parent = Some(self.clone());
		}
		let mut nb = self.lock();
		nb.children.push(node.clone());
		node
	}

	pub fn remove_child(&self, node: &Node) -> &Self {
		{
			node.lock().parent = None;
		}
		let mut nb = self.lock();
		let uuid = nb.uuid();
		let index = nb.children.iter().position(|e| uuid == e.uuid());
		if let Some(index) = index {
			nb.children.swap_remove(index);
		}
		self
	}

	pub fn set_parent(&self, node: Node) -> &Self {
		node.add_child(self.clone());
		self
	}

	pub fn clear_children(&self) -> &Self {
		self.lock().children.clear();
		self
	}

	pub fn uuid(&self) -> Uuid {
		self.0
	}

	pub fn lock(&self) -> MutexGuard<'_, NodeData> {
		self.1.lock().unwrap()
	}

	pub fn build_child(&mut self, name: &str) -> Node {
		let node = Node::new(name);
		self.add_child(node.clone());
		node
	}

	pub fn traverse<F: FnMut(&Node, usize)>(&self, func: &mut F) {
		self.traverse_helper(func, 0);
	}

	fn traverse_helper<F: FnMut(&Node, usize)>(&self, func: &mut F, depth: usize) {
		func(&self, depth);
		let node_base = self.lock();
		for node in node_base.children.iter() {
			node.traverse_helper(func, depth + 1);
		}
	}

	pub fn update_transform(&self, force: bool) {
		let mut node_data = self.lock();

		if !node_data.transform.auto_update {return}

		node_data.transform.update();

		Self::update_transform_helper(&node_data, force);
	}

	fn update_transform_helper(node: &NodeData, force: bool) {
		node.children
			.iter()
			.for_each(|child| {
				let mut nd = child.lock();
				if !nd.transform.auto_update && !force {return}

				nd.transform.update();
				Self::update_transform_helper(&nd, force);
			});
	}

}

#[cfg(test)]
mod tests {
	extern crate uuid;

	use core::{Node, Transform};
	// use core::{create_world, Node, Transform, create_context};
	use std::rc::*;
	use std::sync::{Arc, LockResult, Mutex, MutexGuard};


	#[test]
	fn node_clone() {
		let transform = Transform::default();

		let node1 = Node::new("node");
		{
			let node2 = node1.clone();
			let node3 = node2.clone();

			assert_eq!(Arc::strong_count(&node3.1), 3);
		}
		assert_eq!(Arc::strong_count(&node1.1), 1);
	}

	#[test]
	fn node_traverse() {
		let mut root = Node::new("root");
		let mut a = root.build_child("a");
		root.build_child("b");
		let mut aa = a.build_child("aa");
		aa.build_child("aaa");

		root.traverse(&mut |node, depth| {
			println!("{}: {}", depth, node.lock().name);
		});
	}

	#[test]
	fn node_traverse_mut() {
		let mut root = Node::new("root");
		let mut a = root.build_child("a");
		root.build_child("b");
		let mut aa = a.build_child("aa");
		aa.build_child("aaa");

		root.traverse(&mut |node, depth| {
			let mut nb = node.lock();
			nb.name += "<><>";
			println!("{}: {}", depth, nb.name);
		});
	}

	#[test]
	fn node_set_parrent() {

		let a = Node::new("a");
		let b = Node::new("b");
		let c = Node::new("c");

		c.set_parent(b.clone());
		b.set_parent(a.clone());

		a.traverse(&mut |node, depth| {
			println!("{}: {}", depth, node.lock().name);
		});
	}

	#[test]
	fn node_remove_child() {

		let mut root = Node::new("a");
		let aaaa = root
			.build_child("a")
			.build_child("aa")
			.build_child("aaa")
			.build_child("aaaa");

		let aaa = {aaaa.lock().parent.clone()};

		if let Some(aaa) = aaa {
			aaa.remove_child(&aaaa);
		}

		root.traverse(&mut |node, depth| {
			println!("{}: {}", depth, node.lock().name);
		});
	}
}
