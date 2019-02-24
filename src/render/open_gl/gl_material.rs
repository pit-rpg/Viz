extern crate gl;
extern crate uuid;

use self::gl::types::*;
use self::uuid::Uuid;

use core::{Material, Uniform, UniformItem, ShaderProgram};
use std::collections::HashMap;
use std::path::Path;
use std::ffi::{CString};
use std::ptr;
use std::str;
use helpers::{find_file, read_to_string};
use super::gl_texture::{load_texture, GLTextureIDs, TextureId, GLTexture};
use super::gl_shaderProgram::{compile_shader_program, GLShaderProgramID, read_shader_file, ProgramType, set_uniforms};
use super::BindContext;

pub type GLMaterialIDs = HashMap<Uuid, GLShaderProgramID>;


pub trait GLMaterial
where
	Self: Sized,
{
	fn bind(&mut self, bind_context: &mut BindContext);

	fn unbind(&self) {
		gl_call!({
			gl::UseProgram(0);
		});
	}
}




impl GLMaterial for Material {

	fn bind(&mut self, bind_context: &mut BindContext) {
		let src = self.get_src().to_string();

		match bind_context.gl_material_ids.get_mut(&self.uuid) {
			None => {}
			Some(ref program) => {
				gl_call!({
					gl::UseProgram(program.id);
				});

				set_uniforms(self.get_uniforms(), program, bind_context.gl_texture_ids);
				return;
			}
		}

		let program;
		{
			let uniforms = self.get_uniforms();
			program = compile_shader_program(&src, uniforms, bind_context);
		}

		bind_context.gl_material_ids.insert(self.uuid, program);

		self.bind(bind_context);
	}
}
