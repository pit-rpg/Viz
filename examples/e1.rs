extern crate project;

use project::{
	core::{
		Material, PerspectiveCamera,
		SharedMaterial, SharedTexture2D, Transform, UniformName, Node, NodeData
	},
	glutin,
	helpers::geometry_generators,
	math::{Vector, Vector3, Vector4},
	render,
};

use std::f64::consts::PI as PI_f64;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WindowState {
	pub pointer_pos: (f64, f64),
	pub pointer_pressed: (bool, bool, bool),
	pub pointer_wheel: f32,
	pub window_size: (f64, f64),
}

fn main() {
	let mut f_count = 0.0;
	let up = Vector3::new(0.0, 1.0, 0.0);
	let center = Vector3::new_zero();
	let radius = 10.0;

	let mut color1 = Vector3::<f32>::random();
	let mut color2 = Vector3::<f32>::random();
	let mut color_tmp = Vector3::<f32>::new(color1.x, color1.y, color1.z);

	let mut running = true;

	let geom_container = geometry_generators::box_geometry(1.0, 1.0, 1.0).to_shared();
	let geom_light = geometry_generators::sphere(0.5, 12, 12).to_shared();

	let mut transform_camera = Transform::default();
	transform_camera.position.z = 6.0;
	transform_camera.update();

	let mut transform_light = Transform::default();
	transform_light.scale.set(0.2, 0.2, 0.2);
	transform_light.update();

	let texture2 = SharedTexture2D::new_from_path("images/awesomeface.png");
	let texture_container = SharedTexture2D::new_from_path("images/container2.png");
	let texture_container_specular = SharedTexture2D::new_from_path("images/container2_specular.png");

	let mut material2 = Material::new_basic_texture();
	material2.set_uniform(UniformName::MapColor, texture2.clone());
	let material2 = SharedMaterial::new(material2);

	let root = Node::new("root");
	let camera_node = NodeData::new("container")
		.set_transform(transform_camera)
		.set_camera(PerspectiveCamera::new())
		.to_shared();

	root.add_child(camera_node.clone());


	let e2 = NodeData::new("container")
		.set_material(material2)
		.set_geometry(geom_container.clone())
		.to_shared();
	root.add_child(e2.clone());

	root.add_child(
		NodeData::new("container")
			.set_material(Material::new_basic(Vector4::new(1.0, 1.0, 1.0, 1.0)).to_shared())
			.set_geometry(geom_light.clone())
			.set_transform(transform_light.clone())
			.to_shared()
	);


	let mut boxes = Vec::new();
	{
		let box_mat2 = Material::new_light(
			Vector4::new(1.0, 0.5, 0.31, 1.0),
			Vector3::new_one(),
			transform_light.position.clone(),
		).to_shared();

		let box_phong = Material::new_phong(
			Vector4::new(0.46, 0.46, 1.0, 1.0),
			Vector3::new_one(),
			transform_light.position.clone(),
		).to_shared();

		let mut material_sphere = Material::new_light_texture(
			Vector4::new(1.0, 0.5, 0.31, 1.0),
			Vector3::new_one(),
			transform_light.position.clone(),
		);
		material_sphere.set_uniform(UniformName::MapColor, texture_container);
		material_sphere.set_uniform(UniformName::MapSpecular, texture_container_specular);
		let box_mat = SharedMaterial::new(material_sphere);

		let count = 1000;
		for i in 0..1000 {
			let mut transform = Transform::default();
			transform.scale.set(0.4, 0.4, 0.4);
			transform
				.position
				.randomize()
				.multiply_scalar(10.0)
				.sub_scalar(5.0);

			let mat;
			let geom;

			if i < count / 3 {
				mat = box_mat.clone();
			} else if i < count / 3 * 2 {
				mat = box_phong.clone();
			} else {
				mat = box_mat2.clone();
			}

			if i % 2 == 0 {
				geom = geom_container.clone();
			} else {
				geom = geom_light.clone();
			}

			let node = NodeData::new("item")
				.set_material(mat)
				.set_geometry(geom.clone())
				.set_transform(transform)
				.to_shared();

			root.add_child(node.clone());
			boxes.push(node);
		}
	}

	let mut render_system = render::open_gl::system_render::RenderSystem::new(camera_node.clone(), true, true, true);

	render_system.windowed_context.window().set_resizable(true);

	let hidpi_factor = render_system
		.windowed_context
		.window()
		.get_hidpi_factor()
		.round();
	let mut window_state = WindowState::default();

	while running {
		{
			let windowed_context = &render_system.windowed_context;
			use self::glutin::WindowEvent::*;

			render_system.events_loop.poll_events(|event| match event {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::CloseRequested => running = false,
					glutin::WindowEvent::Resized(logical_size) => {
						let dpi_factor = windowed_context.window().get_hidpi_factor();
						windowed_context.resize(logical_size.to_physical(dpi_factor));
						window_state.window_size.0 = logical_size.width;
						window_state.window_size.1 = logical_size.height;
					}
					CursorMoved { position: pos, .. } => {
						window_state.pointer_pos = pos
							.to_physical(windowed_context.window().get_hidpi_factor())
							.to_logical(hidpi_factor)
							.into();
					}
					_ => (),
				},
				_ => (),
			});
		}

		f_count += 0.01;

		if f_count > 1.0 {
			color1.copy(&color2);
			color2 = Vector3::random();
			f_count = 0.0;
		}

		color_tmp.copy(&color1);
		color_tmp.lerp(&color2, f_count);

		if let Some(color) = &mut render_system.clear_color {
			color.from_vector3(&color_tmp, 1.0);
		}

		{
			let mut node_data = e2.lock();
			node_data.transform.rotation.y += 0.01;
			node_data.transform.rotation.z += 0.01;
			node_data.transform.position.x += 0.001;
			node_data.transform.position.y += 0.001;
			node_data.transform.position.z -= 0.01;
		}

		{
			for m_box in boxes.iter() {
				let mut node_data = m_box.lock();
				let mut transform = &mut node_data.transform;

				if transform.scale.z == 0.5 {
					if render_system.get_duration() < 10.0 {
						transform.rotation.x -= 0.001;
						transform.rotation.y -= 0.002;
						transform.rotation.z -= 0.003;
					}
				} else {
					transform.rotation.x += 0.01;
					transform.rotation.y += 0.02;
					transform.rotation.z += 0.03;
				}
			}
		}

		{
			let mut node_data = camera_node.lock();
			let mut transform = &mut node_data.transform;

			let x_prog = window_state.pointer_pos.0 / window_state.window_size.0;
			let y_prog = window_state.pointer_pos.1 / window_state.window_size.1;

			transform.position.z = ((x_prog * (PI_f64 * 2.0)).sin() * radius) as f32;
			transform.position.x = ((x_prog * (PI_f64 * 2.0)).cos() * radius) as f32;
			transform.position.y = ((y_prog * radius - radius / 2.0) * -2.0) as f32;
			transform.look_at(&center, &up);
		}

		render_system.run(&root);
	}
}
