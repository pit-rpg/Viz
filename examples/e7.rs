extern crate uuid;
#[macro_use] extern crate project;


use std::f64::consts::PI as PI_f64;
use std::path::Path;
use std::path::PathBuf;

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
		Uniform,
		create_world,
		ShaderProgram,
		PointLight,
		DirectionalLight,
		SystemTransform,
		Parent,
		EntityRelations,
		ShaderTag,
	},
	helpers::{
		load_obj,
		load_gltf,
		geometry_generators,
		Nums
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

	let mut world = create_world();
	let mut render_system = render::open_gl::system_render::RenderSystem::new(&mut world);
	let mut system_transform = SystemTransform::new();


	gl_call!({
		gl::Enable(gl::DEPTH_TEST);
		gl::Enable(gl::BLEND);
		gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
	});

	let up = Vector3::new(0.0, 1.0, 0.0);
	let center = Vector3::new_zero();
	let mut radius = 20.0;
	let zoom_speed = 0.5;
	let mut running = true;

	let mut camera = PerspectiveCamera::new();
	let mut transform_camera = Transform::default();
	transform_camera.position.z = 6.0;
	camera.view.enabled = false;

	let geom_light = SharedGeometry::new(geometry_generators::sphere(0.5, 12, 12));
	let geom_box = SharedGeometry::new(geometry_generators::box_geometry(2.0, 2.0, 2.0));

	let e_cam = world
		.create_entity()
		.with(transform_camera)
		.with(camera)
		.build();

	//////////////////////////////////////////////////////////////////////////////////
	////////////// lights ////////////////////////////////////////////////////////////
	//////////////////////////////////////////////////////////////////////////////////
	let lights_parent = world
		.create_entity()
		.with(Transform::default())
		.build();


	let mut lights = Vec::new();
	for _ in  0..5 {
		let mut transform = Transform::default();
		transform.scale.set(0.2,0.2,0.2);
		transform.position
			.randomize()
			.multiply_scalar(40.0)
			.sub_scalar(20.0);

		let mut color = Vector3::random();
		let point_light = PointLight::new(color.clone(), 1.0, 200.0, 1.0);

		let material_light = SharedMaterial::new(Material::new_basic(&Vector4::new(color.x,color.y,color.z,5.0)));

		let e_light = world
			.create_entity()
			.with(transform)
			.with(geom_light.clone())
			.with(material_light.clone())
			.with(point_light.clone())
			.build();

		world.add_child(lights_parent, e_light);

		lights.push(e_light);
	}
	//////////////////////////////////////////////////////////////////////////////////
	////////////// lights ////////////////////////////////////////////////////////////
	//////////////////////////////////////////////////////////////////////////////////


	let texture2 = SharedTexture2D::new_from_path("images/cloud.png");
	let mut mat = Material::new_mesh_standard();
	{
		let tags = mat.get_tags_mut();
		tags.insert(ShaderTag::B_UV);
	}
	mat.set_uniform("map_color", &Uniform::Texture2D(Some(texture2), 0));


	let material = SharedMaterial::new(mat);



	let entries: Vec<Entity> = vec![
		Vector3::new(4.0, 0.0, 0.0),
		Vector3::new(0.0, 4.0, 0.0),
		Vector3::new(0.0, 0.0, 4.0),
		Vector3::new(-4.0, 0.0, 0.0),
		Vector3::new(0.0, -4.0, 0.0),
		Vector3::new(0.0, 0.0, -4.0),
	].into_iter().map(|item|{
		world
			.create_entity()
			.with(Transform::from_position(item))
			.with(geom_box.clone())
			.with(material.clone())
			.build()
	}).collect();






	render_system.camera = Some(e_cam);
	render_system.windowed_context.window().set_resizable(true);
	// render_system.window.set_resizable(true);
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

			////////////// lights ////////////////////////////////////////////////////////////
			{
				let transform = transform_store.get_mut(lights_parent).unwrap();
				transform.rotation.y = time * 0.5;
				transform.rotation.x = time * 0.3;
				transform.rotation.z = time * 0.1;
			}
			////////////// lights ////////////////////////////////////////////////////////////
		}

		system_transform.run_now(&world.res);
		render_system.run_now(&world.res);
	}
}
