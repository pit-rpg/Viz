extern crate specs;
extern crate uuid;

use self::specs::{Component, VecStorage};
use self::uuid::Uuid;
use super::{SharedRenderBuffer, SharedTexture2D, TextureColorType};
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

#[derive(Debug, Clone, PartialEq)]
pub struct FrameBuffer {
    pub uuid: Uuid,
    pub frame_outputs: Vec<FrameOutput>,
    pub need_update: bool, // TODO: UPDATE
}

impl FrameBuffer {
    pub fn new_color_map_output(width: u32, height: u32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            need_update: true,
            frame_outputs: vec![
                FrameOutput::SharedTexture2D(SharedTexture2D::new_color_buffer(width, height)),
                FrameOutput::SharedRenderBuffer(SharedRenderBuffer::new_depth_stencil(
                    width, height,
                )),
            ],
        }
    }

    pub fn new_color_depth_stencil_map_output(width: u32, height: u32) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            need_update: true,
            frame_outputs: vec![
                FrameOutput::SharedTexture2D(SharedTexture2D::new_color_buffer(width, height)),
                FrameOutput::SharedTexture2D(SharedTexture2D::new_depth_stencil(width, height)),
            ],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FrameOutput {
    SharedRenderBuffer(SharedRenderBuffer),
    SharedTexture2D(SharedTexture2D),
}

#[derive(Debug, Clone)]
pub struct SharedFrameBuffer {
    data: Arc<Mutex<FrameBuffer>>,
    uuid: Uuid,
}

impl SharedFrameBuffer {
    pub fn new(buffer: FrameBuffer) -> Self {
        Self {
            uuid: buffer.uuid,
            data: Arc::new(Mutex::new(buffer)),
        }
    }

    pub fn lock(&mut self) -> LockResult<MutexGuard<FrameBuffer>> {
        self.data.lock()
    }

    pub fn get_uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn new_color_map_output(width: u32, height: u32) -> Self {
        Self::new(FrameBuffer::new_color_map_output(width, height))
    }

    pub fn new_color_depth_stencil_map_output(width: u32, height: u32) -> Self {
        Self::new(FrameBuffer::new_color_depth_stencil_map_output(width, height))
    }
}

impl PartialEq for SharedFrameBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Component for SharedFrameBuffer {
    type Storage = VecStorage<Self>;
}
