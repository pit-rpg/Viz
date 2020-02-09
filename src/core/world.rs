extern crate specs;
use self::specs::prelude::*;
use super::{
	Children, DirectionalLight, Parent, PerspectiveCamera, PointLight, SharedFrameBuffer, SharedGeometry, SharedMaterials,
	SharedRenderBuffer, Transform,
};

pub struct Context {
	pub world: World,
}

impl Context {
	// pub fn get_world(&mut self) -> &mut World {
	// 	&mut self._world
	// }

	pub fn create_context() -> Self {
		Context {
			world: create_world()
		}
	}
}

pub fn create_world() -> World {
	let mut world = World::new();
	world.register::<SharedGeometry>();
	world.register::<SharedMaterials>();
	world.register::<Transform>();
	world.register::<PerspectiveCamera>();
	world.register::<SharedFrameBuffer>();
	world.register::<SharedRenderBuffer>();

	world.register::<PointLight>();
	world.register::<DirectionalLight>();

	world.register::<Parent>();
	world.register::<Children>();

	world
}
