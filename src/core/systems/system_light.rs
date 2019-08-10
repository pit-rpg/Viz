extern crate gl;
extern crate glutin;
extern crate rand;
extern crate specs;
extern crate uuid;

use core::{
	SharedMaterial,
	Transform,
	PointLight,
	ShaderProgram,
	UniformName
};

use self::specs::{
	ReadStorage,
	System,
	WriteStorage,
	Join
};


pub struct SystemLight {}
impl SystemLight {
	pub fn new() -> Self {
		Self {}
	}
}


impl<'a> System<'a> for SystemLight {
	type SystemData = (
		ReadStorage<'a, Transform>,
		ReadStorage<'a, PointLight>,
		WriteStorage<'a, SharedMaterial>,
	);

	fn run(&mut self, data: Self::SystemData) {

		let (
			transform_coll,
			light_coll,
			mut material_coll,
		) = data;

		let lights: Vec<_> = (&light_coll, &transform_coll).join().take(4).collect();

		for (_, shared_material) in (&transform_coll, &mut material_coll).join() {
			lights.iter().enumerate()
				.for_each(|(i, (light, light_transform))| {
					let mut material = shared_material.lock().unwrap();

					material.set_uniform(UniformName::Other(format!("pointLights[{}].position", i)), light_transform.position.clone());
					material.set_uniform(UniformName::Other(format!("pointLights[{}].color", i)), light.color.clone());
					material.set_uniform(UniformName::Other(format!("pointLights[{}].distance", i)), light.distance);
					material.set_uniform(UniformName::Other(format!("pointLights[{}].decay", i)), light.decay);
				});
		}
	}
}
