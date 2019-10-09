extern crate specs;

use specs::prelude::*;
use specs::error::WrongGeneration;

pub struct Parent {
	pub entity: Entity,
}
pub struct Children {
	pub children: Vec<Entity>,
}

impl Component for Parent {
	type Storage = VecStorage<Self>;
}

impl Component for Children {
	type Storage = VecStorage<Self>;
}

pub trait EntityRelations {
	fn add_children(&mut self, parent: Entity, children: Vec<Entity>);
	fn add_child(&mut self, parent: Entity, child: Entity);
	fn remove_child(&mut self, parent: Entity, child: Entity);
	fn remove_children(&mut self, parent: Entity, children: &mut Vec<Entity>);
	fn get_all_children_entities_to(&mut self, elem: Entity, children: &mut Vec<Entity>);
	fn get_all_children_entities(&mut self, elem: Entity) -> Vec<Entity>;
	fn remove_recursive(&mut self, elem: Entity) -> Result<(), WrongGeneration>;
}

impl EntityRelations for World {
	fn add_children(&mut self, parent: Entity, mut children: Vec<Entity>) {
		let mut store_parent = self.write_storage::<Parent>();
		let mut store_children = self.write_storage::<Children>();

		children.iter().for_each(|elem| {
			if let Some(parent_link) = store_parent.get_mut(*elem) {
				if let Some(childs) = store_children.get_mut(parent_link.entity) {
					childs
						.children
						.iter()
						.position(|e| e == elem)
						.and_then(|i| Some(childs.children.remove(i)));
				}
			}
		});

		if let Some(child_collection) = store_children.get_mut(parent) {
			child_collection.children.append(&mut children);
			return;
		}

		let new_child = Children { children };
		store_children.insert(parent, new_child).unwrap();
	}

	fn add_child(&mut self, parent: Entity, child: Entity) {
		let mut store_parent = self.write_storage::<Parent>();
		let mut store_children = self.write_storage::<Children>();

		if let Some(parent_link) = store_parent.get_mut(child) {
			if let Some(childs) = store_children.get_mut(parent_link.entity) {
				childs
					.children
					.iter()
					.position(|e| e == &child)
					.and_then(|i| Some(childs.children.remove(i)));
			}
		}

		if let Some(child_collection) = store_children.get_mut(parent) {
			child_collection.children.push(child);
			return;
		}

		let new_child = Children {
			children: vec![child],
		};
		store_children.insert(parent, new_child).unwrap();
	}

	fn remove_child(&mut self, parent: Entity, child: Entity) {
		let mut store_parent = self.write_storage::<Parent>();
		let mut store_children = self.write_storage::<Children>();

		store_parent.remove(child);

		if let Some(child_collection) = store_children.get_mut(parent) {
			child_collection
				.children
				.iter()
				.position(|e| e == &child)
				.and_then(|i| Some(child_collection.children.remove(i)));
		}
	}

	fn remove_children(&mut self, parent: Entity, children: &mut Vec<Entity>) {
		let mut store_parent = self.write_storage::<Parent>();
		let mut store_children = self.write_storage::<Children>();

		children.iter().for_each(|elem| {
			store_parent.remove(*elem);
		});

		if let Some(child_collection) = store_children.get_mut(parent) {
			children.iter().for_each(|elem| {
				child_collection
					.children
					.iter()
					.position(|e| e == elem)
					.and_then(|i| Some(child_collection.children.remove(i)));
			});
		}
	}

	fn get_all_children_entities_to(&mut self, elem: Entity, res: &mut Vec<Entity>) {
		let store_children = self.read_storage::<Children>();
		let mut temp1 = vec![elem];
		let mut temp2 = vec![];

		while temp1.len() > 0 {
			for e in temp1.iter() {
				if let Some(ch) = store_children.get(*e) {
					temp2.append(&mut ch.children.clone());
				}
			}
			res.append(&mut temp1);
			temp1.append(&mut temp2);
		}
	}

	fn get_all_children_entities(&mut self, elem: Entity) -> Vec<Entity> {
		let mut res = vec![];
		self.get_all_children_entities_to(elem, &mut res);
		res
	}

	fn remove_recursive(&mut self, elem: Entity) -> Result<(), WrongGeneration> {
		let mut items = self.get_all_children_entities(elem);
		items.push(elem);
		self.delete_entities(&items)
	}
}
