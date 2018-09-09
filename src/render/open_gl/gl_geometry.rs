extern crate byteorder;
extern crate gl;
extern crate uuid;

use self::byteorder::{BigEndian, LittleEndian, WriteBytesExt};
use self::gl::types::*;
use self::uuid::Uuid;
use core::{BufferAttribute, BufferGeometry, BufferType};
use std::collections::HashMap;
use std::mem;
use std::os::raw::c_void;

// #[derive(Debug)]
pub type VertexArraysIDs = HashMap<Uuid, Buffers>;

#[derive(Debug)]
pub struct Buffers {
	vertex_array: GLuint,
	array_buffer: GLuint,
	element_array_buffer: GLuint,
}

impl Drop for Buffers {
	fn drop(&mut self) {
		println!("delete geometry");
		gl_call!({
			gl::DeleteVertexArrays(1, &self.vertex_array);
			gl::DeleteBuffers(1, &self.array_buffer);
			gl::DeleteBuffers(1, &self.element_array_buffer);
		});
	}
}

impl Default for Buffers {
	fn default() -> Self {
		Buffers {
			vertex_array: 0,
			array_buffer: 0,
			element_array_buffer: 0,
		}
	}
}

#[allow(dead_code)]
pub trait GLGeometry {
	fn bind(&self, hash_map: &mut VertexArraysIDs);
	fn unbind(&self);

	fn alloc_gl_gom(geom: &BufferGeometry) -> Buffers {
		let len = geom.attributes.len();
		if len == 0 {
			panic!("empty Geometry");
		}

		let buffer_size = geom.attributes
			.iter()
			.map(|e| {
				let size = Self::elem_byte_len(e);
				size * &e.len()
			})
			.fold(0, |a, b| a + b);

		let vertex_byte_len = geom.attributes
			.iter()
			.map(|e| Self::elem_byte_len(e))
			.fold(0, |a, b| a + b);

		let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

		let positions_len = geom.get_attribute("positions").unwrap().len();

		for i in 0..positions_len {
			for buffer_data in geom.attributes.iter() {
				match &buffer_data.data {
					&BufferType::Vector4(ref v) => {
						buffer.write_f32::<LittleEndian>(v[i].x).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].y).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].z).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].w).unwrap();
					}
					&BufferType::Vector3(ref v) => {
						buffer.write_f32::<LittleEndian>(v[i].x).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].y).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].z).unwrap();
					}
					&BufferType::Vector2(ref v) => {
						buffer.write_f32::<LittleEndian>(v[i].x).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].y).unwrap();
					}
				}
			}
		}

		let indices: &Vec<i32> = geom.indices.as_ref().unwrap();

		let mut vertex_array = 0;
		let mut array_buffer = 0;
		let mut element_array_buffer = 0;

		gl_call!({
			gl::GenVertexArrays(1, &mut vertex_array);
			gl::GenBuffers(1, &mut array_buffer);
			gl::GenBuffers(1, &mut element_array_buffer);

			gl::BindVertexArray(vertex_array);
			gl::BindBuffer(gl::ARRAY_BUFFER, array_buffer);
			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_array_buffer);
		});

		gl_call!({
			gl::BufferData(
				gl::ARRAY_BUFFER,
				buffer.len() as GLsizeiptr,
				&buffer[0] as *const u8 as *const c_void,
				gl::DYNAMIC_DRAW,
			);

			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(mem::size_of::<GLint>() * indices.len()) as GLsizeiptr,
				&indices[0] as *const i32 as *const c_void,
				gl::STATIC_DRAW,
			);
		});

		let mut byte_offset = 0;
		for i in 0..geom.attributes.len() {
			let ref buffer_data = geom.attributes[i];
			let vals;
			let val_type;

			match buffer_data.data {
				BufferType::Vector3(_) => {
					vals = 3;
					val_type = gl::FLOAT;
				}
				BufferType::Vector4(_) => {
					vals = 4;
					val_type = gl::FLOAT;
				}
				BufferType::Vector2(_) => {
					vals = 2;
					val_type = gl::FLOAT;
				}
			}

			println!("=>VertexAttribPointer index:{}, vals:{}, val_type:{}, vertex_byte_len:{} byte_offset:{}", i,vals,val_type, vertex_byte_len, byte_offset );
			println!("Capacyty {}", buffer.len());
			gl_call!({
				gl::VertexAttribPointer(
					i as GLuint,
					vals,
					val_type,
					gl::FALSE,
					vertex_byte_len as GLsizei,
					byte_offset as *const c_void,
				);
				gl::EnableVertexAttribArray(i as GLuint);
			});

			byte_offset += Self::elem_byte_len(buffer_data);
		}

		Buffers {
			vertex_array,
			array_buffer,
			element_array_buffer,
		}
	}

	fn elem_byte_len(attribute: &BufferAttribute) -> usize {
		match &attribute.data {
			&BufferType::Vector2(_) => mem::size_of::<f32>() * 2,
			&BufferType::Vector3(_) => mem::size_of::<f32>() * 3,
			&BufferType::Vector4(_) => mem::size_of::<f32>() * 4,
		}
	}
}

impl GLGeometry for BufferGeometry {
	fn bind(&self, hash_map: &mut VertexArraysIDs) {
		match hash_map.get_mut(&self.uuid) {
			None => {}
			Some(ref buffers) => {
				gl_call!({
					gl::BindVertexArray(buffers.vertex_array);
				});
				return;
			}
		}

		let buffers = Self::alloc_gl_gom(self);
		hash_map.insert(self.uuid, buffers);

		self.bind(hash_map);
	}

	fn unbind(&self) {
		gl_call!({
			gl::BindVertexArray(0);
		});
	}
}
