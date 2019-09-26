extern crate gl;
extern crate uuid;
extern crate regex;
extern crate heck;

use self::gl::types::*;

use core::{
	Uniform,
	UniformItem,
	ShaderProgram,
	ShaderTag,
};
use std::ffi::{CString};
use std::ptr;
use std::str;
use helpers::{find_file, read_to_string};
use super::gl_texture::{GLTextureIDs, GLTexture};
use super::BindContext;
use self::regex::Regex;
use self::heck::ShoutySnakeCase;


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
		Uniform::Texture2D(data, _) => {
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
				let c_name = CString::new(uniform.name.get_name().as_bytes()).unwrap();
				let location;

				gl_call!({
					location = gl::GetUniformLocation(shader_program.id, c_name.as_ptr());
				});

				// println!(">>...........{} {}", uniform.name, location);
				if let Uniform::Texture2D( _, _ ) = uniform.uniform {
					// println!("...........{} {}", texture_slot, uniform.name);
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
					Uniform::Texture2D( _, _ ) => {
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


fn set_definitions_fragment<T: ShaderProgram>(code: &String, shader: &T, bind_context: &mut BindContext) -> String {
	let core_definitions = format!(
		r###"
		#define NUM_POINT_LIGHTS {}
		#define NUM_DIR_LIGHTS {}
		"###,
		bind_context.lights_point_count,
		bind_context.lights_directional_count
	);

	let textures: String = shader.get_uniforms()
		.iter()
		.filter(|e| {
			match e.uniform {
				Uniform::Texture2D(_, _) => true,
				_ => false,
			}
		})
		.map(|e| {
			if let Uniform::Texture2D(_, n) = e.uniform {
				let texture = e.name.get_name().to_shouty_snake_case();
				return format!("#define {}\n#define {}_UV_INDEX = {}\n", texture, texture, n);
			}
			"".to_string()
		})
		.collect();

	let definitions: String = bind_context.tags
		.iter()
		.chain(shader.get_tags())
		.map(|e| {
			format!("#define {}\n", e.definition())
		})
		.chain(bind_context.geometry.attributes.iter().map(|attribute| format!("#define {}\n", attribute.definition()) ))
		.collect();

		// println!("<><><><<><><><>><><<><><\n{}", core_definitions);
		// println!("..................\n{}..................\n", textures);

	format!("#version 330 core\n{}\n{}\n{}\n{}", core_definitions,  definitions, textures, code)
}


fn set_definitions_vertex<T: ShaderProgram>(code: &String, shader: &T, bind_context: &mut BindContext) -> String {
	let textures: String = shader.get_uniforms()
		.iter()
		.filter(|e| {
			match e.uniform {
				Uniform::Texture2D(_, _) => true,
				_ => false,
			}
		})
		.map(|e| {
			if let Uniform::Texture2D(_, n) = e.uniform {
				let texture = e.name.get_name().to_shouty_snake_case();
				return format!("#define {}\n#define {}_UV_INDEX = {}\n", texture, texture, n);
			}
			"".to_string()
		})
		.collect();

	let definitions: String = bind_context.tags
		.iter()
		.chain(shader.get_tags())
		.map(|tag| {
			format!("#define {}\n", tag.definition())
		})
		.chain(bind_context.geometry.attributes.iter().map(|attribute| format!("#define {}\n", attribute.definition()) ))
		.collect();


	format!("#version 330 core\n{}\n{}\n{}", definitions, textures, code)
}



pub fn get_program<T: ShaderProgram>(shader: &T, bind_context: &mut BindContext) -> GLShaderProgramID {
	let code = read_shader_file(bind_context, shader.get_src());

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

	// println!("=============================================");
	// println!("{}", shader_program.vs_source);
	// println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
	// println!("{}", shader_program.fs_source);
	// println!("=============================================");

	shader_program
}


pub fn compile_shader_program<T:ShaderProgram>(shader: &mut T, bind_context: &mut BindContext ) -> GLShaderProgramID {
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


trait GLShaderTag {
	fn definition(&self) -> &str;
}

impl GLShaderTag for ShaderTag {
	fn definition(&self) -> &str {
		match self {
			// ShaderTag::VertexUV => "VERTEX_UV_0_VEC2",
			// ShaderTag::VertexColor4 => "VERTEX_COLOR_0_VEC4",
			// ShaderTag::VertexColor3 => "VERTEX_COLOR_0_VEC3",
			// ShaderTag::VertexNormal => "VERTEX_NORMAL",
			// ShaderTag::VertexPosition => "VERTEX_POSITION",

			ShaderTag::Lighting => "LIGHTING",
			ShaderTag::Metalness => "METALNESS",
			ShaderTag::AmbientLight => "AMBIENT_LIGHT",
			ShaderTag::Transparent => "TRANSPARENT",
			ShaderTag::Emissive => "EMISSIVE",

			ShaderTag::Other(data) => data,
		}
	}
}
