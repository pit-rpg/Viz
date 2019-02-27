extern crate specs;


use core::{Transform};
// use core::{Transform, Relation};
use self::specs::{ReadStorage, System, Write, WriteStorage, Entity, Join, World};


pub struct SystemTransform {}


impl SystemTransform {
	pub fn new() -> Self {
		Self {}
	}
}


// fn recursive_update(t: &mut Transform, r: &Relation) {
// 	t.update();
// }


impl<'a> System<'a> for SystemTransform {
	type SystemData = (
		WriteStorage<'a, Transform>,
		// ReadStorage<'a, Relation>,
	);

	fn run(&mut self, data: Self::SystemData) {

		let (
			mut transform_coll,
			// mut relation_coll,
		) = data;


		// for (transform, _) in (&mut transform_coll, !&relation_coll).join() {
		// 	transform.update();
		// }
		// for (transform, relation) in (&mut transform_coll, &relation_coll).join().filter(|(t,r)| r.get_parent() == &None) {
		// 	transform.update();

		// }
	}
}
