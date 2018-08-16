extern crate uuid;
extern crate gl;

use core::{MeshBasicMaterial, MeshNormalMaterial, Material, BufferType};
use std::collections::HashMap;
use self::uuid::Uuid;
use self::gl::types::*;
use std::ptr;
use std::str;
use std::ffi::{CStr, CString};
use self::gl::types::*;
// use self::gl;
use super::gl_texture::{GLTextureIDs, load_texture, GLTexture, TextureId};
// use std::ffi::CString;
use std::os::raw::c_char;


pub type GLMaterialIDs = HashMap<Uuid, ShaderProgram>;

#[derive(Debug)]
pub struct ShaderProgram {
	fs_source: String,
	vs_source: String,
	id: GLuint,
}

impl Drop for ShaderProgram {
	fn drop(&mut self) {
		println!("delete program");
		gl_call!({
			gl::DeleteProgram(self.id);
		});
	}
}


fn get_gl_uniform_name(name: &str, texture_id: &TextureId) -> String {
	// uniform sampler2D map_color;
	let s = match texture_id.gl_texture_dimensions {
		gl::TEXTURE_1D => {"sampler1D"}
		gl::TEXTURE_2D => {"sampler2D"}
		gl::TEXTURE_3D => {"sampler3D"}
		_=>{panic!();}
	};

	format!("uniform {} {};\n", s, name)
}


impl ShaderProgram {

	fn compile_shader_program <M: Material> (material: &M, program: &mut ShaderProgram, texture_store: &mut GLTextureIDs) {

		let mut texture_uniforms = String::new();
		let mut texture_data = Vec::new();

		for data in material.get_textures(true).iter().take_while(|e| e.is_some()) {
			let (name, texture_mutex ) = data.as_ref().unwrap();

			let name = name.as_ref().unwrap();
			let texture = texture_mutex.lock().unwrap();

			println!("<><><>{}", name);

			if texture_store.get(&texture.uuid).is_none() {
				let id = load_texture(&*texture).unwrap();
				texture_store.insert(texture.uuid, id);
			}

			let texture_id = texture_store.get(&texture.uuid).unwrap();
			let uniform_name = get_gl_uniform_name(&name[..], texture_id);
			texture_uniforms.push_str(&uniform_name[..]);
			// texture_data.push( name.clone() );
			texture_data.push( (name.clone(), texture_id.id, texture_id.gl_texture_dimensions) );
		}

		let id;
		let fs_source = program.fs_source.replace("#REPLACE_TEXTURE_UNIFORMS", &texture_uniforms[..]);

		println!("{}", fs_source);

		gl_call!({
			id = gl::CreateProgram();
			program.id = id;

			let vs = Self::compile_shader(gl::VERTEX_SHADER, &program.vs_source[..]);
			let fs = Self::compile_shader(gl::FRAGMENT_SHADER, &fs_source[..]);

			gl::AttachShader(id, fs);
			gl::AttachShader(id, vs);

			gl::LinkProgram(id);
			gl::ValidateProgram(id);

			let mut info_log = Vec::with_capacity(512);
			info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
			// gl::GetShaderInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
			// println!("{}", str::from_utf8(&info_log).unwrap());
			let mut success = gl::FALSE as GLint;
			gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
			if success != gl::TRUE as GLint {
				gl::GetProgramInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
				println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
			}

			// TODO - releace remove shasers
			gl::DeleteShader(vs);
			gl::DeleteShader(fs);
		});

		gl_call!({ gl::UseProgram(program.id); });

		let mut tex_loc;
		let mut c_name;

		println!("{:?}", texture_data);

		for (i, (name, tid, gl_texture_dimensions)) in texture_data.iter().enumerate() {
		// for (name, t_id) in texture_data {
			c_name = CString::new(name.as_bytes()).unwrap();

			gl_call!({
				gl::BindTexture(*gl_texture_dimensions, *tid);
			});


			gl_call!({
				tex_loc = gl::GetUniformLocation(program.id, c_name.as_ptr());
			});

			println!("->> {} : {} : {}", tex_loc, i, tid);

			gl_call!({
				gl::Uniform1i(tex_loc, i as i32);
			});
		}
	}


	fn compile_shader(t: GLenum, src: &str) -> u32 {
		let id;

		gl_call!({
			id = gl::CreateShader(t);
			let c_str_frag = CString::new(src[..].as_bytes()).unwrap();

			let mut success = gl::FALSE as GLint;
			let mut info_log = Vec::with_capacity(512);
			info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

			gl::ShaderSource(id, 1, &c_str_frag.as_ptr(), ptr::null());
			gl::CompileShader(id);

			// check for shader compile errors
			gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
			if success != gl::TRUE as GLint {
				gl::GetShaderInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
				match t {
					gl::FRAGMENT_SHADER => println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()),
					gl::VERTEX_SHADER => println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()),
					_ => println!("ERROR::SHADER::?::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()),
				};
				gl::DeleteShader(id);
				panic!();
			}
		});

		id
	}
}


pub trait GLMaterial
where Self: Material+Sized
{
	fn get_program(&self) -> ShaderProgram;

	fn bind(&self, mat_store: &mut GLMaterialIDs, texture_store: &mut GLTextureIDs){

		match mat_store.get_mut(&self.get_uuid()) {
			None => {},
			Some(ref program) => {
				gl_call!({ gl::UseProgram(program.id); });

				self
					.get_textures(false)
					.iter()
					.take_while(|e| e.is_some() )
					.enumerate()
					.for_each(|(i, e)| {
						let (_, t) = e.as_ref().unwrap();
						let texture = t.lock().unwrap();
						let texture_id = texture_store.get(&texture.uuid).unwrap();

						gl_call!({
							gl::ActiveTexture(gl::TEXTURE0 + i as u32);
							gl::BindTexture(texture_id.gl_texture_dimensions, texture_id.id);
						});
					});


				return;
			}
		}

		let mut program = self.get_program();
		ShaderProgram::compile_shader_program(self, &mut program, texture_store);

		mat_store.insert(*self.get_uuid(), program);

		self.bind(mat_store, texture_store);
	}

	fn unbind(&self){
		// self
		// 	.get_textures(false)
		// 	.iter()
		// 	.take_while(|e| e.is_some() )
		// 	.for_each(|e| {
		// 		let (_, t) = e.as_ref().unwrap();
		// 		t.lock().unwrap().unbind();
		// 	});

		gl_call!({ gl::UseProgram(0); });
	}


}

impl GLMaterial for MeshBasicMaterial {
	fn get_program(&self) -> ShaderProgram {
		ShaderProgram {
			fs_source: String::from(BASIC_FRAGMENT_SHADER_SOURCE),
			vs_source: String::from(BASIC_VERTEX_SHADER_SOURCE),
			id: 0,
		}
	}
}

impl GLMaterial for MeshNormalMaterial {
	fn get_program(&self) -> ShaderProgram {
		ShaderProgram {
			fs_source: String::from(NORMAL_FRAGMENT_SHADER_SOURCE),
			vs_source: String::from(NORMAL_VERTEX_SHADER_SOURCE),
			id: 0,
		}
	}
}

const BASIC_VERTEX_SHADER_SOURCE: &str = r#"
	#version 330 core
    ##BASIC_VERTEX_SHADER_SOURCE
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    layout (location = 2) in vec2 aUv;
    out vec4 color;
    out vec2 uv;
    void main() {
        color = vec4(aColor.xyz, 1.0);
        uv = aUv;
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;


const BASIC_FRAGMENT_SHADER_SOURCE: &str = r#"
	#version 330 core
    ##BASIC_FRAGMENT_SHADER_SOURCE
    in vec4 color;
    in vec2 uv;
    layout (location = 0) out vec4 FragColor;

    uniform vec4 u_Color;
	// uniform sampler2D map_color;
	#REPLACE_TEXTURE_UNIFORMS

    void main() {
        // FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        // FragColor = vec4(uv.x+uv.y, uv.x+uv.y, uv.x+uv.y, 1.0);
        // FragColor = color;
		vec4 col = texture(map_color2, uv);

		if (col.r+col.g+col.b+col.a > 3.0) {
			col = vec4(1.0, 1.0, 1.0, 1.0);
		} else {
			col = vec4(1.0, 0.0, 0.0, 1.0);
		}

        FragColor = texture(map_color, uv) * col * color;
    }
"#;

const NORMAL_VERTEX_SHADER_SOURCE: &str = r#"
	#version 330 core
    ##NORMAL_VERTEX_SHADER_SOURCE
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    layout (location = 2) in vec2 aUv;
    out vec4 color;
    out vec2 uv;
    void main() {
        color = vec4(aColor.xyz, 1.0);
        uv = aUv;
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;


const NORMAL_FRAGMENT_SHADER_SOURCE: &str = r#"
	#version 330 core
    ##NORMAL_FRAGMENT_SHADER_SOURCE
    in vec4 color;
    in vec2 uv;
    layout (location = 0) out vec4 FragColor;

    uniform vec4 u_Color;
	// uniform sampler2D map_color;
	#REPLACE_TEXTURE_UNIFORMS

    void main() {
        FragColor = vec4(0.5, 0.5, 0.5, 1.0);
        // FragColor = vec4(uv.x+uv.y, uv.x+uv.y, uv.x+uv.y, 1.0);
        // FragColor = color;
        // FragColor = texture(map_color, uv);
    }
"#;
