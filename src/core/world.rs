extern crate specs;
use self::specs::{World, RunNow, Builder};
use super::{
	SharedGeometry,
	SharedMaterial,
	Transform,
	PerspectiveCamera,
	PointLight,
	Parent,
	Children,
	// Relation,
};

pub fn create_world() -> World {
	let mut world = World::new();
	world.register::<SharedGeometry>();
	world.register::<SharedMaterial>();
	world.register::<Transform>();
	world.register::<PerspectiveCamera>();
	world.register::<PointLight>();
	world.register::<Parent>();
	world.register::<Children>();
	// world.register::<Relation>();
	world
}