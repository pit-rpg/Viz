extern crate specs;
extern crate uuid;

use self::specs::{Component, VecStorage};
use self::uuid::Uuid;
use super::TextureColorType;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

// TODO: mem cleaning
#[derive(Debug, Clone, PartialEq)]
pub struct RenderBuffer {
	pub uuid: Uuid,
	pub color_type: TextureColorType,
	pub width: u32,
	pub height: u32,
	pub need_update: bool, // TODO: UPDATE
}

impl RenderBuffer {
	pub fn new(width: u32, height: u32, color_type: TextureColorType) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color_type,
			width,
			height,
			need_update: true,
		}
	}

	pub fn set_size(&mut self, width: u32, height: u32) {
		self.width = width;
		self.height = height;
		self.need_update = true;
	}
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

	pub fn new_depth_stencil(width: u32, height: u32) -> Self {
		Self::new(RenderBuffer::new(
			width,
			height,
			TextureColorType::DepthStencil,
		))
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

impl Component for SharedRenderBuffer {
	type Storage = VecStorage<Self>;
}
