extern crate uuid;
#[macro_use] extern crate project;


use std::f64::consts::PI as PI_f64;
use std::path::Path;

use project::{
	specs::*,
	glutin::{MouseScrollDelta},
	glutin,
	render,
	math::{Vector3, Vector, Vector4},
	core::{SharedGeometry,
		PerspectiveCamera,
		Transform,
		SharedTexture2D,
		Material,
		SharedMaterial,
		create_world,
		ShaderProgram,
		PointLight,
		SystemTransform,
		BufferType,
		UniformName,
	},
	helpers::{load_obj, geometry_generators, Nums},
};


#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WindowState {
	pub pointer_pos: (f64, f64),
	pub pointer_pressed: (bool, bool, bool),
	pub pointer_wheel: f32,
	pub window_size: (f64, f64),
}


fn main(){

	let mut world = create_world();
	let mut render_system = render::open_gl::system_render::RenderSystem::new(&mut world);
	let mut system_transform = SystemTransform::new();

	let up = Vector3::new(0.0, 1.0, 0.0);
	let center = Vector3::new_zero();
	let mut radius = 10.0;
	let zoom_speed = 0.5;
	let mut running = true;
	let lights_count = 4;




	let mut camera = PerspectiveCamera::new();
	let mut transform_camera = Transform::default();
	transform_camera.position.z = 6.0;
	camera.view.enabled = false;

	let geom_light = SharedGeometry::new(geometry_generators::sphere(0.5, 12, 12));


	let e_cam = world
		.create_entity()
		.with(transform_camera)
		.with(camera)
		.build();


	let path = Path::new("models/Predator.obj");
	let objects = load_obj(&path).expect("cant load file");

	let mut test_mat = Material::new_mesh_phong();
	let mat_cup_texture = SharedTexture2D::new_from_path("images/mc4.jpg");
	test_mat.set_uniform(UniformName::MapColor, mat_cup_texture.clone());
	let shared_test_mat = SharedMaterial::new(test_mat);

	for mut object in objects {

		if !object.has_attribute(BufferType::Normal) {
			object.generate_normals();
		}

		let geom = SharedGeometry::new(object);

		let mut transform = Transform::default();

		let mut mat = shared_test_mat.clone();
		{
			let mut material = mat.lock().unwrap();
			material.set_uniform(UniformName::Color, Vector3::new_one());
			material.set_uniform(UniformName::Specular, Vector3::new_one());
			material.set_uniform(UniformName::Shininess, 1.0);
			material.set_uniform(UniformName::SpecularStrength, 1.0);
		}

		world
			.create_entity()
			.with(transform)
			.with(geom)
			.with(mat)
			.build();
	}


	let mut lights = Vec::new();
	for _ in  0..lights_count {
		let mut transform = Transform::default();
		transform.scale.set(0.2,0.2,0.2);
		transform.position
			.randomize()
			.multiply_scalar(5.0)
			.sub_scalar(2.5);

		let mut color = Vector3::random();
		let point_light = PointLight::new(color.clone(), 1.0, 10.0, 1.0);

		let material_light = SharedMaterial::new(Material::new_basic(Vector4::new(color.x,color.y,color.z,1.0)));

		let e_light = world
			.create_entity()
			.with(transform)
			.with(geom_light.clone())
			.with(material_light.clone())
			.with(point_light.clone())
			.build();

		lights.push(e_light);
	}

	render_system.camera = Some(e_cam);
	render_system.windowed_context.window().set_resizable(true);
	let hidpi_factor = render_system.windowed_context.window().get_hidpi_factor().round();
	let mut window_state = WindowState::default();

	let mut prev_time = 0.0;

	while running {

		{
			let windowed_context = &render_system.windowed_context;
			use self::glutin::WindowEvent::*;

			render_system.events_loop.poll_events(|event| {
				match event {
					glutin::Event::WindowEvent{ event, .. } => match event {
						glutin::WindowEvent::CloseRequested => running = false,
						glutin::WindowEvent::Resized(logical_size) => {
							println!("{:?}", logical_size);
							window_state.window_size.0 = logical_size.width;
							window_state.window_size.1 = logical_size.height;

							gl_call!({
								gl::Viewport(0,0, logical_size.width as i32, logical_size.height as i32);
							});
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

		{
			let mut transform_store = world.write_storage::<Transform>();
			let mut cam_store = world.write_storage::<PerspectiveCamera>();

			{
				let transform_camera = transform_store.get_mut(e_cam).unwrap();
				let aspect = window_state.window_size.0/window_state.window_size.1;

				let  camera = cam_store.get_mut(e_cam).unwrap();
				camera.aspect = aspect as f32;
				camera.update_projection_matrix();

				let x_prog = window_state.pointer_pos.0 / window_state.window_size.0;
				let y_prog = window_state.pointer_pos.1 / window_state.window_size.1;
				transform_camera.position.z = ( (x_prog * (PI_f64*2.0)).sin() * radius ) as f32;
				transform_camera.position.x = ( (x_prog * (PI_f64*2.0)).cos() * radius ) as f32;;
				transform_camera.position.y = (( y_prog * radius - radius/2.0) * -2.0) as f32;
				transform_camera.look_at(&center, &up);
			}
		}

		let time = render_system.get_duration();

		if time - prev_time > 0.1 {
			prev_time = time;

			let mut transform_store = world.write_storage::<Transform>();
			let mut light_store = world.write_storage::<PointLight>();

			for e_light in &lights {
				if let Some(transform) = transform_store.get_mut(*e_light) {
					transform.position
						.randomize()
						.sub_scalar(0.5)
						.multiply_scalar(7.0);
				}
				if let Some(light) = light_store.get_mut(*e_light) {
					light.color.randomize();
					light.decay = f32::random();
				}
			}
		}

		system_transform.run_now(&world.res);
		render_system.run_now(&world.res);
	}
}
