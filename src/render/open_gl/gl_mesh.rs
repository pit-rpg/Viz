extern crate byteorder;
extern crate gl;
extern crate uuid;

// #[macro_use]
// use render::render_gl::macros;
use self::byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use self::gl::types::*;
use self::uuid::Uuid;
use super::{GLGeometry, GLMaterial, GLRenderer};
use core::{BufferAttribute, BufferGeometry, BufferType, Material, Mesh};
use std::collections::HashMap;
use std::mem;
use std::os::raw::c_void;

pub trait GLMesh {
	fn bind(&self, renderer: &mut GLRenderer);
	fn un_bind(&self);
}

impl GLMesh for Mesh {
	fn bind(&self, renderer: &mut GLRenderer) {
		self.geometry.bind(&mut renderer.vartex_arrays_ids);
		self.material.bind(&mut renderer.gl_material_ids);

		match self.geometry.indices {
			Some(ref indices) => {
				let len = indices.len() as GLint;
				gl_call!({
					gl::DrawElements(gl::TRIANGLES, len, gl::UNSIGNED_INT, 0 as *const c_void);
				});
			}
			None => {}
		}
	}


	fn un_bind(&self) {
		self.geometry.un_bind();
		self.material.un_bind();
	}
}
