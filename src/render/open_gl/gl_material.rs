extern crate gl;
extern crate uuid;

use self::gl::types::*;
use self::uuid::Uuid;
use core::{Material, ProgramType, Uniform, UniformItem};
use std::collections::HashMap;
use std::ffi::{CString};
use std::ptr;
use std::str;
use helpers::{find_file, read_to_string};
use super::gl_texture::{load_texture, GLTextureIDs, TextureId, get_texture_dimensions, GLTexture};

pub type GLMaterialIDs = HashMap<Uuid, ShaderProgram>;

#[derive(Debug)]
pub struct ShaderProgram {
	fs_source: String,
	vs_source: String,
	id: GLuint,
	uniform_locations: Vec<i32>,
	texture_locations: Vec<i32>,
}

impl Drop for ShaderProgram {
	fn drop(&mut self) {
		println!("delete program");
		gl_call!({
			gl::DeleteProgram(self.id);
		});
	}
}


pub fn set_uniform(u: &Uniform, loc: i32) {
	if loc == -1 {return}

	match u {
		Uniform::Vector2(data) => {
			gl_call!({
				gl::Uniform2fv(loc, 1, &data.x as *const f32);
			});
		}
		Uniform::Vector3(data) => {
			gl_call!({
				gl::Uniform3fv(loc, 1, &data.x as *const f32);
			});
		}
		Uniform::Vector4(data) => {
			gl_call!({
				gl::Uniform4fv(loc, 1, &data.x as *const f32);
			});
		}
		Uniform::Matrix3f(data) => {
			gl_call!({
				gl::UniformMatrix3fv(loc, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
		Uniform::Matrix4f(data) => {
			gl_call!({
				gl::UniformMatrix4fv(loc, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
		Uniform::Float(data) => {
			gl_call!({
				gl::Uniform1f(loc, *data);
			});
		}
		Uniform::Int(data) => {
			gl_call!({
				gl::Uniform1i(loc, *data);
			});
		}
		Uniform::UInt(data) => {
			gl_call!({
				gl::Uniform1ui(loc, *data);
			});
		}
	};
}

pub fn set_uniforms(uniforms: &mut [UniformItem], shader_program: &ShaderProgram) {
	uniforms
		.iter_mut()
		.enumerate()
		.for_each(|(i, uniform_i)| {
			set_uniform(&uniform_i.uniform, shader_program.uniform_locations[i]);
		});
}

impl ShaderProgram {
	fn compile_shader_program(material: &mut Material, program: &mut ShaderProgram, texture_store: &mut GLTextureIDs ) {
		let id;
		let fs_source = &program.fs_source;

		gl_call!({
			id = gl::CreateProgram();
			program.id = id;

			let vs = Self::compile_shader(gl::VERTEX_SHADER, &program.vs_source[..], material.get_src());
			let fs = Self::compile_shader(gl::FRAGMENT_SHADER, &fs_source[..], material.get_src());

			gl::AttachShader(id, fs);
			gl::AttachShader(id, vs);

			gl::LinkProgram(id);
			gl::ValidateProgram(id);

			let mut info_log = Vec::with_capacity(512);
			info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
			let mut success = gl::FALSE as GLint;
			gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
			if success != gl::TRUE as GLint {
				gl::GetProgramInfoLog(
					id,
					512,
					ptr::null_mut(),
					info_log.as_mut_ptr() as *mut GLchar,
				);
				println!(
					"ERROR::SHADER::PROGRAM::COMPILATION_FAILED: {}\n{}",
					material.get_src(),
					str::from_utf8(&info_log).unwrap()
				);
			}

			// TODO - releace remove shasers
			gl::DeleteShader(vs);
			gl::DeleteShader(fs);
		});

		gl_call!({
			gl::UseProgram(program.id);
		});

		let mut tex_loc;
		let mut c_name;

		for (i, textureItem) in material.get_textures().iter().enumerate() {
			c_name = CString::new(textureItem.name.as_bytes()).unwrap();
			gl_call!({
				tex_loc = gl::GetUniformLocation(program.id, c_name.as_ptr());
			});

			program.texture_locations.push(tex_loc);

			gl_call!({
				gl::Uniform1i(tex_loc, i as i32);
			});
		}

		let uniforms = material.get_uniforms();
		let mut uniform_locations = Vec::<i32>::with_capacity(uniforms.len());

		for uniform in uniforms.iter() {
			let c_name = CString::new(uniform.name.as_bytes()).unwrap();
			let loc;
			gl_call!({
				loc = gl::GetUniformLocation(program.id, c_name.as_ptr());
			});
			uniform_locations.push(loc);
		}

		program.uniform_locations = uniform_locations;
		set_uniforms(uniforms, program);
	}

	fn compile_shader(t: GLenum, src: &str, src_path: &str) -> u32 {
		let id;

		gl_call!({
			id = gl::CreateShader(t);
			let c_str_frag = CString::new(src[..].as_bytes()).unwrap();


			let mut success = gl::FALSE as GLint;
			let mut info_log = Vec::with_capacity(1024);
			info_log.set_len(1024 - 1); // subtract 1 to skip the trailing null character

			gl::ShaderSource(id, 1, &c_str_frag.as_ptr(), ptr::null());
			gl::CompileShader(id);

			// check for shader compile errors
			gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
			if success != gl::TRUE as GLint {
				gl::GetShaderInfoLog(
					id,
					1024,
					ptr::null_mut(),
					info_log.as_mut_ptr() as *mut GLchar,
				);
				match t {
					gl::FRAGMENT_SHADER => println!(
						"ERROR::SHADER::FRAGMENT::COMPILATION_FAILED: {}\n{}",
						src_path,
						str::from_utf8(&info_log).unwrap()
					),
					gl::VERTEX_SHADER => println!(
						"ERROR::SHADER::VERTEX::COMPILATION_FAILED: {}\n{}",
						src_path,
						str::from_utf8(&info_log).unwrap()
					),
					_ => println!(
						"ERROR::SHADER::?::COMPILATION_FAILED: {}\n{}",
						src_path,
						str::from_utf8(&info_log).unwrap()
					),
				};
				gl::DeleteShader(id);
				panic!();
			}
		});

		id
	}
}

pub trait GLMaterial
where
	Self: Sized,
{
	fn get_program(&mut self) -> ShaderProgram;

	fn bind(&mut self, mat_store: &mut GLMaterialIDs, texture_store: &mut GLTextureIDs);

	fn unbind(&self) {
		gl_call!({
			gl::UseProgram(0);
		});
	}
}

impl GLMaterial for Material {
	fn get_program(&mut self) -> ShaderProgram {
		let p = find_file(&["src/render/open_gl/shaders"], self.get_src()).unwrap();
		let code = read_to_string(&p);
		let mut shader_program = ShaderProgram {
			fs_source: String::from(""),
			vs_source: String::from(""),
			id: 0,
			uniform_locations: Vec::new(),
			texture_locations: Vec::new(),
		};

		let mut write_to_prog = ProgramType::None;

		for line in code.lines() {
			if line.starts_with("#<vertex>") {
				write_to_prog = ProgramType::Vertex;
			} else if line.starts_with("#<fragment>") {
				write_to_prog = ProgramType::Fragment;
			} else {
				match write_to_prog {
					ProgramType::Vertex => {
						shader_program.vs_source += line;
						shader_program.vs_source += "\n";
					}
					ProgramType::Fragment => {
						shader_program.fs_source += line;
						shader_program.fs_source += "\n";
					}
					_ => {}
				}
			}
		}

		shader_program
	}

	fn bind(&mut self, mat_store: &mut GLMaterialIDs, texture_store: &mut GLTextureIDs) {
		match mat_store.get_mut(&self.uuid) {
			None => {}
			Some(ref program) => {
				gl_call!({
					gl::UseProgram(program.id);
				});

				self.get_textures()
					.iter()
					.enumerate()
					.for_each(|(i, data)| {
						let loc = program.texture_locations[i];
						if loc == -1 {return;}

						gl_call!({
							gl::ActiveTexture(gl::TEXTURE0 + i as u32);
						});

						match data.texture {
							Some(ref texture) => {
								let texture = texture.lock().unwrap();
								texture.bind(texture_store);
								return;
							}
							None => {}
						}

						gl_call!({
							gl::BindTexture(get_texture_dimensions(&data.dimensions), 0);
						});
					});

				set_uniforms(self.get_uniforms(), program);
				return;
			}
		}

		let mut program = self.get_program();
		ShaderProgram::compile_shader_program(self, &mut program, texture_store);

		mat_store.insert(self.uuid, program);

		self.bind(mat_store, texture_store);
	}
}
