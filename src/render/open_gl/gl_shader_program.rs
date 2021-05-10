extern crate gl;
extern crate heck;
extern crate regex;
extern crate uuid;

use self::gl::types::*;

use self::heck::ShoutySnakeCase;
use self::regex::Regex;
use super::gl_texture::{GLTexture, GLTextureIDs};
use super::BindContext;
use core::{ShaderProgram, ShaderDef, Uniform, UniformName};
use helpers::{find_file, read_to_string};
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::str;

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
	pub uniform_locations: HashMap<UniformName, UniformLocation>,
	pub texture_slots: i32,
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

pub fn set_uniform(uniform: Uniform, loc: &UniformLocation, texture_store: &mut GLTextureIDs) {
	if loc.location == -1 {
		return;
	}

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
		Uniform::Matrix3(data) => {
			gl_call!({
				gl::UniformMatrix3fv(loc.location, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
		Uniform::Matrix4(data) => {
			gl_call!({
				gl::UniformMatrix4fv(loc.location, 1, gl::FALSE, &data.elements[0] as *const f32);
			});
		}
		Uniform::Float(data) => {
			gl_call!({
				gl::Uniform1f(loc.location, data);
			});
		}
		Uniform::Int(data) => {
			gl_call!({
				gl::Uniform1i(loc.location, data);
			});
		}
		Uniform::UInt(data) => {
			gl_call!({
				gl::Uniform1ui(loc.location, data);
			});
		}
		Uniform::Texture2D(mut data, _) => {
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

pub fn set_uniforms(
	uniforms: &HashMap<UniformName, Uniform>,
	shader_program: &mut ShaderProgram,
	shader_program_id: &mut GLShaderProgramID,
	texture_store: &mut GLTextureIDs,
	force: bool,
) {
	uniforms.iter().for_each(|(name, uniform)| {
		// TODO: do not rebind textures if material is not changed
		let is_texture = match uniform {
			Uniform::Texture2D(_, _) => true,
			_ => false,
		};

		if !force && !shader_program.set_uniform(name, (*uniform).clone()) && !is_texture {
			return;
		}

		if shader_program_id.uniform_locations.get(name).is_none() {
			let c_name = CString::new(name.get_name().as_bytes()).unwrap();
			let location;

			gl_call!({
				location = gl::GetUniformLocation(shader_program_id.id, c_name.as_ptr());
			});

			println!(">>...........{:?} {}", name, location);
			if let Uniform::Texture2D(_, _) = uniform {
				println!("...........{} {:?}", shader_program_id.texture_slots, name);
				gl_call!({
					gl::Uniform1i(location, shader_program_id.texture_slots);
				});
				shader_program_id.uniform_locations.insert(
					name.clone(),
					UniformLocation {
						location,
						texture_slot: shader_program_id.texture_slots,
					},
				);
				shader_program_id.texture_slots += 1;
			} else {
				shader_program_id.uniform_locations.insert(
					name.clone(),
					UniformLocation {
						location,
						texture_slot: -1,
					},
				);
			}
		}

		if let Some(uniform_location) = shader_program_id.uniform_locations.get(name) {
			set_uniform(uniform.clone(), uniform_location, texture_store);
		}
	});
}

pub fn read_shader_file(bind_context: &BindContext, path: &str) -> String {
	let path = path.to_string() + ".glsl";

	let p = find_file(&["res/shaders"], &path).unwrap();
	let mut code = read_to_string(&p);

	// let mut code = bind_context.shader_sources
	// 	.iter()
	// 	.find(|e| e.name == path)
	// 	.unwrap()
	// 	.src.to_string();

	while let Some(cap) = RE_INCLUDE.captures(&code.clone()) {
		let include_data = read_shader_file(bind_context, &cap[1]);
		code = code.replace(&cap[0], &include_data);
	}

	code
}

fn set_definitions(code: &String, shader: &ShaderProgram, bind_context: &mut BindContext) -> String {
	let textures: String = shader
		.get_uniforms()
		.iter()
		.map(|(name, uniform)| {
			if let Uniform::Texture2D(_, n) = uniform {
				let texture = name.get_name().to_shouty_snake_case();
				return format!("#define {}\n#define {}_UV_INDEX = {}\n", texture, texture, n);
			}
			"".to_string()
		})
		.collect();

	let definitions: String = shader.get_definitions()
		.iter()
		.map(|(key, val)| format!("#define {} {}\n", key.definition(), val))
		.chain(
			bind_context
				.geometry
				.attributes
				.iter()
				.map(|attribute| format!("#define {}\n", attribute.definition())),
		)
		.collect();

	format!("#version 330 core\n{}\n{}\n{}", definitions, textures, code)
}

pub fn get_program(shader: &ShaderProgram, bind_context: &mut BindContext) -> GLShaderProgramID {
	let code = read_shader_file(bind_context, shader.get_src());

	let mut shader_program = GLShaderProgramID {
		fs_source: String::from(""),
		vs_source: String::from(""),
		id: 0,
		uniform_locations: HashMap::new(),
		texture_slots: 0,
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

	shader_program.fs_source = set_definitions(&shader_program.fs_source, shader, bind_context);
	shader_program.vs_source = set_definitions(&shader_program.vs_source, shader, bind_context);

	println!("TAGS==========================================");
	println!("{:?}", shader.get_definitions());
	println!("/TAGS=========================================");

	// println!("=============================================");
	// println!("{}", shader_program.vs_source);
	// println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
	// println!("{}", shader_program.fs_source);
	// println!("=============================================");

	shader_program
}

pub fn compile_shader_program(shader: &ShaderProgram, bind_context: &mut BindContext) -> GLShaderProgramID {
	// println!("compile shader: {}", shader.get_src());

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
			gl::GetProgramInfoLog(id, 512, ptr::null_mut(), info_log.as_ptr() as *mut gl::types::GLchar);
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

	// let uniforms = shader.get_uniforms_slice_mut();
	// set_uniforms(uniforms, &mut program, bind_context.gl_texture_ids);
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
			gl::GetShaderInfoLog(id, 1024, ptr::null_mut(), info_log.as_ptr() as *mut gl::types::GLchar);
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

trait GLShaderDef {
	fn definition(&self) -> &str;
}

impl GLShaderDef for ShaderDef {
	fn definition(&self) -> &str {
		match self {
			ShaderDef::Lighting => "LIGHTING",
			ShaderDef::Metalness => "METALNESS",
			ShaderDef::AmbientLight => "AMBIENT_LIGHT",
			ShaderDef::Transparent => "TRANSPARENT",
			ShaderDef::Additive => "ADDITIVE",
			ShaderDef::Emissive => "EMISSIVE",
			ShaderDef::Shadeless => "SHADELESS",
			ShaderDef::ReceiveShadows => "RECEIVE_SHADOWS",
			ShaderDef::CastShadows => "CAST_SHADOWS",

			ShaderDef::Other(data) => data,
		}
	}
}

// fn get_blending_tags(blending: Blending) -> HashSet<ShaderDef> {
// 	let mut set = HashSet::new();

// 	match blending {
// 		Blending::None => {}
// 		Blending::Transparent => {
// 			set.insert(ShaderDef::Transparent);
// 		}
// 		Blending::Additive => {
// 			set.insert(ShaderDef::Transparent);
// 			set.insert(ShaderDef::Additive);
// 		}
// 	};

// 	set
// }
