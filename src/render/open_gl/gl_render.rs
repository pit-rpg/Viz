extern crate gl;
extern crate glutin;
extern crate rand;

use self::gl::types::*;
use core::BufferGeometry;
use core::Material;
use core::Transform;
use core::Uniform;
use core::PerspectiveCamera;
use math::Matrix4;
use std::os::raw::c_void;

use self::glutin::dpi::*;
use self::glutin::{EventsLoop, GlContext, GlWindow};
use super::super::Renderer;
use super::gl_geometry::VertexArraysIDs;
use super::gl_material::GLMaterialIDs;
use super::gl_texture::GLTextureIDs;
use super::GLGeometry;
use super::GLMaterial;

#[allow(dead_code)]
pub struct GLRenderer {
	pub window: GlWindow,
	pub events_loop: EventsLoop,
}

extern crate specs;
// use self::specs::{Component, ReadStorage, RunNow, System, VecStorage, World, Write, WriteStorage};
use self::specs::{ReadStorage, System, Write, WriteStorage, Entity};

pub struct RenderSystem {
	pub camera: Option<Entity>,
}

impl Default for RenderSystem {
	fn default() -> Self {
		Self {
			camera: None,
		}
	}
}

impl<'a> System<'a> for RenderSystem {
	type SystemData = (
		ReadStorage<'a, PerspectiveCamera>,
		ReadStorage<'a, Transform>,
		ReadStorage<'a, BufferGeometry>,
		WriteStorage<'a, Material>,
		Write<'a, VertexArraysIDs>,
		Write<'a, GLMaterialIDs>,
		Write<'a, GLTextureIDs>,
	);

	fn run(&mut self, data: Self::SystemData) {
		use self::specs::Join;

		let (
			camera_coll,
			transform_coll,
			geometry_coll,
			mut material_coll,
			mut vertex_arrays_ids,
			mut gl_material_ids,
			mut gl_texture_ids,
		) = data;

		let mut view_matrix;

		match self.camera {
			None => {view_matrix = Matrix4::new();}
			Some( ref cam ) => {
				let cam_transform = transform_coll.get(*cam).unwrap();
				let camera = camera_coll.get(*cam).unwrap();
				view_matrix = Matrix4::new();
				view_matrix.get_inverse(&(cam_transform.matrix_world * cam_transform.matrix_local * camera.matrix_projection_inverse));
			}
		}

		for (transform, geometry, material) in (&transform_coll, &geometry_coll, &mut material_coll).join() {
			material
				.set_uniform("transform", &Uniform::Matrix4(view_matrix * transform.matrix_world * transform.matrix_local))
				.unwrap();

			geometry.bind(&mut vertex_arrays_ids);
			material.bind(&mut gl_material_ids, &mut gl_texture_ids);

			match geometry.indices {
				Some(ref indices) => {
					let len = indices.len() as GLint;
					gl_call!({
						gl::DrawElements(gl::TRIANGLES, len, gl::UNSIGNED_INT, 0 as *const c_void);
					});
				}
				None => {}
			}

			geometry.unbind();
			material.unbind();
		}
	}
}

impl Renderer for GLRenderer {
	fn new() -> Self {
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

		super::print_gl_version();

		GLRenderer {
			window: gl_window,
			events_loop,
		}
	}

	fn clear(&self) {
		gl_call!({
			gl::Clear(gl::COLOR_BUFFER_BIT|gl::DEPTH_BUFFER_BIT);
		});
	}

	fn render() {}
}
