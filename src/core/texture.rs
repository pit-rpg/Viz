extern crate image;
extern crate uuid;

use self::image::{ColorType, GenericImageView};
use self::uuid::Uuid;
use std::path::Path;
use std::sync::{Arc,Mutex, LockResult, MutexGuard};


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


#[allow(dead_code)]
#[derive(Debug)]
pub struct Texture2D {
	pub path: Option<String>,
	pub uuid: Uuid,
	pub wrapping_x: Wrapping,
	pub wrapping_y: Wrapping,
	pub filtering: Filtering,
	pub auto_clear_texture_data: bool,
	pub need_update: bool,
	texture_data: Option<TextureData>,
}


#[derive(Debug)]
pub struct TextureData {
	pub color_type: TextureColorType,
	pub width: u32,
	pub height: u32,
	pub data: Vec<u8>, // TODO optional data for memory save
}


impl Texture2D {

	pub fn new (path: &str) -> Self {
		Self {
			path: Some(path.to_string()),
			uuid: Uuid::new_v4(),
			wrapping_x: Wrapping::Repeat,
			wrapping_y: Wrapping::Repeat,
			filtering: Filtering::NEAREST,
			texture_data: None,
			auto_clear_texture_data: true,
			need_update: true,
		}
	}


	pub fn new_from (data: TextureData) -> Self {
		Self {
			path: None,
			uuid: Uuid::new_v4(),
			wrapping_x: Wrapping::Repeat,
			wrapping_y: Wrapping::Repeat,
			filtering: Filtering::NEAREST,
			texture_data: Some(data),
			auto_clear_texture_data: true,
			need_update: true,
		}
	}


	pub fn load (&mut self) -> Result<&TextureData, (String)> {

		match (&self.path, self.texture_data.is_none()) {
			(_, false) => {
				Ok(self.texture_data.as_ref().unwrap())
			}
			(Some(path), true) => {
				let img =  match image::open(&Path::new( path )){
					Err(e) => {return Err( format!("cant open image: {}", path) );}
					Ok(im) => im.flipv()
				};

				let color_type = match img.color() {
					ColorType::Gray(d) => TextureColorType::Gray(d),
					ColorType::RGB(d) =>  TextureColorType::RGB(d),
					ColorType::RGBA(d) => TextureColorType::RGBA(d),
					_ =>{ return Err( format!("unknown color type for: {}", path) )}
				};

				let data = img.raw_pixels();
				let (width, height) = img.dimensions();

				self.texture_data = Some(TextureData {
					data,
					width,
					height,
					color_type,
				});

				Ok(self.texture_data.as_ref().unwrap())
			}
			_=> { Err("missing path for load image".to_string()) }
		}
	}

	pub fn set_texture_data(&mut self, data: Option<TextureData>) {
		self.texture_data = data;
	}

	pub fn get_texture_data_ref (&self) -> Option<&TextureData> {
		self.texture_data.as_ref()
	}

	pub fn get_texture_data_ref_mut (&mut self) -> Option<&mut TextureData> {
		self.texture_data.as_mut()
	}

	pub fn has_texture_data(&self) -> bool { self.texture_data.is_some() }
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
