extern crate gl;
extern crate uuid;
extern crate regex;


use self::gl::types::*;

use core::{Uniform, UniformItem, ShaderProgram};
use std::ffi::{CString};
use std::ptr;
use std::str;
use helpers::{find_file, read_to_string};
use super::gl_texture::{GLTextureIDs, GLTexture};
use super::BindContext;
use self::regex::Regex;


lazy_static! {
	static ref RE_INCLUDE: Regex = Regex::new(r"#include\s+<(\S+)>").unwrap();
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProgramType {
	None,
	Vertex,
	Fragment,
}


#[derive(Debug)]
pub struct UniformLocation {
	location: i32,
	texture_slot: i32,
}

#[derive(Debug)]
pub struct GLShaderProgramID {
	pub fs_source: String,
	pub vs_source: String,
	pub id: GLuint,
	pub uniform_locations: Vec<UniformLocation>,
}

impl Drop for GLShaderProgramID {
	fn drop(&mut self) {
		println!("delete program");
		gl_call!({
			gl::DeleteProgram(self.id);
		});
	}
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
	// allocate buffer of correct size
	let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
	// fill it with len spaces
	buffer.extend([b' '].iter().cycle().take(len));
	// convert buffer to CString
	unsafe { CString::from_vec_unchecked(buffer) }
}


pub fn set_uniform(uniform: &mut Uniform, loc: &UniformLocation, texture_store: &mut GLTextureIDs) {
	if loc.location == -1 {return}

	match uniform {
		Uniform::Vector2(data) => {
			gl_call!({
				gl::Uniform2fv(loc.location, 1, &data.x as *const f32);
			});
		}
		Uniform::Vector3(data) => {
			gl_call!({
				gl::Uniform3fv(loc.location, 1, &data.x as *const f32);
			});
		}
		Uniform::Vector4(data) => {
			gl_call!({
				gl::Uniform4fv(loc.location, 1, &data.x as *const f32);
			});
		}
		Uniform::Matrix3f(data) => {
			gl_call!({
				gl::UniformMatrix3fv(loc.location, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
		Uniform::Matrix4f(data) => {
			gl_call!({
				gl::UniformMatrix4fv(loc.location, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
		Uniform::Float(data) => {
			gl_call!({
				gl::Uniform1f(loc.location, *data);
			});
		}
		Uniform::Int(data) => {
			gl_call!({
				gl::Uniform1i(loc.location, *data);
			});
		}
		Uniform::UInt(data) => {
			gl_call!({
				gl::Uniform1ui(loc.location, *data);
			});
		}
		Uniform::Texture2D(data) => {
			gl_call!({
				gl::ActiveTexture(gl::TEXTURE0 + loc.texture_slot as u32);
			});
			match data {
				Some(ref mut texture) => {
					let mut texture = texture.lock().unwrap();
					texture.bind(texture_store);
				}
				None => {
					gl_call!({
						gl::BindTexture(gl::TEXTURE_2D, 0);
					});
				}
			}
		}
	};
}


pub fn set_uniforms(uniforms: &mut[UniformItem], shader_program: &mut GLShaderProgramID, texture_store: &mut GLTextureIDs) {
	let mut texture_slot = 0;

	uniforms
		.iter_mut()
		.enumerate()
		.for_each(|(i, uniform)| {
			if shader_program.uniform_locations.get(i).is_none() {
				let c_name = CString::new(uniform.name.as_bytes()).unwrap();
				let location;

				gl_call!({
					location = gl::GetUniformLocation(shader_program.id, c_name.as_ptr());
				});

				println!(">>...........{} {}", uniform.name, location);
				if let Uniform::Texture2D( _ ) = uniform.uniform {
					println!("...........{} {}", texture_slot, uniform.name);
					gl_call!({
						gl::Uniform1i(location, texture_slot as i32);
					});
					shader_program.uniform_locations.push(UniformLocation{location, texture_slot: texture_slot});
					texture_slot +=1 ;
				} else {
					shader_program.uniform_locations.push(UniformLocation{location, texture_slot: -1});
				}
			}

			if shader_program.uniform_locations[i].location != -1 {
				match uniform.uniform {
					Uniform::Texture2D( _ ) => {
						set_uniform(&mut uniform.uniform, &shader_program.uniform_locations[i], texture_store);
					}
					_=> {
						if uniform.need_update {
							set_uniform(&mut uniform.uniform, &shader_program.uniform_locations[i], texture_store);
							uniform.need_update = false;
						}
					}
				}
			}
		});
}


pub fn read_shader_file(search_dirs: &Vec<&str>, path: &str) -> String {
	let path = path.to_string() + ".glsl";

	let p = find_file(&["src/render/open_gl/shaders"], &path).unwrap();
	let mut code = read_to_string(&p);

	while let Some(cap) = RE_INCLUDE.captures(&code.clone()) {
		let include_data = read_shader_file(search_dirs, &cap[1]);
		code = code.replace(&cap[0], &include_data);
	}

	code
}


fn set_definitions_fragment<T: ShaderProgram>(code: &String, shader: &T, bind_context: &mut BindContext) -> String {

	let core_definitions = format!("#define NUM_POINT_LIGHTS {}\n", bind_context.render_settings.num_point_lights);

	let definitions: String = bind_context.tags
		.iter()
		.chain(shader.get_tags())
		.map(|e| {
			format!("#define {}\n", e)
		})
		.collect();

	format!("#version 330 core\n{}\n{}\n{}",core_definitions,  definitions, code)
}


fn set_definitions_vertex<T: ShaderProgram>(code: &String, shader: &T, bind_context: &mut BindContext) -> String {

	let definitions: String = bind_context.tags
		.iter()
		.chain(shader.get_tags())
		.map(|e| {
			format!("#define {}\n", e)
		})
		.collect();


	format!("#version 330 core\n{}\n{}", code, definitions)
}



pub fn get_program<T: ShaderProgram>(shader: &T, bind_context: &mut BindContext) -> GLShaderProgramID {
	let code = read_shader_file(&vec!("src/render/open_gl/shaders"), shader.get_src());

	let mut shader_program = GLShaderProgramID {
		fs_source: String::from(""),
		vs_source: String::from(""),
		id: 0,
		uniform_locations: Vec::with_capacity(shader.get_uniforms().len()),
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

	shader_program.fs_source = set_definitions_fragment(&shader_program.fs_source, shader, bind_context);
	shader_program.vs_source = set_definitions_vertex(&shader_program.vs_source, shader, bind_context);

	shader_program
}


pub fn compile_shader_program<T:ShaderProgram>(shader: &mut T, bind_context: &mut BindContext ) -> GLShaderProgramID {
	println!("compile shader: {}", shader.get_src());

	let mut program = get_program(shader, bind_context);
	let id;
	// let fs_source = &program.fs_source;

	gl_call!({
		id = gl::CreateProgram();
		program.id = id;

		let vs = compile_shader(gl::VERTEX_SHADER, &program.vs_source[..], shader.get_src());
		let fs = compile_shader(gl::FRAGMENT_SHADER, &program.fs_source[..], shader.get_src());

		gl::AttachShader(id, fs);
		gl::AttachShader(id, vs);

		gl::LinkProgram(id);
		gl::ValidateProgram(id);

		let info_log = create_whitespace_cstring_with_len(1024);
		let mut success = gl::FALSE as GLint;
		gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetProgramInfoLog(
				id,
				512,
				ptr::null_mut(),
				info_log.as_ptr() as *mut gl::types::GLchar,
			);
			println!(
				"ERROR::SHADER::PROGRAM::COMPILATION_FAILED: {}\n{}",
				shader.get_src(),
				info_log.to_string_lossy()
			);
		}

		// TODO - release remove shaders
		gl::DeleteShader(vs);
		gl::DeleteShader(fs);
	});

	gl_call!({
		gl::UseProgram(program.id);
	});

	let uniforms = shader.get_uniforms_slice_mut();
	set_uniforms(uniforms, &mut program, bind_context.gl_texture_ids);
	program
}



pub fn compile_shader(t: GLenum, src: &str, src_path: &str) -> u32 {
	let id;

	gl_call!({
		id = gl::CreateShader(t);
		let c_str_frag = CString::new(src[..].as_bytes()).unwrap();


		let mut success = gl::FALSE as GLint;
		let info_log = create_whitespace_cstring_with_len(1024);

		gl::ShaderSource(id, 1, &c_str_frag.as_ptr(), ptr::null());
		gl::CompileShader(id);

		// check for shader compile errors
		gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
		if success != gl::TRUE as GLint {
			gl::GetShaderInfoLog(
				id,
				1024,
				ptr::null_mut(),
				info_log.as_ptr() as *mut gl::types::GLchar
			);
			match t {
				gl::FRAGMENT_SHADER => println!(
					"ERROR::SHADER::FRAGMENT::COMPILATION_FAILED: {}\n{}",
					src_path,
					info_log.to_string_lossy()
				),
				gl::VERTEX_SHADER => println!(
					"ERROR::SHADER::VERTEX::COMPILATION_FAILED: {}\n{}",
					src_path,
					info_log.to_string_lossy()
				),
				_ => println!(
					"ERROR::SHADER::?::COMPILATION_FAILED: {}\n{}",
					src_path,
					info_log.to_string_lossy()
				),
			};
			gl::DeleteShader(id);
			panic!();
		}
	});

	id
}


