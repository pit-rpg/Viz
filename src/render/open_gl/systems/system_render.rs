extern crate gl;
extern crate glutin;
extern crate rand;
extern crate specs;
extern crate uuid;

use std::time::{Instant, Duration};
use std::os::raw::c_void;
use std::ffi::CStr;
// use std::collections::HashSet;
use std::path::PathBuf;

use core::{
	SharedGeometry,
	SharedMaterial,
	Transform,
	Uniform,
	PerspectiveCamera,
	ShaderProgram,
	PointLight,
	DirectionalLight,
	ShaderTag,
};


use self::gl::types::*;
use self::gl::GetString;
use self::glutin::dpi::*;
use self::glutin::{EventsLoop, GlContext, GlWindow, ContextError};
use self::specs::{ReadStorage, System, Write, WriteStorage, Entity, Join, World};
use self::uuid::Uuid;

use math::{Matrix3, Matrix4, Vector4, Vector3, Vector};
use super::super::{
	gl_geometry::VertexArraysIDs,
	gl_material::GLMaterialIDs,
	gl_texture::GLTextureIDs,
	GLGeometry,
	GLMaterial,
};


pub struct RenderSettings {
	// pub num_point_lights: usize,
	// pub num_directional_lights: usize,
}

impl Default for RenderSettings {
	fn default() -> Self {
		RenderSettings{
			// num_point_lights: 4,
		}
	}
}

// pub struct ShaderSource {
// 	pub name: String,
// 	pub src: &'static str,
// }

// impl ShaderSource {
// 	fn new(name: &str, src: &'static str) -> Self {
// 		// let src = include_str!("../shaders/light.glsl");
// 		Self {
// 			name: name.to_string(),
// 			src
// 		}

// 		// unimplemented!()
// 	}
// }

pub struct BindContext<'z> {
	pub tags: &'z Vec<ShaderTag>,
	// pub shader_sources: &'z Vec<ShaderSource>,
	// pub render_settings: &'z RenderSettings,
	pub gl_material_ids: &'z mut GLMaterialIDs,
	pub gl_texture_ids: &'z mut GLTextureIDs,

	pub lights_point_count: usize,
	pub lights_directional_count: usize,
}


pub struct RenderSystem {
	pub camera: Option<Entity>,
	pub window: GlWindow,
	pub events_loop: EventsLoop,
	pub timer: Instant,
	pub time: Duration,
	pub delta_time: Duration,
	pub delta_max: Option<Duration>,
	pub clear_color: Vector4<f32>,
	pub clear_color_need_update: bool,
	pub tags: Vec<ShaderTag>,
	pub render_settings: RenderSettings,
	// shader_sources: Vec<ShaderSource>,

	lights_point_count: usize,
	lights_directional_count: usize,
}


impl RenderSystem {
	pub fn new(world: &mut World) -> Self {
		// TODO: ensure once
		world.add_resource(VertexArraysIDs::new());
		world.add_resource(GLMaterialIDs::new());
		world.add_resource(GLTextureIDs::new());
		// TODO: ensure once /

		let events_loop = glutin::EventsLoop::new();
		let window = glutin::WindowBuilder::new()
			.with_title("Hello, world!")
			.with_dimensions(LogicalSize::new(1024.0, 768.0));

		let context = glutin::ContextBuilder::new().with_vsync(true);

		let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

		unsafe {
			gl_window.make_current().unwrap();
		}

		gl_call!({
			gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
			gl::ClearColor(0.0, 0.2, 0.2, 1.0);
		});


		// Flags
		gl_call!({
			gl::Enable(gl::DEPTH_TEST);
			gl::Enable(gl::STENCIL_TEST);
		});
		// /Flags

		RenderSystem::print_gl_version();

		let mut render_system = Self {
			camera: None,
			window: gl_window,
			events_loop,
			timer: Instant::now(),
			time: Duration::new(0,0),
			delta_time: Duration::new(0,0),
			delta_max: None,
			clear_color: Vector4::new_zero(),
			clear_color_need_update: true,
			tags: Vec::new(),
			render_settings: RenderSettings::default(),
			lights_point_count: 0,
			lights_directional_count: 0,
			// shader_sources: Vec::new()
		};

		// render_system.include_shader("basic", 						include_str!("../shaders/basic.glsl"));
		// render_system.include_shader("basic-texture", 				include_str!("../shaders/basic-texture.glsl"));
		// render_system.include_shader("light", 						include_str!("../shaders/light.glsl"));
		// render_system.include_shader("light_texture", 				include_str!("../shaders/light_texture.glsl"));
		// render_system.include_shader("lololo", 						include_str!("../shaders/lololo.glsl"));
		// render_system.include_shader("mat_cup2", 					include_str!("../shaders/mat_cup2.glsl"));
		// render_system.include_shader("mat_cup", 					include_str!("../shaders/mat_cup.glsl"));
		// render_system.include_shader("mesh_phong", 					include_str!("../shaders/mesh_phong.glsl"));
		// render_system.include_shader("mesh_standard", 				include_str!("../shaders/mesh_standard.glsl"));
		// render_system.include_shader("normal", 						include_str!("../shaders/normal.glsl"));
		// render_system.include_shader("phong", 						include_str!("../shaders/phong.glsl"));
		// render_system.include_shader("point_light", 				include_str!("../shaders/point_light.glsl"));
		// render_system.include_shader("snippet-common", 				include_str!("../shaders/snippet-common.glsl"));
		// render_system.include_shader("snippet-common-lighting", 	include_str!("../shaders/snippet-common-lighting.glsl"));
		// render_system.include_shader("snippet-phong", 				include_str!("../shaders/snippet-phong.glsl"));
		// render_system.include_shader("snippet-standart", 			include_str!("../shaders/snippet-standart.glsl"));
		// render_system.include_shader("test_mat", 					include_str!("../shaders/test_mat.glsl"));

		render_system
	}

	pub fn clear(&self) {
		gl_call!({
			gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT|gl::STENCIL_BUFFER_BIT);
		});
	}

	pub fn swap_buffers(&self) -> Result<(), ContextError> {
		self.window.swap_buffers()
	}

	pub fn gl_clear_error() {
		while unsafe { gl::GetError() } != gl::NO_ERROR {}
	}

	pub fn print_gl_version() {
		gl_call!({
			let version = GetString(gl::VERSION) as *const i8;
			println!("{:?}", CStr::from_ptr(version));
		});
	}

	pub fn get_duration(&self) -> f32 {
		self.time.as_secs() as f32 + self.time.subsec_nanos() as f32 * 1e-9
	}

	pub fn get_delta(&self) -> f32 {
		self.delta_time.as_secs() as f32 + self.delta_time.subsec_nanos() as f32 * 1e-9
	}

	// pub fn include_shader(&mut self, name: &str, src: &'static str) {
	// 	self.shader_sources.push(ShaderSource::new(name, src));
	// }
}


impl<'a> System<'a> for RenderSystem {
	type SystemData = (
		ReadStorage<'a, PerspectiveCamera>,
		ReadStorage<'a, Transform>,
		WriteStorage<'a, SharedGeometry>,
		WriteStorage<'a, SharedMaterial>,
		WriteStorage<'a, PointLight>,
		WriteStorage<'a, DirectionalLight>,
		Write<'a, VertexArraysIDs>,
		Write<'a, GLMaterialIDs>,
		Write<'a, GLTextureIDs>,
	);

	fn run(&mut self, data: Self::SystemData) {

		Self::gl_clear_error();

		// let mut prev_mat = Uuid::new_v4();
		let mut prev_geom = Uuid::new_v4();

		if self.clear_color_need_update {
			gl_call!({
				gl::ClearColor(self.clear_color.x, self.clear_color.y, self.clear_color.z, self.clear_color.w);
			});
			self.clear_color_need_update = false;
		}
		self.clear();

		// Time
		let new_now = Instant::now();
		let mut delta = new_now.duration_since(self.timer);
		self.timer = new_now;
		match self.delta_max {
			None => {}
			Some(ref mut max) => {
				if delta > *max {delta = max.clone()}
			}
		}

		self.time += delta;

		let time = self.get_duration();

		let (
			camera_coll,
			transform_coll,
			mut geometry_coll,
			mut material_coll,
			light_point_coll,
			light_direct_coll,
			mut vertex_arrays_ids,
			mut gl_material_ids,
			mut gl_texture_ids,
		) = data;

		let mut matrix_cam_position;
		let matrix_projection;
		// let matrix_projection_inverse;

		match self.camera {
			None => {
				matrix_cam_position = Matrix4::new();
				matrix_projection = Matrix4::new();
				// matrix_projection_inverse = Matrix4::new();
			}
			Some( ref cam ) => {
				let cam_transform = transform_coll.get(*cam).unwrap();
				let camera = camera_coll.get(*cam).unwrap();
				// matrix_projection = Matrix4::new();
				matrix_cam_position = Matrix4::new();
				matrix_cam_position.get_inverse(&(cam_transform.matrix_world * cam_transform.matrix_local ));
				// matrix_projection = camera.matrix_projection_inverse * cam_transform.matrix_world * cam_transform.matrix_local;
				// matrix_projection = camera.matrix_projection * matrix_cam_position;
				matrix_projection = camera.matrix_projection.clone();
				// matrix_projection_inverse = camera.matrix_projection_inverse.clone();
				// matrix_cam_position = cam_transform.matrix_world * cam_transform.matrix_local;
			}
		}




		let mut light_materials_need_update = false;

		let lights_point: Vec<_> = (&light_point_coll, &transform_coll)
			.join()
			.map(|(light, transform)| {
				let mut pos = transform.position.clone();
				pos.apply_matrix_4(&(matrix_cam_position * transform.matrix_world * transform.matrix_local));
				(light, pos)
			})
			.collect();

		let lights_direct: Vec<_> = (&light_direct_coll, &transform_coll)
			.join()
			.map(|(light, transform)| {
				let mut direction = light.direction.clone();
				let mut matrix_normal = Matrix3::new();
				matrix_normal.get_normal_matrix(&(matrix_cam_position * transform.matrix_world * transform.matrix_local));
				direction.apply_matrix_3(&matrix_normal);
				direction.normalize();
				(light, direction)
			})
			.collect();


		if lights_point.len() != self.lights_point_count {
			self.lights_point_count = lights_point.len();
			light_materials_need_update = true;
		}
		if lights_direct.len() != self.lights_directional_count {
			self.lights_directional_count = lights_direct.len();
			light_materials_need_update = true;
		}

		for (_, shared_material) in (&transform_coll, &mut material_coll).join() {
			let mut material = shared_material.lock().unwrap();

			if !material.get_tags().contains(&ShaderTag::Lighting) {
				continue;
			}

			lights_point.iter().enumerate()
				.for_each(|(i, (light, pos))| {
					let mut col = light.color.clone();
					col.multiply_scalar(light.power);

					material.set_uniform(&format!("pointLights[{}].position", i), &Uniform::Vector3(pos.clone()));
					material.set_uniform(&format!("pointLights[{}].color", i), &Uniform::Vector3(col));
					material.set_uniform(&format!("pointLights[{}].distance", i), &Uniform::Float(light.distance));
					material.set_uniform(&format!("pointLights[{}].decay", i), &Uniform::Float(light.decay));
				});
			lights_direct.iter().enumerate()
				.for_each(|(i, (light, direction))| {
					let mut col = light.color.clone();
					col.multiply_scalar(light.power);

					material.set_uniform(&format!("directionalLights[{}].color", i), &Uniform::Vector3(col));
					material.set_uniform(&format!("directionalLights[{}].direction", i), &Uniform::Vector3(direction.clone()));
				});

			if light_materials_need_update {
				material.set_need_update(true);
			}
		}


		let mut bind_context = BindContext {
			gl_texture_ids: &mut gl_texture_ids,
			gl_material_ids: &mut gl_material_ids,
			tags: &self.tags,
			// shader_sources: &self.shader_sources,

			lights_point_count: self.lights_point_count,
			lights_directional_count: self.lights_directional_count,
		};


		for (transform, geometry, shared_material) in (&transform_coll, &mut geometry_coll, &mut material_coll).join() {
			let matrix_model = matrix_cam_position * transform.matrix_world * transform.matrix_local;
			let mut matrix_normal = Matrix3::new();
			matrix_normal.get_normal_matrix(&(matrix_cam_position * transform.matrix_world * transform.matrix_local));

			let mut position_light = Vector3::new_zero();
			position_light.apply_matrix_4(&( matrix_cam_position ));

			// matrix_normal.get_normal_matrix(&matrix_model);

			let mut material = shared_material.lock().unwrap();
			let geom = geometry.lock().unwrap();

			material
				.set_uniform("matrix_model", &Uniform::Matrix4f(matrix_model));

			material
				.set_uniform("matrix_view", &Uniform::Matrix4f(matrix_projection));

			material
				.set_uniform("matrix_normal", &Uniform::Matrix3f(matrix_normal));

			material
				.set_uniform("time", &Uniform::Float(time));

			if prev_geom != geom.uuid {
				geom.bind(&mut vertex_arrays_ids);
				prev_geom = geom.uuid;
			}

			material.bind(&mut bind_context);

			match geom.indices {
				Some(ref indices) => {
					let len = indices.len() as GLint;
					gl_call!({
						gl::DrawElements(gl::TRIANGLES, len, gl::UNSIGNED_INT, 0 as *const c_void);
					});
				}
				None => {}
			}

			// geom.unbind();
			// material.unbind();

		}

		self.swap_buffers().unwrap();
	}
}
