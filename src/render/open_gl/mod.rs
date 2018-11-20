// #![macro_escape]


extern crate gl;
extern crate glutin;
extern crate rand;
extern crate uuid;


#[macro_use]
pub mod macros;
mod gl_geometry;
mod gl_material;
mod gl_render;
mod gl_texture;
mod gl_shaderProgram;

extern crate image;

use std::sync::{Arc, Mutex};
use std::f64::consts::PI as PI_f64;


use self::glutin::GlContext;
use math::Vector4;
use math::Vector3;
use math::Vector2;
use math::Vector;

use self::gl_geometry::VertexArraysIDs;
use self::gl_geometry::GLGeometry;
use core::BufferType;
use core::BufferGeometry;
use core::SharedGeometry;
use core::Material;
use core::SharedMaterial;
use core::{Texture2D, SharedTexture2D, Uniform, ShaderProgram};
use core::PerspectiveCamera;
use core::Transform;
use render::Renderer;
use self::gl_render::*;
use self::gl_texture::*;
use self::gl_material::GLMaterial;
use self::gl_material::GLMaterialIDs;
use helpers::geometry_generators;
use std::f32::consts::PI;


#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct WindowState {
	pointer_pos: (f64, f64),
	pointer_pressed: (bool, bool, bool),
	pointer_wheel: f32,
	window_size: (f64, f64),
}





extern crate specs;
use self::specs::{World, RunNow, Builder};



pub fn test()
// where T:Nums+'static
{
	let mut render_system = self::RenderSystem::default();

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


	// let geom2 = box_geometry(1.0,1.0,1.0);
	let geom_container = SharedGeometry::new(geometry_generators::box_geometry(1.0, 1.0, 1.0));
	// let geom_container = box_geometry(1.0,1.0,1.0);
	// let geom_sphere = sphere(0.5, 32, 32);
	let geom_light = SharedGeometry::new(geometry_generators::sphere(0.5, 12, 12));

	let camera = PerspectiveCamera::new();

	let transform2 = Transform::default();
	let transform_spare = Transform::default();

	let mut transform_camera = Transform::default();
	transform_camera.position.z = 6.0;
	transform_camera.update();

	let mut transform_light = Transform::default();
	// transform_light.position.set(1.2, 1.0, 2.0);
	transform_light.scale.set(0.2, 0.2, 0.2);
	transform_light.update();

	let texture1 = SharedTexture2D::new_from_path("images/tile.jpg");
	let texture2 = SharedTexture2D::new_from_path("images/awesomeface.png");
	let texture3 = SharedTexture2D::new_from_path("images/earth.jpg");

	let texture_a = SharedTexture2D::new_from_path("images/Stone_Tiles_003_COLOR.jpg");
	let texture_a2 = SharedTexture2D::new_from_path("images/Stone_Tiles_003_ROUGH.jpg");

	let texture_container = SharedTexture2D::new_from_path("images/container2.png");
	let texture_container_specular = SharedTexture2D::new_from_path("images/container2_specular.png");


	let mut material2 = Material::new_basic_texture(&Vector4::random());
	material2.set_uniform("texture_color", &Uniform::Texture2D(Some(texture2.clone())));
	let mut material2 = SharedMaterial::new(material2);

	let normal_mat = SharedMaterial::new(Material::new_normal());

	let mut material_sphere = Material::new_light_texture(&Vector4::new(1.0,0.5,0.31,1.0), &Vector3::new_one(), &transform_light.position);
	material_sphere.set_uniform("texture_color", &Uniform::Texture2D(Some(texture_container)));
	material_sphere.set_uniform("texture_specular", &Uniform::Texture2D(Some(texture_container_specular)));
	let mut boxMat = SharedMaterial::new(material_sphere);

	let mut material_sphere3 = Material::new_light_texture(&Vector4::new(1.0,0.5,0.31,1.0), &Vector3::new_one(), &transform_light.position);
	material_sphere3.set_uniform("texture_color", &Uniform::Texture2D(Some(texture_a)));
	material_sphere3.set_uniform("texture_specular", &Uniform::Texture2D(Some(texture_a2)));
	let mut boxMat3 = SharedMaterial::new(material_sphere3);

	let material_sphere2 = Material::new_light(&Vector4::new(1.0,0.5,0.31,1.0), &Vector3::new_one(), &transform_light.position);
	let mut boxMat2 = SharedMaterial::new(material_sphere2);

	let material_phong = Material::new_phong(&Vector4::new(0.46,0.46,1.0,1.0), &Vector3::new_one(), &transform_light.position);
	let mut box_phong = SharedMaterial::new(material_phong);

	let material_light = SharedMaterial::new(Material::new_basic(&Vector4::new(1.0,1.0,1.0,1.0)));


	let mut world = World::new();
	world.register::<SharedGeometry>();
	world.register::<SharedMaterial>();
	world.register::<Transform>();
	world.register::<PerspectiveCamera>();
	world.add_resource(VertexArraysIDs::new());
	world.add_resource(GLMaterialIDs::new());
	world.add_resource(GLTextureIDs::new());

	// println!("{}", geom2.uuid);

	{
		// let a = geom2.0;
	}

	let e2 = world
		.create_entity()
		.with(geom_container.clone())
		.with(material2)
		.with(transform2)
		.build();

	// let e3 = world
	//	 .create_entity()
	//	 .with(geom_container)
	//	 // .with(geom_sphere)
	//	 .with(material_sphere)
	//	 .with(transform_spare)
	//	 .build();

	let e_cam = world
		.create_entity()
		.with(transform_camera)
		.with(camera)
		.build();

	let e_Light = world
		.create_entity()
		.with(geom_light.clone())
		.with(material_light)
		.with(transform_light)
		.build();

	// {
		// let mut transform_store = world.write_storage::<Transform>();
		// transform_store.
		// let mut cam_store = world.write_storage::<PerspectiveCamera>();
		// let transform_camera = transform_store.get_mut(e_cam).unwrap();
	// }

	let mut boxes = Vec::new();

	let count = 1000;
	for i in 0..1000 {
		let mut transform = Transform::default();
		transform.scale.set(0.4,0.4,0.4);
		transform.position
			.randomize()
			.multiply_scalar(10.0)
			.sub_scalar(5.0);

		let mut mat;
		let mut geom;

		if i < count/3 {
			mat = boxMat.clone();
		} else if i < count/3*2 {
			// transform.scale.set(0.2,0.2,0.5);
			// mat = boxMat3.clone();
			mat = box_phong.clone();
		} else {
			mat = boxMat2.clone();
		}

		if i%2 == 0 {
			geom = geom_container.clone();
		} else {
			geom = geom_light.clone();
		}


		transform.update();

		let m_box = world
			.create_entity()
			// .with(geom_light.clone())
			.with(geom.clone())
			// .with(normal_mat.clone())
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
							let dpi_factor = window.get_hidpi_factor();
							window.resize(logical_size.to_physical(dpi_factor));
							// window.set_inner_size(logical_size);
							// window.context().resize(logical_size.to_physical(dpi_factor));
							// println!("{:?}", logical_size);
							window_state.window_size.0 = logical_size.width;
							window_state.window_size.1 = logical_size.height;
						},
						CursorMoved { position: pos, .. } =>{
							window_state.pointer_pos = pos
								.to_physical(window.get_hidpi_factor())
								.to_logical(hidpi_factor)
								.into();
							// println!("{:?}", mouse_state.pos);
						}
						// WindowEvent::Resized(data) => {
						//	 println!("{:?}", data);
						//	 // window.resize(w, h),
						// }
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

					// transform.position.x += 0.001;
					// transform.position.y += 0.001;
					// transform.position.z -= 0.01;
				}
			}

			{
				// let transform_spare = transform_store.get_mut(e3).unwrap();
				// transform_spare.rotation.y += 0.001;
				// transform_spare.rotation.z += 0.002;
				// transform_spare.rotation.x += 0.003;
				// transform_spare.scale.y = 2.0 * render_system.get_duration().sin().abs();
				// transform_spare.update();
			}
			{
				let transform_camera = transform_store.get_mut(e_cam).unwrap();
				let camera = cam_store.get_mut(e_cam).unwrap();
				let x_prog = window_state.pointer_pos.0 / window_state.window_size.0;
				let y_prog = window_state.pointer_pos.1 / window_state.window_size.1;
				transform_camera.position.z = ( (x_prog * (PI_f64*2.0)).sin() * radius ) as f32;
				transform_camera.position.x = ( (x_prog * (PI_f64*2.0)).cos() * radius ) as f32;;
				transform_camera.position.y = (( y_prog * radius - radius/2.0) * -2.0) as f32;
				// println!("{:?}", transform_camera.rotation);
				transform_camera.look_at(&center, &up);
				transform_camera.update();
				// camera.aspect = (window_state.window_size.0/window_state.window_size.1) as f32;
				// camera.view.full_width = window_state.window_size.0 as f32;
				// camera.view.full_height = window_state.window_size.1 as f32;
				// camera.view.width = window_state.window_size.0 as f32;
				// camera.view.height = window_state.window_size.1 as f32;
				// camera.update_projection_matrix();
			}
		}

		render_system.run_now(&world.res);

	}

}