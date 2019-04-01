extern crate gl;
extern crate uuid;

use self::uuid::Uuid;

use core::{
	Material,
	ShaderProgram
};

use std::collections::HashMap;
use super::{
	BindContext,
	gl_shader_program::{
		compile_shader_program,
		GLShaderProgramID,
		set_uniforms,
	},
};

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

		if self.is_need_update() {
			bind_context.gl_material_ids.remove(&self.get_uuid());
			self.set_need_update(false);
		} else {
			match bind_context.gl_material_ids.get_mut(&self.get_uuid()) {
				None => {}
				Some(program) => {
					gl_call!({
						gl::UseProgram(program.id);
					});

					set_uniforms(self.get_uniforms_slice_mut(), program, bind_context.gl_texture_ids);
					return;
				}
			}
		}

		let program = compile_shader_program(self, bind_context);

		bind_context.gl_material_ids.insert(self.get_uuid(), program);

		self.bind(bind_context);
	}
}
