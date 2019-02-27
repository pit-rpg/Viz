

extern crate specs;
// use self::specs::{ReadStorage, System, Write, WriteStorage, Entity, Join, World};
use self::specs::{
	Component,
	VecStorage,
	Entity,
	World,
	WriteStorage,
};

// use math::{
// 	Matrix4,
// 	Vector3,
// 	Vector,
// 	Euler,
// 	Quaternion,
// };


pub struct Relation {
	parent: Option<Entity>,
	children: Vec<Entity>,
}

impl Relation {
	pub fn new() -> Self { Self::default() }
	pub fn get_parent(&self) -> &Option<Entity> { &self.parent }
	pub fn get_children(&self) -> &[Entity] { &self.children[..] }
	pub fn get_children_mut(&mut self) -> &mut [Entity] { &mut self.children[..] }
}



impl Default for Relation {
	fn default() -> Self {
		Self {
			parent: None,
			children: Vec::new(),
		}
	}
}

impl Component for Relation {
	type Storage = VecStorage<Self>;
}


pub trait EntityRelations {
	fn add_child(&mut self, parent: Entity, child: Entity);
	fn add_children(&mut self, parent: Entity, children: &mut Vec<Entity>);
	fn remove_child(&mut self, parent: Entity, child: Entity);
	fn remove_children(&mut self, parent: Entity, children: &mut Vec<Entity>);
}


fn ensure_relation_mut<'a>(store: &'a mut WriteStorage<Relation>, entity: Entity) -> &'a mut Relation {
	if store.get_mut(entity).is_none() {
		let relation = Relation::new();
		store.insert(entity, relation).unwrap();
	}

	store.get_mut(entity).unwrap()
}


impl EntityRelations for World {

	fn add_child(&mut self, parent: Entity, child: Entity) {
		let mut prev_parent = None;
		let mut store = self.write_storage::<Relation>();
		{
			let p_rel = ensure_relation_mut(&mut store, parent);
			p_rel.children.push(child);
		}

		{
			let c_rel = ensure_relation_mut(&mut store, child);
			prev_parent = c_rel.parent;
			c_rel.parent = Some(parent);
		}

		if let Some (parent) = prev_parent {
			let p_rel = ensure_relation_mut(&mut store, child);
			p_rel.children.iter().position(|e| *e == parent).and_then(|i| Some(p_rel.children.remove(i)) );
		}
	}


	fn add_children(&mut self, parent: Entity, children: &mut Vec<Entity>) {
		let mut prev_parent = None;
		let mut store = self.write_storage::<Relation>();

		for child in children.iter_mut() {
			{
				let c_rel = ensure_relation_mut(&mut store, *child);
				prev_parent = c_rel.parent;
				c_rel.parent = Some(parent);
			}

			if let Some (parent) = prev_parent {
				let p_rel = ensure_relation_mut(&mut store, *child);
				p_rel.children.iter().position(|e| *e == parent).and_then(|i| Some(p_rel.children.remove(i)) );
			}
		}

		{
			let p_rel = ensure_relation_mut(&mut store, parent);
			p_rel.children.append(children);
		}
	}


	fn remove_child(&mut self, parent: Entity, child: Entity) {
		let mut store = self.write_storage::<Relation>();
		{
			let p_rel = ensure_relation_mut(&mut store, parent);
			p_rel.children.iter().position(|e| *e == child).and_then(|i| Some(p_rel.children.remove(i)) );
		}

		{
			let c_rel = ensure_relation_mut(&mut store, child);
			c_rel.parent = None;
		}
	}


	fn remove_children(&mut self, parent: Entity, children: &mut Vec<Entity>) {
		let mut store = self.write_storage::<Relation>();
		for child in children.iter_mut() {
			{
				let p_rel = ensure_relation_mut(&mut store, parent);
				p_rel.children.iter().position(|e| e == child).and_then(|i| Some(p_rel.children.remove(i)) );
			}

			{
				let c_rel = ensure_relation_mut(&mut store, *child);
				c_rel.parent = None;
			}
		}
	}
}