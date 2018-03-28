extern crate gl;

use self::gl::types::*;
use std::mem;


use core::BufferGeometry;
use std::collections::HashMap;
// use std::sync::{Mutex, Arc};
extern crate uuid;
use self::uuid::Uuid;
// let mut book_reviews = HashMap::new();

#[derive(Clone, Debug)]
pub struct Buffers {
	vertex_array: GLuint,
	array_buffer: GLuint,
	element_array_buffer: Option<GLuint>,
}

impl Default for Buffers {
	fn default() -> Self {
		Buffers{
			vertex_array: 0,
			array_buffer: 0,
			element_array_buffer: None,
		}
	}
}

pub type VartexArrays<'a> = HashMap<Uuid, Buffers>;

// pub static mut VARTEX_ARRAYS:  Option<Mutex<VartexArrays>> = None;
// pub static mut VARTEX_ARRAYS: Option<HashMap<i32, i32>> = None;

#[allow(dead_code)]
pub trait GLGeometry {
	fn bind(&self, hash_map: &mut VartexArrays);
	fn un_bind(&self);
	fn alloc_gl_gom(&self, hash_map: & mut VartexArrays) -> Buffers;
}



// #[allow(dead_code)]
// pub fn init() {
// 	unsafe {
// 		match VARTEX_ARRAYS {
// 			None => { VARTEX_ARRAYS = Some(Mutex::new(HashMap::new())); }
// 			Some(_) =>{}
// 		}
// 	}
// }

impl GLGeometry for BufferGeometry {

	fn alloc_gl_gom(&self, hash_map: &mut VartexArrays) -> Buffers {
		match hash_map.get(&self.uuid) {
			Some(val) => {return val.clone();}
			None =>{}
		};

		let mut buffers = Buffers::default();

		let mut byte_len = 0;
		// let mut
		for attribute in self.attributes.iter() {
			byte_len += attribute.byte_len();
		}





		// gl_call!({
		// 	gl::GenVertexArrays(1, &mut buffers.vertex_array);
		// 	gl::GenBuffers(1, &mut buffers.array_buffer);

		// 	gl::BindVertexArray(buffers.vertex_array);
		// 	gl::BindBuffer(gl::ARRAY_BUFFER, buffers.array_buffer);

		// 	gl::BufferData(
		// 		gl::ARRAY_BUFFER,
		// 		(mem::size_of::<GLfloat>() * positions.len()) as GLsizeiptr,
		// 		&positions[0] as *const f32 as *const c_void,
		// 		gl::DYNAMIC_DRAW
		// 	);
		// });


		// gl_call!({


		// 	gl::GenBuffers(1, &mut EBO);
		// 	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);


		// 	gl::BufferData(
		// 		gl::ELEMENT_ARRAY_BUFFER,
		// 		(mem::size_of::<GLint>() * indices.len()) as GLsizeiptr,
		// 		&indices[0] as *const i32 as *const c_void,
		// 		gl::STATIC_DRAW
		// 	);

		// 	gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<GLfloat>() as GLsizei, 0 as *const c_void);
		// 	gl::EnableVertexAttribArray(0);
		// 	gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void );
		// 	gl::EnableVertexAttribArray(1);
		// });









		// let id = 1;
		// hash_map.insert(self.uuid.clone() , id);
		buffers
	}


	fn bind(&self, hash_map: &mut VartexArrays) {
		let id = self.alloc_gl_gom(hash_map);

		// let option = hash_map.get(&self.uuid);
		// let id = match option {
		// 	Some(gl_id) => {
		// 		gl_id
		// 	}
		// 	None => {
		// 		self.alloc_gl_gom(hash_map)
		// 	}
		// };
		// VartexArrays.insert("11", 123);
	}

	fn un_bind(&self) {
	}
}