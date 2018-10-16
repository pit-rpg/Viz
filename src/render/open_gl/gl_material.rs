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


pub type GLMaterialIDs = HashMap<Uuid, GLShaderProgramID>;


pub trait GLMaterial
where
	Self: Sized,
{
	fn bind(&mut self, mat_store: &mut GLMaterialIDs, texture_store: &mut GLTextureIDs);

	fn unbind(&self) {
		gl_call!({
			gl::UseProgram(0);
		});
	}
}




impl GLMaterial for Material {

	fn bind(&mut self, mat_store: &mut GLMaterialIDs, texture_store: &mut GLTextureIDs) {
		let src = self.get_src().to_string();

		match mat_store.get_mut(&self.uuid) {
			None => {}
			Some(ref program) => {
				gl_call!({
					gl::UseProgram(program.id);
				});

				set_uniforms(self.get_uniforms(), program, texture_store);
				return;
			}
		}

		let program;
		{
			let uniforms = self.get_uniforms();
			program = compile_shader_program(&src, uniforms, texture_store);
		}

		mat_store.insert(self.uuid, program);

		self.bind(mat_store, texture_store);
	}
}
