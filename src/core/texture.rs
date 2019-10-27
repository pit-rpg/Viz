extern crate image;
extern crate uuid;

use self::image::{ColorType, GenericImageView};
use self::uuid::Uuid;
use std::path::Path;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Wrapping {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagFilter {
	Nearest,
	Linear,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MinFilter {
	Nearest,
	Linear,
	NearestMipmapNearest,
	LinearMipmapNearest,
	NearestMipmapLinear,
	LinearMipmapLinear,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextureColorType {
	R(u8),
	RG(u8),
	RGB(u8),
	RGBA(u8),
	DepthStencil,
	Stencil,
	Depth,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextureDataSource {
	Raw(Vec<u8>),
	RawUploaded,
	TextureBuffer,
}

// TODO: mem cleaning
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Texture2D {
	pub path: Option<String>,
	pub uuid: Uuid,
	pub wrapping_x: Wrapping,
	pub wrapping_y: Wrapping,
	pub mag_filter: MagFilter,
	pub min_filter: MinFilter,
	pub auto_clear_texture_data: bool,
	pub need_update: bool, // TODO: UPDATE
	texture_data: Option<TextureData>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureData {
	pub color_type: TextureColorType,
	pub width: u32,
	pub height: u32,
	pub data: TextureDataSource, // TODO optional data for memory save
}

impl Texture2D {
	pub fn new(path: &str) -> Self {
		let mut e = Self::default();
		e.path = Some(path.to_string());
		e
	}

	pub fn new_from(data: TextureData) -> Self {
		let mut e = Self::default();
		e.texture_data = Some(data);
		e
	}

	pub fn new_from_bytes(bytes: &[u8]) -> Self {
		let img = image::load_from_memory(bytes).unwrap();

		let color_type = img.color().into();

		let data = img.raw_pixels();
		let (width, height) = img.dimensions();

		let mut e = Self::default();
		e.texture_data = Some(TextureData {
			data: TextureDataSource::Raw(data),
			width,
			height,
			color_type,
		});
		e
	}

	pub fn load(&mut self) -> Result<&TextureData, (String)> {
		match (&self.path, self.texture_data.is_none()) {
			(_, false) => Ok(self.texture_data.as_ref().unwrap()),
			(Some(path), true) => {
				let img = match image::open(&Path::new(path)) {
					Err(_) => {
						return Err(format!("cant open image: {}", path));
					}
					Ok(im) => im.flipv(),
				};

				let color_type = img.color().into();

				let data = img.raw_pixels();
				let (width, height) = img.dimensions();

				self.texture_data = Some(TextureData {
					data: TextureDataSource::Raw(data),
					width,
					height,
					color_type,
				});

				Ok(self.texture_data.as_ref().unwrap())
			}
			_ => Err("missing path for load image".to_string()),
		}
	}

	pub fn set_texture_data(&mut self, data: Option<TextureData>) {
		self.texture_data = data;
	}

	pub fn get_texture_data_ref(&self) -> Option<&TextureData> {
		self.texture_data.as_ref()
	}

	pub fn get_texture_data_ref_mut(&mut self) -> Option<&mut TextureData> {
		self.texture_data.as_mut()
	}

	pub fn has_texture_data(&self) -> bool {
		self.texture_data.is_some()
	}

	pub fn set_size(&mut self, width: u32, height: u32) {
		if let Some(texture_data) = &mut self.texture_data {
			match texture_data.data {
				TextureDataSource::TextureBuffer => {
					texture_data.width = width;
					texture_data.height = height;
				}
				TextureDataSource::Raw(_) => unimplemented!(),
				TextureDataSource::RawUploaded => unimplemented!(),
			}
			self.need_update = true;
		}
	}
}

#[derive(Debug, Clone)]
pub struct SharedTexture2D {
	data: Arc<Mutex<Texture2D>>,
	uuid: Uuid,
}

impl SharedTexture2D {
	pub fn new(texture: Texture2D) -> Self {
		Self {
			uuid: texture.uuid,
			data: Arc::new(Mutex::new(texture)),
		}
	}

	pub fn new_from_path(path: &str) -> Self {
		let texture = Texture2D::new(path);
		Self::new(texture)
	}

	pub fn new_color_buffer(width: u32, height: u32) -> Self {
		let data = TextureData {
			color_type: TextureColorType::RGB(8),
			width,
			height,
			data: TextureDataSource::TextureBuffer,
		};

		let texture = Texture2D {
			path: None,
			uuid: Uuid::new_v4(),
			wrapping_x: Wrapping::ClampToEdge,
			wrapping_y: Wrapping::ClampToEdge,
			min_filter: MinFilter::Linear,
			mag_filter: MagFilter::Linear,
			auto_clear_texture_data: false,
			need_update: true,
			texture_data: Some(data),
		};

		Self::new(texture)
	}

	pub fn new_depth_stencil(width: u32, height: u32) -> Self {
		let data = TextureData {
			color_type: TextureColorType::DepthStencil,
			width,
			height,
			data: TextureDataSource::TextureBuffer,
		};

		let texture = Texture2D {
			path: None,
			uuid: Uuid::new_v4(),
			wrapping_x: Wrapping::ClampToEdge,
			wrapping_y: Wrapping::ClampToEdge,
			min_filter: MinFilter::Nearest,
			mag_filter: MagFilter::Nearest,
			auto_clear_texture_data: false,
			need_update: true,
			texture_data: Some(data),
		};

		Self::new(texture)
	}

	pub fn lock(&mut self) -> LockResult<MutexGuard<Texture2D>> {
		self.data.lock()
	}

	pub fn get_uuid(&self) -> Uuid {
		self.uuid
	}
}

impl Default for Texture2D {
	fn default() -> Self {
		Self {
			path: None,
			uuid: Uuid::new_v4(),
			wrapping_x: Wrapping::Repeat,
			wrapping_y: Wrapping::Repeat,
			min_filter: MinFilter::LinearMipmapLinear,
			mag_filter: MagFilter::Linear,
			auto_clear_texture_data: true,
			need_update: true,
			texture_data: None,
		}
	}
}

impl PartialEq for SharedTexture2D {
	fn eq(&self, other: &Self) -> bool {
		self.uuid == other.uuid
	}
}

impl From<ColorType> for TextureColorType {
	fn from(color_type: ColorType) -> Self {
		match color_type {
			ColorType::Gray(d) => TextureColorType::R(d),
			ColorType::RGB(d) => TextureColorType::RGB(d),
			ColorType::RGBA(d) => TextureColorType::RGBA(d),
			_ => panic!(format!("can't convert color type from: {:?}", color_type)),
		}
	}
}
