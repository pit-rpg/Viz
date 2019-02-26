extern crate specs;


use core::{Transform};
use self::specs::{ReadStorage, System, Write, WriteStorage, Entity, Join, World};


pub struct SystemTransform {}


impl SystemTransform {
	pub fn new() -> Self {
		Self {}
	}
}


impl<'a> System<'a> for SystemTransform {
	type SystemData = (
		WriteStorage<'a, Transform>,
	);

	fn run(&mut self, data: Self::SystemData) {

		let (
			mut transform_coll,
		) = data;


		for transform in (&mut transform_coll).join() {
			transform.update();
		}
	}
}
