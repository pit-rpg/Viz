extern crate gl;
extern crate glutin;
extern crate rand;

use std::cell::{ RefCell };


enum Renderable {
	Mesh(Mesh)
}

// use std::ptr;
use core::Node;
use core::Component;
use core::Mesh;
use helpers::Nums;
// use std::str;
// use std::ffi::{CStr, CString};
// use self::gl::types::*;
// use self::gl::GetString;
use self::glutin::{EventsLoop, GlContext, GlWindow};
use super::gl_geometry::{VertexArraysIDs};
use super::gl_material::GLMaterialIDs;
use super::gl_mesh::*;
use super::super::Renderer;

#[allow(dead_code)]
pub struct GLRenderer {
	pub window: GlWindow,
	pub events_loop: EventsLoop,
	pub vertex_arrays_ids: VertexArraysIDs,
	pub gl_material_ids: GLMaterialIDs,
}


impl Renderer for GLRenderer {

	fn new() -> Self {
		let events_loop = glutin::EventsLoop::new();
		let window = glutin::WindowBuilder::new()
			.with_title("Hello, world!")
			.with_dimensions(1024, 768);

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
            vertex_arrays_ids: VertexArraysIDs::new(),
            gl_material_ids: GLMaterialIDs::new(),
		}
	}


	fn clear(&self) {
		gl_call!({
			gl::Clear(gl::COLOR_BUFFER_BIT);
		});
	}


	fn render<T:Nums>(&self, node: &mut Node<T>)
	// where Mesh: Component
	{

		node.traverse(|ref mut node|  {
			for component in &node.components  {
				// let a:&Component = component.deref();
				let a = component;
				// component.test();
				match **component {
					// Renderable::Mesh(m) =>{}
				// match a {
				// Mesh {geometry, material, uuid, name} => {}
					// Component  => {}
					// RefCell => {

						// !println!("{}", component);
					// }
					// TODO render
					// &RefCell<Box<Component>> =>{}

					_ => {}
				}
			}
		});

    }
}
