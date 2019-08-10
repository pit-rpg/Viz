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
		Material,
		SharedMaterial,
		create_world,
		ShaderProgram,
		PointLight,
		SystemTransform,
		Parent,
		EntityRelations,
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


	let mut mat_phong1 = Material::new_mesh_phong();
	let shared_mat_phong1 = SharedMaterial::new(mat_phong1);

	let mut mat_standard2 = Material::new_mesh_standard();
	let shared_mat_standard2 = SharedMaterial::new(mat_standard2);


	let obj_parent = world
		.create_entity()
		.with(Transform::default())
		.build();



	for mut object in objects {

		if !object.has_attribute(BufferType::Normal) {
			object.generate_normals();
		}

		let geom = SharedGeometry::new(object);

		let mut transform1 = Transform::default();
		let mut transform2 = Transform::default();

		transform1.position.x -= 0.5;
		transform2.position.x += 0.5;
		transform1.scale.set_scalar(0.4);
		transform2.scale.set_scalar(0.4);

		let mut mat1 = shared_mat_phong1.clone();
		let mut mat2 = shared_mat_standard2.clone();

		{
			let mut material = mat1.lock().unwrap();
			material.set_uniform(UniformName::Color, Vector3::new_one());
			material.set_uniform(UniformName::Specular, Vector3::new_one());
			material.set_uniform(UniformName::Shininess, 1.0);
			material.set_uniform(UniformName::SpecularStrength, 1.0);
		}
		{
			let mut material = mat2.lock().unwrap();
			material.set_uniform(UniformName::Color, Vector3::new_one());
			material.set_uniform(UniformName::Specular, Vector3::new_one());
			material.set_uniform(UniformName::Roughness, 0.5);
			material.set_uniform(UniformName::Metalness, 1.0);
			material.set_uniform(UniformName::AmbientLight, Vector3::new_zero());
		}

		let elem1 = world
			.create_entity()
			.with(transform1)
			.with(geom.clone())
			.with(mat1)
			.build();

		let elem2 = world
			.create_entity()
			.with(transform2)
			.with(geom)
			.with(mat2)
			.build();

		world.add_child(obj_parent, elem1);
		world.add_child(obj_parent, elem2);
	}

		let lights_parent = world
			.create_entity()
			.with(Transform::default())
			.build();



	let mut lights = Vec::new();
	for _ in  0..4 {
		let mut transform = Transform::default();
		transform.scale.set(0.2,0.2,0.2);
		transform.position
			.randomize()
			.multiply_scalar(10.0)
			.sub_scalar(5.0);

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

		world.add_child(lights_parent, e_light);

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
			{
				let transform = transform_store.get_mut(lights_parent).unwrap();
				transform.rotation.y = time * 0.5;
				transform.rotation.x = time * 0.3;
				transform.rotation.z = time * 0.1;
			}
			// {
			// 	let transform = transform_store.get_mut(obj_parent).unwrap();
			// 	let scale = (time * 0.4).sin();
			// 	transform.scale.set(scale,scale,scale);
			// }
		}

		system_transform.run_now(&world.res);
		render_system.run_now(&world.res);
	}
}
