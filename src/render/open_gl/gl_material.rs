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
use super::gl_texture::{load_texture, GLTextureIDs, TextureId};

pub type GLMaterialIDs = HashMap<Uuid, ShaderProgram>;

#[derive(Debug)]
pub struct ShaderProgram {
	fs_source: String,
	vs_source: String,
	id: GLuint,
	uniform_locations: Vec<i32>,
}

impl Drop for ShaderProgram {
	fn drop(&mut self) {
		println!("delete program");
		gl_call!({
			gl::DeleteProgram(self.id);
		});
	}
}

fn get_gl_texture_uniform_name(name: &str, texture_id: &TextureId) -> String {
	let u_type = match texture_id.gl_texture_dimensions {
		gl::TEXTURE_1D => "sampler1D",
		gl::TEXTURE_2D => "sampler2D",
		gl::TEXTURE_3D => "sampler3D",
		_ => {
			panic!();
		}
	};

	format!("uniform {} {};\n", u_type, name)
}

fn get_gl_uniform_name(uniform_item: &UniformItem) -> String {
	// uniform sampler2D map_color;
	let u_type = match uniform_item.uniform {
		Uniform::Vector4(_) => "vec4",
		Uniform::Vector3(_) => "vec3",
		Uniform::Vector2(_) => "vec2",
		Uniform::Matrix4(_) => "mat4",
	};

	format!("uniform {} {};\n", u_type, uniform_item.name)
}

pub fn set_uniform(u: &Uniform, loc: i32) {
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
		Uniform::Matrix4(data) => {
			gl_call!({
				gl::UniformMatrix4fv(loc, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
	};
}

pub fn set_uniforms(uniforms: &mut [UniformItem], shader_program: &ShaderProgram) {
	uniforms
		.iter_mut()
		.enumerate()
		.filter(|(_, e)| e.need_update)
		.for_each(|(i, uniform_i)| {
			set_uniform(&uniform_i.uniform, shader_program.uniform_locations[i]);
			uniform_i.need_update = false;
		});
}

impl ShaderProgram {
	fn compile_shader_program(
		material: &mut Material,
		program: &mut ShaderProgram,
		texture_store: &mut GLTextureIDs,
	) {
		let mut texture_uniforms = String::new();
		let mut texture_data = Vec::new();

		for data in material.get_textures() {
			let texture = data.texture.lock().unwrap();

			if texture_store.get(&texture.uuid).is_none() {
				let id = load_texture(&*texture).unwrap();
				texture_store.insert(texture.uuid, id);
			}

			let texture_id = texture_store.get(&texture.uuid).unwrap();
			let uniform_name = get_gl_texture_uniform_name(&data.name[..], texture_id);
			texture_uniforms.push_str(&uniform_name[..]);
			texture_data.push((
				data.name.clone(),
				texture_id.id,
				texture_id.gl_texture_dimensions,
			));
		}

		let id;
		let fs_source = program
			.fs_source
			.replace("#<textures>", &texture_uniforms[..]);

		// println!("{}", &program.vs_source);
		// println!("{}", fs_source);

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
					"ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
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

		// println!("{:?}", texture_data);

		for (i, (name, tid, gl_texture_dimensions)) in texture_data.iter().enumerate() {
			c_name = CString::new(name.as_bytes()).unwrap();

			gl_call!({
				gl::BindTexture(*gl_texture_dimensions, *tid);
			});
			gl_call!({
				tex_loc = gl::GetUniformLocation(program.id, c_name.as_ptr());
			});

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
				gl::GetShaderInfoLog(
					id,
					512,
					ptr::null_mut(),
					info_log.as_mut_ptr() as *mut GLchar,
				);
				match t {
					gl::FRAGMENT_SHADER => println!(
						"ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
						str::from_utf8(&info_log).unwrap()
					),
					gl::VERTEX_SHADER => println!(
						"ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
						str::from_utf8(&info_log).unwrap()
					),
					_ => println!(
						"ERROR::SHADER::?::COMPILATION_FAILED\n{}",
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

			if shader_program.vs_source.contains("#<uniforms>") {
				let uniforms: String = self.get_uniforms()
					.iter()
					.filter(|e| e.program_type == ProgramType::Vertex)
					.map(|e| get_gl_uniform_name(e) + "\n")
					.collect();
				shader_program.vs_source = shader_program
					.vs_source
					.replace("#<uniforms>", &uniforms[..])
			}
			if shader_program.fs_source.contains("#<uniforms>") {
				let uniforms: String = self.get_uniforms()
					.iter()
					.filter(|e| e.program_type == ProgramType::Fragment)
					.map(|e| get_gl_uniform_name(e) + "\n")
					.collect();
				shader_program.fs_source = shader_program
					.fs_source
					.replace("#<uniforms>", &uniforms[..])
			}
		}

		println!("{:?}", shader_program);

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
						let texture = data.texture.lock().unwrap();
						let texture_id = texture_store.get(&texture.uuid).unwrap();

						gl_call!({
							gl::ActiveTexture(gl::TEXTURE0 + i as u32);
							gl::BindTexture(texture_id.gl_texture_dimensions, texture_id.id);
						});
					});

				if self.uniform_need_update {
					set_uniforms(self.get_uniforms(), program);
					self.uniform_need_update = false;
				}

				return;
			}
		}

		let mut program = self.get_program();
		ShaderProgram::compile_shader_program(self, &mut program, texture_store);

		mat_store.insert(self.uuid, program);

		self.bind(mat_store, texture_store);
	}
}
