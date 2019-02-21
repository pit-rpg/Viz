extern crate uuid;
#[macro_use] extern crate project;


use std::f64::consts::PI as PI_f64;
use project::{
	specs::*,
	glutin,
	render,
	math::{Vector3, Vector, Vector4},
	core::{SharedGeometry, PerspectiveCamera, Transform, SharedTexture2D, Material, SharedMaterial, Uniform, create_world, ShaderProgram},
	helpers::{geometry_generators},
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
	let mut render_system = render::open_gl::gl_render::RenderSystem::new(&mut world);
	
	let count = 400;
	let mut boxes = Vec::with_capacity(count);

	gl_call!({
		gl::Enable(gl::DEPTH_TEST);
	});

	let mut f_count = 0.0;
	let up = Vector3::new(0.0, 1.0, 0.0);
	let center = Vector3::new_zero();
	let radius = 10.0;

	let mut color1 = Vector3::<f32>::random();
	let mut color2 = Vector3::<f32>::random();
	let mut color_tmp = Vector3::<f32>::new(color1.x, color1.y, color1.z);

	let mut running = true;


	let geom_container = SharedGeometry::new(geometry_generators::box_geometry(1.0, 1.0, 1.0));

	let geom_light = SharedGeometry::new(geometry_generators::sphere(0.5, 12, 12));


	let transform2 = Transform::default();

	let mut camera = PerspectiveCamera::new();
	let mut transform_camera = Transform::default();
	transform_camera.position.z = 6.0;
	transform_camera.update();
	camera.view.enabled = false;
	println!("{:?}", camera);

	let mut transform_light = Transform::default();
	transform_light.scale.set(0.2, 0.2, 0.2);
	transform_light.update();

	let texture2 = SharedTexture2D::new_from_path("images/awesomeface.png");

	let texture_container = SharedTexture2D::new_from_path("images/container2.png");
	let texture_container_specular = SharedTexture2D::new_from_path("images/container2_specular.png");


	let mut material2 = Material::new_basic_texture(&Vector4::random());
	material2.set_uniform("texture_color", &Uniform::Texture2D(Some(texture2.clone())));
	let material2 = SharedMaterial::new(material2);

	let mut material_sphere = Material::new_light_texture(&Vector4::new(1.0,0.5,0.31,1.0), &Vector3::new_one(), &transform_light.position);
	material_sphere.set_uniform("texture_color", &Uniform::Texture2D(Some(texture_container)));
	material_sphere.set_uniform("texture_specular", &Uniform::Texture2D(Some(texture_container_specular)));
	let box_mat = SharedMaterial::new(material_sphere);

	let material_sphere2 = Material::new_light(&Vector4::new(1.0,0.5,0.31,1.0), &Vector3::new_one(), &transform_light.position);
	let box_mat2 = SharedMaterial::new(material_sphere2);

	let material_phong = Material::new_phong(&Vector4::new(0.46,0.46,1.0,1.0), &Vector3::new_one(), &transform_light.position);
	let box_phong = SharedMaterial::new(material_phong);

	let material_light = SharedMaterial::new(Material::new_basic(&Vector4::new(1.0,1.0,1.0,1.0)));

	let e2 = world
		.create_entity()
		.with(geom_container.clone())
		.with(material2)
		.with(transform2)
		.build();


	let e_cam = world
		.create_entity()
		.with(transform_camera)
		.with(camera)
		.build();

	world
		.create_entity()
		.with(geom_light.clone())
		.with(material_light)
		.with(transform_light)
		.build();


	for i in 0..count {
		let mut transform = Transform::default();
		transform.scale.set(0.4,0.4,0.4);
		transform.position
			.randomize()
			.multiply_scalar(10.0)
			.sub_scalar(5.0);

		let mut mat;
		let mut geom;

		if i < count/3 {
			mat = box_mat.clone();
		} else if i < count/3*2 {
			mat = box_phong.clone();
		} else {
			mat = box_mat2.clone();
		}

		if i%2 == 0 {
			geom = geom_container.clone();
		} else {
			geom = geom_light.clone();
		}


		transform.update();

		let m_box = world
			.create_entity()
			.with(geom.clone())
			.with(mat)
			.with(transform)
			.build();
		boxes.push(m_box);
	}


	render_system.camera = Some(e_cam);
	render_system.window.set_resizable(true);
	let hidpi_factor = render_system.window.get_hidpi_factor().round();
	let mut window_state = WindowState::default();

	while running {

		{
			let window = &render_system.window;
			// let mut events_loop = &test_gl_render.events_loop;
			use self::glutin::WindowEvent::*;

			render_system.events_loop.poll_events(|event| {
				match event {
					glutin::Event::WindowEvent{ event, .. } => match event {
						glutin::WindowEvent::CloseRequested => running = false,
						glutin::WindowEvent::Resized(logical_size) => {
							// let dpi_factor = window.get_hidpi_factor();
							// window.resize(logical_size.to_physical(dpi_factor));
							// window.set_inner_size(logical_size);
							// window.context().resize(logical_size.to_physical(dpi_factor));
							println!("{:?}", logical_size);
							window_state.window_size.0 = logical_size.width;
							window_state.window_size.1 = logical_size.height;

							gl_call!({
								gl::Viewport(0,0, logical_size.width as i32, logical_size.height as i32);
							});
						},
						CursorMoved { position: pos, .. } =>{
							window_state.pointer_pos = pos
								.to_physical(window.get_hidpi_factor())
								.to_logical(hidpi_factor)
								.into();
						}
						_ => ()
					},
					_ => ()
				}
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

		render_system.clear_color.from_vector3(&color_tmp, 1.0);
		render_system.clear_color_need_update = true;

		{
			let mut transform_store = world.write_storage::<Transform>();
			let mut cam_store = world.write_storage::<PerspectiveCamera>();
			{
				let transform = transform_store.get_mut(e2).unwrap();
				transform.rotation.y += 0.01;
				transform.rotation.z += 0.01;
				transform.position.x += 0.001;
				transform.position.y += 0.001;
				transform.position.z -= 0.01;
				transform.update();
			}
			{
				for m_box in boxes.iter() {
					let transform = transform_store.get_mut(*m_box).unwrap();
					if transform.scale.z == 0.5 {
						if render_system.get_duration() < 10.0 {
							transform.rotation.x -= 0.001;
							transform.rotation.y -= 0.002;
							transform.rotation.z -= 0.003;
							transform.update();
						}

					} else {
						transform.rotation.x += 0.01;
						transform.rotation.y += 0.02;
						transform.rotation.z += 0.03;
						transform.update();
					}
				}
			}


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
				transform_camera.update();
			}
		}

		render_system.run_now(&world.res);

	}
}
