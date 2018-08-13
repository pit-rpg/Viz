extern crate image;
extern crate uuid;

use self::image::{GenericImage, ColorType};
use self::uuid::Uuid;
use std::path::Path;

pub enum Wrapping {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
}

pub enum Filtering {
	NEAREST,
	LINEAR,
}

pub enum TextureColorType {
	None,
	Gray(u8),
	RGB(u8),
	RGBA(u8),
}


#[allow(dead_code)]
pub struct RawTexture {
	pub name: String,
	pub path: String,
	pub uuid: Uuid,
	pub width: u32,
	pub height: u32,
	pub wrapping_x: Wrapping,
	pub wrapping_y: Wrapping,
	pub filtering: Filtering,
	pub keep_data: bool,
	pub color_type: TextureColorType,
	pub data: Option<Vec<u8>>,
}

impl RawTexture {

	pub fn new (name: &str, path: &str) -> Self {
		Self {
			name: name.to_string(),
			path: path.to_string(),
			uuid: Uuid::new_v4(),
			width: 0,
			height: 0,
			keep_data: false,
			color_type: TextureColorType::None,
			wrapping_x: Wrapping::Repeat,
			wrapping_y: Wrapping::Repeat,
			filtering: Filtering::NEAREST,
			data: None
		}
	}

	pub fn new_and_load (name: &str, path: &str) -> Self {
		let mut data = Self::new(name, path);
		data.load();
		data
	}

	pub fn load(&mut self) -> &Vec<u8> {
		let img = image::open(&Path::new("images/tile.jpg")).expect("Failed to load texture");

		match img.color() {
			ColorType::Gray(d) => { self.color_type = TextureColorType::Gray(d) },
			ColorType::RGB(d) => { self.color_type = TextureColorType::RGB(d) },
			ColorType::RGBA(d) => { self.color_type = TextureColorType::RGBA(d) },
			_ =>{}
		}

		let data = img.raw_pixels();
		let (width, height) = img.dimensions();
		self.width = width;
		self.height = height;
		self.data = Some(data);
		self.data.as_ref().unwrap()
	}
}