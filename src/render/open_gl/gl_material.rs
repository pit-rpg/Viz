extern crate gl;
extern crate uuid;

use self::uuid::Uuid;

use core::{Material};

use super::{
	gl_shader_program::{compile_shader_program, set_uniforms, GLShaderProgramID},
	BindContext,
};
use std::collections::HashMap;

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
		{
			let mut shader_program = self.get_shader_program().unwrap();

			if shader_program.is_need_update() {
				bind_context.gl_material_ids.remove(&shader_program.get_uuid());
				shader_program.set_need_update(false);
			} else {
				match bind_context.gl_material_ids.get_mut(&shader_program.get_uuid()) {
					None => {}
					Some(program) => {
						gl_call!({
							gl::UseProgram(program.id);
						});

						set_uniforms(
							&self.uniforms,
							&mut *shader_program,
							program,
							bind_context.gl_texture_ids,
							false,
						);
						return;
					}
				}
			}

			self.uniforms.iter().for_each(|(key, val)| {
				shader_program.set_uniform(key, val.clone());
			});
			let mut program = compile_shader_program(&*shader_program, bind_context);
			set_uniforms(
				&self.uniforms,
				&mut *shader_program,
				&mut program,
				bind_context.gl_texture_ids,
				true,
			);

			bind_context.gl_material_ids.insert(shader_program.get_uuid(), program);
		}

		self.bind(bind_context);
	}
}
