extern crate specs;
extern crate rayon;


use core::{
	Transform,
	Parent,
	Children
};
use self::specs::{
	ReadStorage,
	System,
	WriteStorage,
	Entity,
	Join,
	Entities
};


pub struct SystemTransform {}


impl SystemTransform {
	pub fn new() -> Self {
		Self {}
	}
}


impl<'a> System<'a> for SystemTransform {
	type SystemData = (
		Entities<'a>,
		WriteStorage<'a, Transform>,
		ReadStorage<'a, Parent>,
		ReadStorage<'a, Children>,
	);

	fn run(&mut self, data: Self::SystemData) {
		use self::rayon::prelude::*;
		use specs::ParJoin;

		let (
			entities,
			mut transform_coll,
			parent_coll,
			children_coll,
		) = data;


		(&mut transform_coll, !&parent_coll, !&children_coll)
			.par_join()
			.for_each(|(transform, _, _)|  {
				transform.update();
			});

		let mut temp1: Vec<Entity> = Vec::with_capacity(128);
		let mut temp2: Vec<Entity> = Vec::with_capacity(128);

		let mut parent_transform = Transform::default();

		for (_, children, entity) in (!&parent_coll, &children_coll, &entities).join() {
			parent_transform = Transform::default();

			if let Some(t) = transform_coll.get_mut(entity) {
				t.update();
				parent_transform = t.clone();
			}

			temp1.append( &mut children.children.clone() );
			while temp1.len() > 0 {
				for e in temp1.iter() {
					if let Some(ch) = children_coll.get(*e) {
						temp2.append(&mut ch.children.clone());
					}
				}

				temp1.iter()
					.for_each(|e| {
						if let Some(transform) = transform_coll.get_mut(*e) {
							transform.update();
							transform.matrix_world = parent_transform.matrix_world * parent_transform.matrix_local;
						}
					});
				temp1.clear();
				temp1.append(&mut temp2);
			}
		}
	}
}
