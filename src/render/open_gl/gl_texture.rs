extern crate gl;
extern crate uuid;

use self::gl::types::*;
use uuid::Uuid;
use core::{MagFilter, MinFilter, Texture2D, TextureColorType, TextureDataSource, Wrapping};
use std::collections::HashMap;
use std::os::raw::c_void;

pub type GLTextureIDs = HashMap<Uuid, TextureId>;

#[derive(Debug)]
pub struct TextureId {
	pub id: GLuint,
}

impl Drop for TextureId {
	fn drop(&mut self) {
		println!("delete texture");

		gl_call!({
			// TODO remove textures
			gl::DeleteTextures(1, &self.id);
		});
	}
}

pub trait GLTexture {
	fn bind(&mut self, hash_map: &mut GLTextureIDs) -> u32;
	fn unbind(&self);
}

impl GLTexture for Texture2D {
	fn bind(&mut self, hash_map: &mut GLTextureIDs) -> u32{
		let gl_texture_dimensions = gl::TEXTURE_2D;
		// let gl_texture_dimensions = get_texture_dimensions(&self.dimensions);

		if self.need_update {
			hash_map.remove(&self.uuid);
			self.need_update = false;
		}

		if hash_map.get(&self.uuid).is_none() {
			let tid = load_texture(self).unwrap();
			gl_call!({
				gl::BindTexture(gl_texture_dimensions, tid.id);
			});
			hash_map.insert(self.uuid, tid);
		}

		let tid = hash_map.get(&self.uuid).unwrap();
		gl_call!({
			gl::BindTexture(gl_texture_dimensions, tid.id);
		});

		tid.id
	}

	fn unbind(&self) {
		unimplemented!()
	}
}

pub fn load_texture(texture: &mut Texture2D) -> Result<TextureId, ()> {
	println!("_/ LOAD TEXTURE______________________________",);

	let mut id: u32 = 0;
	let mut uploaded = false;
	let need_clear = texture.auto_clear_texture_data;


	let gl_texture_dimensions = gl::TEXTURE_2D;

	gl_call!({
		gl::GenTextures(1, &mut id);
		gl::BindTexture(gl_texture_dimensions, id);

		gl::TexParameteri(
			gl::TEXTURE_2D,
			gl::TEXTURE_WRAP_S,
			to_gl_wrapping(texture.wrapping_x) as i32,
		);
		gl::TexParameteri(
			gl::TEXTURE_2D,
			gl::TEXTURE_WRAP_T,
			to_gl_wrapping(texture.wrapping_y) as i32,
		);

		gl::TexParameteri(
			gl::TEXTURE_2D,
			gl::TEXTURE_MIN_FILTER,
			to_gl_min_filter(texture.min_filter) as i32,
		);
		gl::TexParameteri(
			gl::TEXTURE_2D,
			gl::TEXTURE_MAG_FILTER,
			to_gl_mag_filter(texture.mag_filter) as i32,
		);
	});

	if !texture.has_texture_data() && texture.path.is_some() {
		texture.load().unwrap();
	}


	let texture_data = texture.get_texture_data_ref_mut().unwrap();

	match &texture_data.data {
		TextureDataSource::Raw(data) => {
			gl_call!({
				gl::TexImage2D(
					gl_texture_dimensions,
					0,
					to_gl_color_internal_type(texture_data.color_type) as i32,
					texture_data.width as i32,
					texture_data.height as i32,
					0,
					to_gl_color_pixel_data_type(texture_data.color_type),
					gl::UNSIGNED_BYTE,
					&data[0] as *const u8 as *const c_void,
				);
			});
		}
		TextureDataSource::RawUploaded => panic!("can't upload to video memory cleaned texture"),
		TextureDataSource::TextureBuffer => {
			gl_call!({
				gl::TexImage2D(
					gl_texture_dimensions,
					0,
					to_gl_color_internal_type(texture_data.color_type) as i32,
					texture_data.width as i32,
					texture_data.height as i32,
					0,
					to_gl_color_pixel_data_type(texture_data.color_type),
					gl::UNSIGNED_BYTE,
					0 as *const c_void,
				);
			});
		}
	}

	if let TextureDataSource::Raw(_) = texture_data.data {
		gl_call!({
			gl::GenerateMipmap(gl_texture_dimensions);
		});
		uploaded = true;
	}

	if uploaded && need_clear {
		texture_data.data = TextureDataSource::RawUploaded;
	}

	Ok(TextureId { id })
}

fn to_gl_wrapping(wrapping: Wrapping) -> u32 {
	match wrapping {
		Wrapping::ClampToEdge => gl::CLAMP_TO_EDGE,
		Wrapping::MirroredRepeat => gl::MIRRORED_REPEAT,
		Wrapping::Repeat => gl::REPEAT,
	}
}

pub fn to_gl_mag_filter(mag_filter: MagFilter) -> u32 {
	match mag_filter {
		MagFilter::Linear => gl::LINEAR,
		MagFilter::Nearest => gl::NEAREST,
	}
}

pub fn to_gl_min_filter(min_filter: MinFilter) -> u32 {
	match min_filter {
		MinFilter::Linear => gl::LINEAR,
		MinFilter::Nearest => gl::NEAREST,
		MinFilter::LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
		MinFilter::LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
		MinFilter::NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
		MinFilter::NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
	}
}

pub fn to_gl_color_internal_type(color_type: TextureColorType) -> u32 {
	match color_type {
		TextureColorType::R(_) => gl::RED,
		TextureColorType::RG(_) => gl::RG,
		// TextureColorType::RGB(_) => gl::SRGB,
		// TextureColorType::RGBA(_) => gl::SRGB_ALPHA,
		TextureColorType::RGB(_) => gl::RGB,
		TextureColorType::RGBA(_) => gl::RGBA,
		TextureColorType::Stencil => gl::STENCIL_ATTACHMENT,
		TextureColorType::Depth => gl::DEPTH_ATTACHMENT,
		TextureColorType::DepthStencil => gl::DEPTH24_STENCIL8,
	}
}

pub fn to_gl_color_pixel_data_type(color_type: TextureColorType) -> u32 {
	match color_type {
		TextureColorType::R(_) => gl::RED,
		TextureColorType::RG(_) => gl::RG,
		TextureColorType::RGB(_) => gl::RGB,
		TextureColorType::RGBA(_) => gl::RGBA,
		TextureColorType::Stencil => gl::STENCIL_INDEX,
		TextureColorType::Depth => gl::DEPTH_COMPONENT,
		TextureColorType::DepthStencil => gl::UNSIGNED_INT_24_8,
	}
}
