extern crate uuid;
extern crate gl;

use core::MeshBasicMaterial;
use std::collections::HashMap;
use self::uuid::Uuid;
use self::gl::types::*;
use std::ptr;
use std::str;
use std::ffi::{CStr, CString};
use self::gl::types::*;

pub type GLMaterialIDs = HashMap<Uuid, ShaderProgram>;

#[derive(Debug)]
pub struct ShaderProgram {
	fs_source: String,
	vs_source: String,
	id: GLuint,
}

impl Drop for ShaderProgram {
	fn drop(&mut self) {
		gl_call!({
			gl::DeleteProgram(self.id);
		});
	}
}

pub trait GLMaterial {
	fn get_program(&self) -> ShaderProgram;
	fn bind(&self, hash_map: &mut GLMaterialIDs);
	fn unbind(&self);

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


impl <T> GLMaterial for MeshBasicMaterial<T> {
	fn get_program(&self) -> ShaderProgram {
		ShaderProgram {
			fs_source: String::from(FRAGMENT_SHADER_SOURCE),
			vs_source: String::from(VERTEX_SHADER_SOURCE),
			id: 0,
		}
	}

	fn bind(&self, hash_map: &mut GLMaterialIDs){
		match hash_map.get_mut(&self.uuid) {
			None => {},
			Some(ref program) => {
				gl_call!({ gl::UseProgram(program.id); });
				return;
			}
		}

		let mut program = self.get_program();
		Self::compile_shader_program(&mut program);

		gl_call!({ gl::UseProgram(program.id); });
		hash_map.insert(self.uuid, program);

		self.bind(hash_map);
	}

	fn unbind(&self){
		gl_call!({ gl::UseProgram(0); });
	}
}

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    out vec4 color;
    void main() {
        color = vec4(aColor.xyz, 1.0);
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;


const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    in vec4 color;
    layout (location = 0) out vec4 FragColor;
    uniform vec4 u_Color;

    void main() {
        // FragColor = vec4(1.0, 0.0, 0.0, 1.0);
        FragColor = color;
    }
"#;
