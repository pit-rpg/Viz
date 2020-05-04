extern crate uuid;
#[macro_use] extern crate project;


use std::f64::consts::PI as PI_f64;
use std::path::PathBuf;

use project::{
	glutin::{MouseScrollDelta},
	glutin,
	render,
	math::{Vector3, Vector, Vector4},
	core::{
		SharedGeometry, Node, NodeData, PerspectiveCamera, Transform,
		Material, Light,
	},
	helpers::{
		load_gltf,
		geometry_generators,
	},
};


#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WindowState {
	pub pointer_pos: (f64, f64),
	pub pointer_pressed: (bool, bool, bool),
	pub pointer_wheel: f32,
	pub window_size: (f64, f64),
}


fn main(){
	let root = Node::new("root");

	let up = Vector3::new_up();
	let center = Vector3::new_zero();
	let mut radius = 20.0;
	let zoom_speed = 0.5;
	let mut running = true;

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

	let geom_light = SharedGeometry::new(geometry_generators::sphere(0.5, 12, 12));

	{
		let node = load_gltf(PathBuf::from("models/girl_speedsculpt/scene.gltf"), "girl").unwrap();
		{
			let mut node_data = node.lock();
			node_data.transform.position.y += 2.2;
			node_data.transform.position.x -= 2.0;
			node_data.transform.scale.set_scalar(0.4);
		}
		root.add_child(node.clone());
	}

	{
		let node = load_gltf(PathBuf::from("models/Duck.glb"), "duck").unwrap();
		root.add_child(node);
	}

	{
		let node = load_gltf(PathBuf::from("models/pony_cartoon/scene.gltf"), "pony").unwrap();
		{
			let mut node_data = node.lock();
			node_data.transform.scale.set_scalar(0.02);
			node_data.transform.position.y -= 5.0;
			node_data.transform.position.x += 5.0;
		}
		root.add_child(node);

	}

	let lights_parent = root.add_child(NodeData::new("lights_parent").to_shared());
	// let mut lights = Vec::new();

	for _ in  0..5 {
		let mut transform = Transform::default();
		transform.scale.set(0.2,0.2,0.2);
		transform.position
			.randomize()
			.multiply_scalar(40.0)
			.sub_scalar(20.0);

		let color = Vector3::random();
		let point_light = Light::new(color.clone(), 1.0);
		// let point_light = Light::new(color.clone(), 1.0, 200.0, 1.0);

		let material_light = Material::new_basic(Vector4::new(color.x,color.y,color.z,5.0)).to_shared();

		let node = NodeData::new("light")
			.set_transform(transform)
			.set_light(point_light)
			.set_geometry(geom_light.clone())
			.set_material(material_light.clone())
			.to_shared();

		lights_parent.add_child(node.clone());
		// lights.push(node);
	}

	{
		let mut transform = Transform::default();
		transform.rotation.x = 3.14/2.0;

		let color = Vector3::new_one();
		let material_light = Material::new_basic(Vector4::new(color.x,color.y,color.z,5.0)).to_shared();
		let light = Light::new(color.clone(), 1.0);

		root.add_child(
			NodeData::new("light")
				.set_transform(transform)
				.set_light(light.clone())
				.set_material(material_light.clone())
				.set_geometry(geom_light.clone())
				.to_shared()
		);
	}

	let mut render_system = render::open_gl::system_render::RenderSystem::new(camera.clone(), true, true, true);

	render_system.windowed_context.window().set_resizable(true);
	let hidpi_factor = render_system.windowed_context.window().get_hidpi_factor().round();
	let mut window_state = WindowState::default();

	while running {

		{
			let windowed_context = &render_system.windowed_context;
			use self::glutin::WindowEvent::*;

			render_system.events_loop.poll_events(|event| {
				match event {
					glutin::Event::WindowEvent{ event, .. } => match event {
						glutin::WindowEvent::CloseRequested => running = false,
						glutin::WindowEvent::Resized(logical_size) => {
							window_state.window_size.0 = logical_size.width;
							window_state.window_size.1 = logical_size.height;

							let dpi_factor = windowed_context.window().get_hidpi_factor();
							windowed_context.resize(logical_size.to_physical(dpi_factor));

							gl_call!({
								gl::Viewport(0,0, (logical_size.width * dpi_factor) as i32, (logical_size.height * dpi_factor) as i32);
							});
							println!("logical_size: {:?}, dpi_factor: {:?}", logical_size, dpi_factor);
						},
						glutin::WindowEvent::MouseWheel{ delta, .. } => {
							match delta {
								MouseScrollDelta::LineDelta(_, y) => {
									if y > 0.0 { radius -= zoom_speed } else {radius += zoom_speed};
								}
								MouseScrollDelta::PixelDelta(_) => {}
							}
						}
						CursorMoved { position: pos, .. } =>{
							window_state.pointer_pos = pos
								.to_physical(windowed_context.window().get_hidpi_factor())
								.to_logical(hidpi_factor)
								.into();
						}
						_ => ()
					},
					_ => ()
				}
			});
		}

		let time = render_system.get_duration();

		{
			let mut node_data = camera.lock();
			{
				let transform_camera = &mut node_data.transform;
				let x_prog = window_state.pointer_pos.0 / window_state.window_size.0;
				let y_prog = window_state.pointer_pos.1 / window_state.window_size.1;
				transform_camera.position.z = ((x_prog * (PI_f64 * 2.0)).sin() * radius) as f32;
				transform_camera.position.x = ((x_prog * (PI_f64 * 2.0)).cos() * radius) as f32;
				transform_camera.position.y = ((y_prog * radius - radius / 2.0) * -2.0) as f32;
				transform_camera.look_at(&center, &up);
			}

			if let Some(camera) = &mut node_data.camera {
				let aspect = window_state.window_size.0 / window_state.window_size.1;
				camera.aspect = aspect as f32;
				camera.update_projection_matrix();
			}
		}

		{
			let mut node_data = lights_parent.lock();
			node_data.transform.rotation.y = time * 0.5;
			node_data.transform.rotation.x = time * 0.3;
			node_data.transform.rotation.z = time * 0.1;
		}

		render_system.run(&root);
	}
}
