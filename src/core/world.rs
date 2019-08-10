extern crate specs;
use self::specs::{World};
use super::{
	SharedGeometry,
	SharedMaterial,
	Transform,
	PerspectiveCamera,
	PointLight,
	Parent,
	Children,
	DirectionalLight,
};

pub fn create_world() -> World {
	let mut world = World::new();
	world.register::<SharedGeometry>();
	world.register::<SharedMaterial>();
	world.register::<Transform>();
	world.register::<PerspectiveCamera>();

	world.register::<PointLight>();
	world.register::<DirectionalLight>();

	world.register::<Parent>();
	world.register::<Children>();

	world
}