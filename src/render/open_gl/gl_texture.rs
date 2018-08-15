extern crate gl;
extern crate uuid;

use std::collections::HashMap;
use core::{ Texture, TextureColorType, TextureData };
use self::gl::types::*;
use self::uuid::Uuid;
use std::os::raw::c_void;


pub type GLTextureIDs = HashMap<Uuid, TextureId>;


#[derive(Debug)]
pub struct TextureId {
	id: GLuint,
}


impl Drop for TextureId {
	fn drop(&mut self) {
		println!("delete texture");
		gl_call!({
			// TODO remove textures
			unimplemented!()
		});
	}
}

pub trait GLTexture {
	fn bind(&self, hash_map: &mut GLTextureIDs) {}
	fn unbind(&self) {}
}


impl GLTexture for Texture {

	fn bind(&self, hash_map: &mut GLTextureIDs) {

		match hash_map.get_mut(&self.uuid) {
			None => {},
			Some(ref program) => {
				gl_call!({ gl::UseProgram(program.id); });
				return;
			}
		}

		let texture_id = load_texture(self).expect(&format!("cant load texture:{}", &self.path));
		hash_map.insert(self.uuid, texture_id);

		self.bind(hash_map);
	}


	fn unbind(&self) {
		gl_call!({ gl::BindTexture(gl::TEXTURE_2D, 0); });
	}
}


fn to_gl_color_type (color_type: &TextureColorType) -> u32 {
	// TODO color depth support
	match color_type {
		TextureColorType::RGB(_) => { gl::RGB }
		TextureColorType::RGBA(_) => { gl::RGBA }
		TextureColorType::Gray(_) => { gl::DEPTH_COMPONENT }
		_ => { gl::RGBA }
	}
}


pub fn load_texture (texture: &Texture) -> Result<TextureId, ()> {

	println!("_/ LOAD TEXTURE______________________________", );

	let mut id: u32 = 0;
	let texture_data = texture.load()?;

	// let data = &texture.data;
	println!("{:?}", texture_data.color_type );
	// let a = gl::types::RGB;
	let color_type = to_gl_color_type(&texture_data.color_type);


	gl_call!({
		gl::GenTextures(1, &mut id);
		gl::BindTexture(gl::TEXTURE_2D, id);

		gl::TexImage2D(
			gl::TEXTURE_2D,
			0,
			color_type as i32,
			texture_data.width as i32,
			texture_data.height as i32,
			0,
			color_type,
			gl::UNSIGNED_BYTE,
			&texture_data.data[0] as *const u8 as *const c_void
		);
		// gl::TexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, data);
		gl::GenerateMipmap(gl::TEXTURE_2D);
	});

	println!("__ LOAD TEXTURE______________________________", );

	Ok( TextureId{id} )
}