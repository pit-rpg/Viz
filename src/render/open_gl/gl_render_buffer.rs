extern crate gl;
extern crate uuid;

use self::gl::types::*;
use self::uuid::Uuid;
use core::{RenderBuffer};
use super::gl_texture::{to_gl_color_internal_type};
use std::collections::HashMap;
// use std::os::raw::c_void;

pub type GLRenderBufferIDs = HashMap<Uuid, RenderBufferId>;

#[derive(Debug)]
pub struct RenderBufferId {
	pub id: GLuint,
}

impl Drop for RenderBufferId {
	fn drop(&mut self) {
		println!("delete RenderBufferId");

		gl_call!({
			// TODO remove Renderbuffers
			gl::DeleteRenderbuffers(1, self.id as *const u32);
		});
	}
}

pub trait GLRenderBuffer {
	fn bind(&mut self, hash_map: &mut GLRenderBufferIDs)-> u32;
	fn unbind(&self);
}

impl GLRenderBuffer for RenderBuffer {
	fn bind(&mut self, hash_map: &mut GLRenderBufferIDs) -> u32 {

		if self.need_update {
			hash_map.remove(&self.uuid);
			self.need_update = false;
		}

		if hash_map.get(&self.uuid).is_none() {
			let buffer_id = create_render_buffer(self);
			gl_call!({
				gl::BindRenderbuffer(gl::RENDERBUFFER, buffer_id.id);
			});
			hash_map.insert(self.uuid, buffer_id);
		}

		let buffer_id = hash_map.get(&self.uuid).unwrap();
		gl_call!({
			gl::BindRenderbuffer(gl::RENDERBUFFER, buffer_id.id);
		});

		buffer_id.id
	}

	fn unbind(&self) {
		unimplemented!()
	}
}

pub fn create_render_buffer(render_buffer: &mut RenderBuffer) -> RenderBufferId {
	let mut id: u32 = 0;
	gl_call!({
		gl::GenRenderbuffers(1, &mut id);
		gl::BindRenderbuffer(gl::RENDERBUFFER, id);
		gl::RenderbufferStorage(
			gl::RENDERBUFFER,
			to_gl_color_internal_type(render_buffer.color_type),
			render_buffer.width as i32,
			render_buffer.height as i32,
		);
	});

	RenderBufferId{id}
}
