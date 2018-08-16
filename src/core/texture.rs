extern crate image;
extern crate uuid;

use self::image::{GenericImage, ColorType};
use self::uuid::Uuid;
use std::path::Path;

#[derive(Debug)]
pub enum Wrapping {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
}

#[derive(Debug)]
pub enum Filtering {
	NEAREST,
	LINEAR,
}

#[derive(Debug)]
pub enum TextureColorType {
	None,
	Gray(u8),
	RGB(u8),
	RGBA(u8),
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct Texture {
	pub name: String,
	pub path: String,
	pub uuid: Uuid,
	pub wrapping_x: Wrapping,
	pub wrapping_y: Wrapping,
	pub filtering: Filtering,
}


pub struct TextureData {
	pub color_type: TextureColorType,
	pub width: u32,
	pub height: u32,
	pub data: Vec<u8>, // TODO optional data for memory save
}

impl Texture {

	pub fn new (name: &str, path: &str) -> Self {
		Self {
			name: name.to_string(),
			path: path.to_string(),
			uuid: Uuid::new_v4(),
			wrapping_x: Wrapping::Repeat,
			wrapping_y: Wrapping::Repeat,
			filtering: Filtering::NEAREST,
		}
	}

	pub fn load (&self) -> Result<TextureData, ()> {

		let img =  match image::open(&Path::new(&self.path)) {
			Ok(img) => { img }
			Err(e) => {
				println!("{}", e);
				return Err(())
			}
		};

		let img = img.flipv();

		let color_type;
		match img.color() {
			ColorType::Gray(d) => { color_type = TextureColorType::Gray(d) },
			ColorType::RGB(d) =>  { color_type = TextureColorType::RGB(d) },
			ColorType::RGBA(d) => { color_type = TextureColorType::RGBA(d) },
			_ =>{ return Err(()) }
		}

		let data = img.raw_pixels();
		let (width, height) = img.dimensions();

		Ok(TextureData {
			data,
			width,
			height,
			color_type,
		})
	}
}