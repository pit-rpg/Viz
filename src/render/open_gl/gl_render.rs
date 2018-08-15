extern crate gl;
extern crate glutin;
extern crate rand;

use std::cell::{ RefCell };
// use std::ptr;
// use core::Node;
// use core::Component;
use core::BufferGeometry;
use core::Materials;
// use core::Mesh;
use helpers::Nums;
use self::gl::types::*;
use std::os::raw::c_void;

// use std::str;
// use std::ffi::{CStr, CString};
// use self::gl::types::*;
// use self::gl::GetString;
use self::glutin::{EventsLoop, GlContext, GlWindow};
use self::glutin::dpi::*;
use super::gl_geometry::{VertexArraysIDs};
use super::gl_material::GLMaterialIDs;
use super::gl_texture::GLTextureIDs;
// use super::gl_mesh::*;
use super::super::Renderer;
use super::GLMaterial;
use super::GLGeometry;

#[allow(dead_code)]
pub struct GLRenderer {
	pub window: GlWindow,
	pub events_loop: EventsLoop,
	pub vertex_arrays_ids: VertexArraysIDs,
	pub gl_material_ids: GLMaterialIDs,
}


extern crate specs;
use self::specs::{Write, Component, ReadStorage, System, VecStorage, World, RunNow};


pub struct RenderSystem;




impl<'a> System<'a> for RenderSystem {

	type SystemData = (
		ReadStorage<'a, BufferGeometry>,
		ReadStorage<'a, Materials>,
		Write<'a, VertexArraysIDs>,
		Write<'a, GLMaterialIDs>,
		Write<'a, GLTextureIDs>,
	);


	fn run(&mut self, data: Self::SystemData) {
        use self::specs::Join;

		let (
			geometry,
			material,
			mut vertex_arrays_ids,
			mut gl_material_ids,
			mut gl_texture_ids,
		) = data;

        for (geometry, material) in ( &geometry, &material).join() {
			// println!("1");
			geometry.bind(&mut vertex_arrays_ids);
			match material {
				Materials::Basic(m) =>{
					m.bind(&mut gl_material_ids, &mut gl_texture_ids);
					// println!("bind");
				},
				// Materials::Normal(m) =>{ m.bind(&mut gl_material_ids); },
				_ => {}
			}

			match geometry.indices {
				Some(ref indices) => {
					let len = indices.len() as GLint;
					gl_call!({
						gl::DrawElements(gl::TRIANGLES, len, gl::UNSIGNED_INT, 0 as *const c_void);
					});
					// println!("draw");
				}
				None => {}
			}

			geometry.unbind();
			match material {
				Materials::Basic(m) =>{
					  m.unbind();
					//   println!("unbind");
				},
				// Materials::Normal(m) =>{ m.bind(&mut gl_material_ids); },
				_ => {}
			}
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
            vertex_arrays_ids: VertexArraysIDs::new(),
            gl_material_ids: GLMaterialIDs::new(),
		}
	}


	fn clear(&self) {
		gl_call!({
			gl::Clear(gl::COLOR_BUFFER_BIT);
		});
	}


	fn render() {

	}
	// fn render<T:Nums>(&self, node: &mut Node<T>)
	// // where Mesh: Component
	// {

	// 	node.traverse(|ref mut node|  {
	// 		for component in &node.components  {
	// 			// let a:&Component = component.deref();
	// 			let a = component;
	// 			// component.test();
	// 			match **component {
	// 				// Renderable::Mesh(m) =>{}
	// 			// match a {
	// 			// Mesh {geometry, material, uuid, name} => {}
	// 				// Component  => {}
	// 				// RefCell => {

	// 					// !println!("{}", component);
	// 				// }
	// 				// TODO render
	// 				// &RefCell<Box<Component>> =>{}

	// 				_ => {}
	// 			}
	// 		}
	// 	});

    // }
}
