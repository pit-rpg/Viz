extern crate typer;

use core::{Texture2D, SharedTexture2D, TextureData, Rect, TextureColorType, SharedGeometry, TextureDataSource};
use math::{Vector4};
use self::typer::{TextRenderer, Typer};
use self::typer::rusttype::{Font};
use std::path::{PathBuf};
use helpers::geometry_generators::{plane_buffer_geometry};


pub trait Text {
	fn set_text(&mut self, text: String);
	fn render(&mut self, texture: SharedTexture2D, fonts: &Vec<(String, Font)>);

	fn load_fonts<'a>(fonts: Vec<(String, PathBuf)>) -> Vec<(String, Font<'a>)> {
		TextRenderer::load_fonts(fonts)
	}
}

pub struct TextDinamic {
	pub size: usize,
	pub font: String,
	pub color: Vector4<f32>,
	pub bg_color: Vector4<f32>,
	pub need_update: bool,
	text: String,
	typer: Typer,
}


impl TextDinamic {

	pub fn new(text: &str, font: &str) -> Self {
		Self {
			font: font.to_string(),
			text: text.to_string(),
			size: 22,
			color: Vector4::new(0.0, 0.0, 0.0, 1.0),
			bg_color: Vector4::new(1.0, 1.0, 1.0, 1.0),
			need_update: true,
			typer: Typer::new(),
		}
	}

	pub fn new_transparent(text: &str, font: &str) -> Self {
		Self {
			font: font.to_string(),
			text: text.to_string(),
			size: 22,
			color: Vector4::new(0.0, 0.0, 0.0, 1.0),
			bg_color: Vector4::new(0.0, 0.0, 0.0, 0.0),
			need_update: true,
			typer: Typer::new(),
		}
	}

	pub fn get_formated(&self) -> String {
		format!(
			"<block><s font=\"{}\" color=\"{}\">{}</s></block>",
			self.font,
			self.color.as_hex_color(),
			self.text
		)
	}


	pub fn create_elements(&mut self, fonts: &Vec<(String, Font)>) -> (SharedGeometry, SharedTexture2D) {
		let texture_data = TextureData {
			color_type: TextureColorType::RGBA(8),
			width: 0,
			height: 0,
			data: TextureDataSource::Raw(Vec::new()),
		};
		let texture = Texture2D::new_from(texture_data);
		let mut s_texture = SharedTexture2D::new(texture);

		self.render(s_texture.clone(), fonts);

		let s_plane = {
			let mut texture = s_texture.lock().unwrap();
			let texture_data = texture.get_texture_data_ref_mut().unwrap();
			let plane = plane_buffer_geometry(texture_data.width as f32, texture_data.height as f32, 1, 1);
			SharedGeometry::new(plane)
		};

		(s_plane, s_texture)
	}
}


impl Text for TextDinamic {

	fn set_text(&mut self, text: String) {
		self.text = text;
		self.need_update = true;
	}


	fn render(&mut self, mut texture: SharedTexture2D, fonts: &Vec<(String, Font)>) {
		let text = self.get_formated();
		let blocks = self.typer.parse(&text);
		let mut  layout = TextRenderer::format(blocks, 1.0, fonts);
		layout.calk_view();

		let mut buffer = layout.create_buffer(&self.bg_color.as_u8_color()).unwrap();
		TextRenderer::render(&layout, &mut buffer);
		let mut texture = texture.lock().unwrap();
		let texture_data = texture.get_texture_data_ref_mut().unwrap();

		texture_data.color_type = TextureColorType::RGBA(8);
		texture_data.width = buffer.width as u32;
		texture_data.height = buffer.height as u32;
		texture_data.data = TextureDataSource::Raw(buffer.buffer);
	}
}
