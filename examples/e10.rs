extern crate project;

use std::f32::consts::PI;
use std::path::Path;

use project::{
	core::{
		FrameOutput, Material, PerspectiveCamera, NodeData, Node, Light, LightType,
		SharedFrameBuffer, SharedGeometry, Transform, UniformName, SharedTexture2D, BufferType,
	},
	helpers::{geometry_generators, load_gltf, load_obj, DemoRunner},
	math::{Vector, Vector3, Vector4},
	render,
};

fn main() {

	let root = Node::new("root");

	let camera = {
		let mut camera = PerspectiveCamera::new();
		let mut transform_camera = Transform::default();
		transform_camera.position.z = 6.0;
		camera.view.enabled = false;

		root.add_child(
			NodeData::new("camera")
				.set_camera(camera)
				.set_transform(transform_camera)
				.to_shared()
		)
	};


	let path = Path::new("models/Predator.obj");
	let mut objects = load_obj(&path).expect("cant load file");

	let mut mat_cup_mat = Material::new_mat_cup();
	let mat_cup_texture = SharedTexture2D::new_from_path("images/mc4.jpg");
	mat_cup_mat.set_uniform(UniformName::MapColor, mat_cup_texture.clone());
	let shared_mat_cup_mat = mat_cup_mat.to_shared();

	let nodes: Vec<Node> = objects
		.drain(..)
		.map(|mut object|{
			if !object.has_attribute(BufferType::Normal) {
				object.generate_normals();
			}

			let geom = SharedGeometry::new(object);

			let mut transform = Transform::default();
			transform.rotation.y = PI;

			let node = NodeData::new("obj_object")
				.set_transform(transform)
				.set_geometry(geom)
				.set_material(shared_mat_cup_mat.clone())
				.to_shared();

			root.add_child(node.clone());
			node
		})
		.collect();



	DemoRunner::run(move |render_system|{
		render_system.run(&camera, &root);

		for node in &nodes {
			let mut data = node.lock();
			data.transform.rotation.y += 0.05;
		}
	});
}
