extern crate gl;
extern crate glutin;
extern crate rand;
extern crate specs;
extern crate uuid;

// use core::BufferGeometry;
use core::{
	SharedGeometry,
	SharedMaterial,
	PerspectiveCamera,
	Transform,
	PointLight,
	ShaderProgram,
	Uniform,
};
// use core::Material;
// use core::Uniform;
// use core::ShaderProgram;

use self::specs::{ReadStorage, System,  WriteStorage, Join};
// use self::specs::{ReadStorage, System, Write, WriteStorage, Entity, Join, World};
// use self::uuid::Uuid;




pub struct SystemLight {}


impl SystemLight {
	pub fn new() -> Self {
		Self {}
	}
}


impl<'a> System<'a> for SystemLight {
	type SystemData = (
		ReadStorage<'a, PerspectiveCamera>,
		ReadStorage<'a, Transform>,
		ReadStorage<'a, SharedGeometry>,
		ReadStorage<'a, PointLight>,
		WriteStorage<'a, SharedMaterial>,
	);

	fn run(&mut self, data: Self::SystemData) {

		let (
			camera_coll,
			transform_coll,
			geometry_coll,
			light_coll,
			mut material_coll,
		) = data;

		let lights: Vec<_> = (&light_coll, &transform_coll).join().take(4).collect();

		for (transform, shared_material) in (&transform_coll, &mut material_coll).join() {
			lights.iter().enumerate()
				.for_each(|(i, (light, light_transform))| {
					let mut material = shared_material.lock().unwrap();

					material.set_uniform(&format!("pointLights[{}].position", i), &Uniform::Vector3(light_transform.position.clone()));
					material.set_uniform(&format!("pointLights[{}].color", i), &Uniform::Vector3(light.color.clone()));
					material.set_uniform(&format!("pointLights[{}].distance", i), &Uniform::Float(light.distance));
					material.set_uniform(&format!("pointLights[{}].decay", i), &Uniform::Float(light.decay));
				});
		}
	}
}
