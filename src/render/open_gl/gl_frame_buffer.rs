extern crate gl;
extern crate uuid;

use self::gl::types::*;
use uuid::Uuid;
use super::gl_render_buffer::{GLRenderBuffer, GLRenderBufferIDs};
use super::gl_texture::{GLTexture, GLTextureIDs};
use core::{FrameBuffer, FrameOutput};
use std::collections::HashMap;
// use std::os::raw::c_void;

pub type GLFrameBufferIDs = HashMap<Uuid, FrameBufferId>;

#[derive(Debug)]
pub struct FrameBufferId {
	pub id: GLuint,
}

impl Drop for FrameBufferId {
	fn drop(&mut self) {
		println!("delete FrameBuffer");

		gl_call!({
			// TODO: remove Renderbuffers
			gl::DeleteFramebuffers(1, &self.id);
		});
	}
}

pub trait GLFrameBuffer {
	fn bind(
		&mut self,
		frame_buffer_hash_map: &mut GLFrameBufferIDs,
		texture_hash_map: &mut GLTextureIDs,
		render_hash_map: &mut GLRenderBufferIDs,
	);
	fn unbind(&self);

	fn bind_default();
}

impl GLFrameBuffer for FrameBuffer {
	fn bind(
		&mut self,
		frame_buffer_hash_map: &mut GLFrameBufferIDs,
		texture_hash_map: &mut GLTextureIDs,
		render_hash_map: &mut GLRenderBufferIDs,
	) {
		if self.need_update {
			frame_buffer_hash_map.remove(&self.uuid);
			self.need_update = false;
		}

		if frame_buffer_hash_map.get(&self.uuid).is_none() {
			let buffer_id = create_frame_buffer(self);

			self.frame_outputs.iter_mut().for_each(|item| match item {
				FrameOutput::SharedRenderBuffer(shared_buffer) => {
					let buffer = &mut shared_buffer.lock().unwrap();
					let id = buffer.bind(render_hash_map);
					gl_call!({
						gl::FramebufferRenderbuffer(
							gl::FRAMEBUFFER,
							gl::DEPTH_STENCIL_ATTACHMENT,
							gl::RENDERBUFFER,
							id,
						);
					});
				}
				FrameOutput::SharedTexture2D(shared_texture) => {
					let texture = &mut shared_texture.lock().unwrap();
					let id = texture.bind(texture_hash_map);
					gl_call!({
						gl::FramebufferTexture2D(
							gl::FRAMEBUFFER,
							gl::COLOR_ATTACHMENT0,
							gl::TEXTURE_2D,
							id,
							0,
						);
					});
				}
			});
			frame_buffer_hash_map.insert(self.uuid, buffer_id);

			gl_call!({
				if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
					println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
				}
			});
		}

		let buffer_id = frame_buffer_hash_map.get(&self.uuid).unwrap();
		gl_call!({
			gl::BindFramebuffer(gl::FRAMEBUFFER, buffer_id.id);
		});
	}

	fn unbind(&self) {
		unimplemented!()
	}

	fn bind_default() {
		gl_call!({
			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
		});
	}
}

pub fn create_frame_buffer(_frame_buffer: &mut FrameBuffer) -> FrameBufferId {
	let mut id: u32 = 0;

	gl_call!({
		gl::GenFramebuffers(1, &mut id);
		gl::BindFramebuffer(gl::FRAMEBUFFER, id);
	});

	FrameBufferId { id }
}
