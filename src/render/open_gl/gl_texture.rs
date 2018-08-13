extern crate gl;
use core::{ RawTexture };
use self::gl::types::*;

fn load_textures (texures:&[&RawTexture]) -> Result<(), String> {
	for texture in texures {

		match texture.data {
			None => { return Err("Texture data missing".to_string()); }
			_=>{}
		};

		let data = texture.data.as_ref().unwrap();
		let mut texture_id: u32 = 0;

		gl_call!({
			gl::GenTextures(1, &mut texture_id);
			gl::BindTexture(gl::TEXTURE_2D, texture_id);

			// gl::TexImage2D(GL_TEXTURE_2D, 0, GL_RGB, width, height, 0, GL_RGB, GL_UNSIGNED_BYTE, data);
			// gl::GenerateMipmap(GL_TEXTURE_2D);
		});
	}

	Ok(())
}