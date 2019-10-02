extern crate specs;
use self::specs::{World};
use super::{
	SharedGeometry,
	SharedMaterials,
	Transform,
	PerspectiveCamera,
	PointLight,
	Parent,
	Children,
	DirectionalLight,
    SharedFrameBuffer,
    SharedRenderBuffer
};

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