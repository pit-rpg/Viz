extern crate uuid;

use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use self::uuid::Uuid;
use super::{TextureColorType, SharedTexture2D};


#[derive(Debug, Clone, PartialEq)]
pub struct FrameBuffer {
	pub uuid: Uuid,
	pub frame_outputs: Vec<FrameOutput>,
	pub need_update: bool, // TODO: UPDATE
}

#[derive(Debug, Clone, PartialEq)]
pub enum FrameOutput {
	SharedRenderBuffer(SharedRenderBuffer),
	SharedTexture2D(SharedTexture2D),
}

// TODO: mem cleaning
#[derive(Debug, Clone, PartialEq)]
pub struct RenderBuffer {
	pub uuid: Uuid,
	pub color_type: TextureColorType,
	pub with: u32,
	pub height: u32,
	pub need_update: bool, // TODO: UPDATE
}

#[derive(Debug, Clone)]
pub struct SharedRenderBuffer {
	data: Arc<Mutex<RenderBuffer>>,
	uuid: Uuid,
}

impl SharedRenderBuffer {
	pub fn new(render_buffer: RenderBuffer) -> Self {
		Self {
			uuid: render_buffer.uuid,
			data: Arc::new(Mutex::new(render_buffer)),
		}
	}

	pub fn lock(&mut self) -> LockResult<MutexGuard<RenderBuffer>> {
		self.data.lock()
	}

	pub fn get_uuid(&self) -> Uuid {
		self.uuid
	}
}

impl PartialEq for SharedRenderBuffer {
	fn eq(&self, other: &Self) -> bool {
		self.uuid == other.uuid
	}
}