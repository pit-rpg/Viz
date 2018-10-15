extern crate gl;
extern crate uuid;

use self::gl::types::*;
use self::uuid::Uuid;
use core::{Texture2D, TextureColorType, SharedTexture2D};
// use core::{Texture, TextureColorType, TextureDimensions};
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
			gl::DeleteTextures(1, self.id as *const u32);
		});
	}
}

pub trait GLTexture {
	fn bind(&self, hash_map: &mut GLTextureIDs);
	fn unbind(&self);
}

impl GLTexture for Texture2D {
	fn bind(&self, hash_map: &mut GLTextureIDs) {
		let gl_texture_dimensions = gl::TEXTURE_2D;
		// let gl_texture_dimensions = get_texture_dimensions(&self.dimensions);

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
	}

	fn unbind(&self) {
		unimplemented!()
	}
}



fn to_gl_color_type(color_type: &TextureColorType) -> u32 {
	// TODO color depth support
	match color_type {
		TextureColorType::RGB(_) => gl::RGB,
		TextureColorType::RGBA(_) => gl::RGBA,
		TextureColorType::Gray(_) => gl::DEPTH_COMPONENT,
		_ => gl::RGBA,
	}
}

// pub fn get_texture_dimensions(d: &TextureDimensions) -> u32{
// 	match d {
// 		TextureDimensions::D1 => {gl::TEXTURE_1D}
// 		TextureDimensions::D2 => {gl::TEXTURE_2D}
// 		TextureDimensions::D3 => {gl::TEXTURE_3D}
// 	}
// }

pub fn load_texture(texture: &Texture2D) -> Result<TextureId, ()> {
	println!("_/ LOAD TEXTURE______________________________",);

	let mut id: u32 = 0;
	let texture_data = texture.load().expect(&format!("Error cant load texture: {}", texture.path));
	let gl_texture_dimensions = gl::TEXTURE_2D;
	// let gl_texture_dimensions = get_texture_dimensions(&texture.dimensions);

	println!("{:?}", texture_data.color_type);
	let color_type = to_gl_color_type(&texture_data.color_type);

	gl_call!({
		gl::GenTextures(1, &mut id);
		gl::BindTexture(gl_texture_dimensions, id);

		gl::TexImage2D(
			gl_texture_dimensions,
			0,
			color_type as i32,
			texture_data.width as i32,
			texture_data.height as i32,
			0,
			color_type,
			gl::UNSIGNED_BYTE,
			&texture_data.data[0] as *const u8 as *const c_void,
		);
		gl::GenerateMipmap(gl_texture_dimensions);
	});

	println!("__ LOAD TEXTURE______________________________",);

	Ok(TextureId { id })
}
