extern crate image;
extern crate uuid;

use self::image::{ColorType, GenericImageView};
use self::uuid::Uuid;
use std::path::Path;
use std::sync::{Arc,Mutex, LockResult, MutexGuard};


#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Wrapping {
	Repeat,
	MirroredRepeat,
	ClampToEdge,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum MagFilter {
    Nearest,
    Linear,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum MinFilter {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TextureColorType {
	None,
	R(u8),
	RG(u8),
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
	pub mag_filter: MagFilter,
	pub min_filter: MinFilter,
	pub auto_clear_texture_data: bool,
	pub need_update: bool,
	texture_data: Option<TextureData>,
}


#[derive(Debug, Clone)]
pub struct TextureData {
	pub color_type: TextureColorType,
	pub width: u32,
	pub height: u32,
	pub data: Vec<u8>, // TODO optional data for memory save
}


impl Texture2D {

	pub fn new (path: &str) -> Self {
		let mut e = Self::default();
		e.path = Some(path.to_string());
		e
	}


	pub fn new_from (data: TextureData) -> Self {
		let mut e = Self::default();
		e.texture_data = Some(data);
		e
	}


	pub fn new_from_bytes (path: Option<String>, bytes: &[u8]) -> Self {
		let img = image::load_from_memory(bytes).unwrap();

		let color_type = match img.color() {
			ColorType::Gray(d) => TextureColorType::R(d),
			ColorType::RGB(d) =>  TextureColorType::RGB(d),
			ColorType::RGBA(d) => TextureColorType::RGBA(d),
			_ =>{  panic!( format!("unknown color type for: {}", path.unwrap_or("<Unknown path (bytes)>".to_string()) ) )}
		};

		let data = img.raw_pixels();
		let (width, height) = img.dimensions();

		let mut e = Self::default();
		e.texture_data = Some(TextureData {
				data,
				width,
				height,
				color_type,
			});
		e
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
					ColorType::Gray(d) => TextureColorType::R(d),
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