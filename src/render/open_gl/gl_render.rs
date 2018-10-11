#![feature(box_syntax)]

extern crate gl;
extern crate glutin;
extern crate rand;
extern crate specs;
extern crate uuid;

use std::time::{Instant, Duration};
use std::os::raw::c_void;
use std::ffi::CStr;

use core::BufferGeometry;
use core::SharedGeometry;
use core::Material;
use core::SharedMaterial;
use core::Transform;
use core::Uniform;
use core::PerspectiveCamera;

use self::gl::types::*;
use self::gl::GetString;
use self::glutin::dpi::*;
use self::glutin::{EventsLoop, GlContext, GlWindow, ContextError};
use self::specs::{ReadStorage, System, Write, WriteStorage, Entity, Join};
use self::uuid::Uuid;

use math::{Matrix3, Matrix4, Vector4, Vector};
use super::super::Renderer;
use super::gl_geometry::VertexArraysIDs;
use super::gl_material::GLMaterialIDs;
use super::gl_texture::GLTextureIDs;
use super::GLGeometry;
use super::GLMaterial;
// #[allow(dead_code)]
// pub struct GLRenderer {
// 	pub window: GlWindow,
// 	pub events_loop: EventsLoop,
// }

// use self::specs::{Component, ReadStorage, RunNow, System, VecStorage, World, Write, WriteStorage};

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

}


impl Default for RenderSystem {
	fn default() -> Self {
		Self::new()
	}
}


impl RenderSystem {
	pub fn new() -> Self {
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

		RenderSystem::print_gl_version();

		Self {
			camera: None,
			window: gl_window,
			events_loop,
			timer: Instant::now(),
			time: Duration::new(0,0),
			delta_time: Duration::new(0,0),
			delta_max: None,
			clear_color: Vector4::new_zero(),
			clear_color_need_update: true,
		}
	}

	pub fn clear(&self) {
		gl_call!({
			gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT);
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
}


impl<'a> System<'a> for RenderSystem {
	type SystemData = (
		ReadStorage<'a, PerspectiveCamera>,
		ReadStorage<'a, Transform>,
		WriteStorage<'a, SharedGeometry>,
		WriteStorage<'a, SharedMaterial>,
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
		// let time = self.get_duration();
		// /Time

		// println!("{:?}", time);

		let (
			camera_coll,
			transform_coll,
			mut geometry_coll,
			mut material_coll,
			mut vertex_arrays_ids,
			mut gl_material_ids,
			mut gl_texture_ids,
		) = data;

		let mut matrix_cam_position;
		let matrix_projection;

		match self.camera {
			None => {
				matrix_cam_position = Matrix4::new();
				matrix_projection = Matrix4::new();
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
				// matrix_cam_position = cam_transform.matrix_world * cam_transform.matrix_local;
			}
		}

		for (transform, geometry, shared_material) in (&transform_coll, &mut geometry_coll, &mut material_coll).join() {
			let matrix_model = matrix_cam_position * transform.matrix_world * transform.matrix_local;
			let mut matrix_normal = Matrix3::new();
			matrix_normal.get_normal_matrix(&matrix_model);

			let mut material = shared_material.lock().unwrap();
			let geom = geometry.lock().unwrap();

			material
				.set_uniform("matrix_model", &Uniform::Matrix4f(matrix_model))
				.unwrap();

			material
				.set_uniform("matrix_view", &Uniform::Matrix4f(matrix_projection))
				.unwrap();

			material
				.set_uniform("matrix_normal", &Uniform::Matrix3f(matrix_normal));

			// material
			// 	.set_uniform("matrix_normal", &Uniform::Matrix4(matrix_normal));
			// println!("{:?}", matrix_normal);

			if prev_geom != geom.uuid {
				geom.bind(&mut vertex_arrays_ids);
				prev_geom = geom.uuid;
			}

			material.bind(&mut gl_material_ids, &mut gl_texture_ids);

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
