extern crate uuid;
extern crate gl;

use core::{MeshBasicMaterial, Material, BufferType};
use std::collections::HashMap;
use self::uuid::Uuid;
use self::gl::types::*;
use std::ptr;
use std::str;
use std::ffi::{CStr, CString};
use self::gl::types::*;
use super::gl_texture::{GLTextureIDs, GLTexture};

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


impl ShaderProgram {

	fn compile_shader_program(program: &mut ShaderProgram) {
		let id;

		gl_call!({
			id = gl::CreateProgram();

			let fs = Self::compile_shader(gl::FRAGMENT_SHADER, &program.fs_source[..]);
			let vs = Self::compile_shader(gl::VERTEX_SHADER, &program.vs_source[..]);

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



		program.id = id;
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
where Self: Material
{
	fn get_program(&self) -> ShaderProgram;

	fn bind(&self, mat_store: &mut GLMaterialIDs, texture_store: &mut GLTextureIDs){
		match mat_store.get_mut(&self.get_uuid()) {
			None => {},
			Some(ref program) => {
				self
					.get_textures()
					.iter()
					.for_each(|t| t.lock().unwrap().bind(texture_store) );

				gl_call!({ gl::UseProgram(program.id); });
				return;
			}
		}

		let mut program = self.get_program();
		ShaderProgram::compile_shader_program(&mut program);

		mat_store.insert(*self.get_uuid(), program);

		self.bind(mat_store, texture_store);
	}

	fn unbind(&self){
		self
			.get_textures()
			.iter()
			.for_each(|t| t.lock().unwrap().unbind() );

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

const BASIC_VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
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
    in vec4 color;
    in vec2 uv;
    layout (location = 0) out vec4 FragColor;

    uniform vec4 u_Color;
	uniform sampler2D map_color;

    void main() {
        // FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        // FragColor = vec4(uv.x+uv.y, uv.x+uv.y, uv.x+uv.y, 1.0);
        // FragColor = color;
        FragColor = texture(map_color, uv);
    }
"#;
