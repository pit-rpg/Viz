extern crate image;
extern crate uuid;

use self::image::{ColorType, GenericImageView};
use self::uuid::Uuid;
use std::path::Path;
use std::sync::{Arc,Mutex, LockResult, MutexGuard};
// use std::cmp::PartialEq;


#[allow(dead_code)]
#[derive(Debug)]
pub enum Wrapping {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Filtering {
	NEAREST,
	LINEAR,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum TextureColorType {
	None,
	Gray(u8),
	RGB(u8),
	RGBA(u8),
}

// // #[allow(dead_code)]
// #[derive(Debug, Clone)]
// pub enum TextureDimensions {
// 	D1,
// 	D2,
// 	D3,
// }


#[allow(dead_code)]
#[derive(Debug)]
pub struct Texture2D {
	pub path: String,
	pub uuid: Uuid,
	pub wrapping_x: Wrapping,
	pub wrapping_y: Wrapping,
	pub filtering: Filtering,
	// pub dimensions: TextureDimensions,
}


pub struct TextureData {
	pub color_type: TextureColorType,
	pub width: u32,
	pub height: u32,
	pub data: Vec<u8>, // TODO optional data for memory save
}

impl Texture2D {

	pub fn new (path: &str) -> Self {
		Self {
			path: path.to_string(),
			uuid: Uuid::new_v4(),
			wrapping_x: Wrapping::Repeat,
			wrapping_y: Wrapping::Repeat,
			filtering: Filtering::NEAREST,
			// dimensions: TextureDimensions::D2,
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


#[derive(Debug, Clone)]
pub struct SharedTexture2D (Arc<Mutex<Texture2D>>);


impl SharedTexture2D {
	pub fn new(texture: Texture2D) -> Self {
		SharedTexture2D(Arc::new(Mutex::new(texture)))
	}

	pub fn new_from_path(path: &str) -> Self {
		SharedTexture2D(Arc::new(Mutex::new(Texture2D::new(path))))
	}

	pub fn lock(&mut self) -> LockResult<MutexGuard<Texture2D>> {
		self.0.lock()
	}
}

// impl PartialEq for TextureDimensions {
// 	 fn eq(&self, other: &Self) -> bool {
// 		match (self, other) {
// 			(TextureDimensions::D1, TextureDimensions::D1) => { true },
// 			(TextureDimensions::D2, TextureDimensions::D2) => { true },
// 			(TextureDimensions::D3, TextureDimensions::D3) => { true },
// 			_ => { false }
// 		}
// 	}
// }