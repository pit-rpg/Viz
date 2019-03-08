extern crate byteorder;
extern crate gl;
extern crate uuid;

use self::byteorder::{LittleEndian, WriteBytesExt};
use self::gl::types::*;
use self::uuid::Uuid;
use core::{BufferAttribute, BufferGeometry, BufferData, BufferType};
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
		let buffers: Vec<_> = geom.iter_attributes().collect();

		let len = buffers.len();
		if len == 0 {
			panic!("empty Geometry");
		}

		let buffer_size = buffers
			.iter()
			.map(|e| {
				let size = e.data.elem_byte_len();
				size * &e.len()
			})
			.fold(0, |a, b| a + b);

		let vertex_byte_len = buffers
			.iter()
			.map(|e| e.data.elem_byte_len())
			.fold(0, |a, b| a + b);

		let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

		let positions_len = geom.get_attribute(BufferType::Position).unwrap().len();
		buffers.iter().for_each(|b| {
			println!("BUFFER_DATA: {:?}", b.buffer_type);
		});

		for i in 0..positions_len {
			for buffer_data in buffers.iter() {
				match &buffer_data.data {
					BufferData::Matrix4(v) => {
						buffer.write_f32::<LittleEndian>(v[i][0]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][1]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][2]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][3]).unwrap();

						buffer.write_f32::<LittleEndian>(v[i][4]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][5]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][6]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][7]).unwrap();

						buffer.write_f32::<LittleEndian>(v[i][8]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][9]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][10]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][11]).unwrap();

						buffer.write_f32::<LittleEndian>(v[i][12]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][13]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][14]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][15]).unwrap();
					}
					BufferData::Matrix3(v) => {
						buffer.write_f32::<LittleEndian>(v[i][0]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][1]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][2]).unwrap();

						buffer.write_f32::<LittleEndian>(v[i][3]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][4]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][5]).unwrap();

						buffer.write_f32::<LittleEndian>(v[i][6]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][7]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][8]).unwrap();
					}
					BufferData::Matrix2(v) => {
						buffer.write_f32::<LittleEndian>(v[i][0]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][1]).unwrap();

						buffer.write_f32::<LittleEndian>(v[i][2]).unwrap();
						buffer.write_f32::<LittleEndian>(v[i][3]).unwrap();
					}
					BufferData::Vector4(v) => {
						buffer.write_f32::<LittleEndian>(v[i].x).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].y).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].z).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].w).unwrap();
					}
					BufferData::Vector3(v) => {
						buffer.write_f32::<LittleEndian>(v[i].x).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].y).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].z).unwrap();
					}
					BufferData::Vector2(v) => {
						buffer.write_f32::<LittleEndian>(v[i].x).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].y).unwrap();
					}
					BufferData::F32(v) => {
						buffer.write_f32::<LittleEndian>(v[i]).unwrap();
					}
					BufferData::I32(v) => {
						buffer.write_i32::<LittleEndian>(v[i]).unwrap();
					}
					BufferData::U32(v) => {
						buffer.write_u32::<LittleEndian>(v[i]).unwrap();
					}
					BufferData::I16(v) => {
						buffer.write_i16::<LittleEndian>(v[i]).unwrap();
					}
					BufferData::U16(v) => {
						buffer.write_u16::<LittleEndian>(v[i]).unwrap();
					}
					BufferData::I8(v) => {
						buffer.write_i8(v[i]).unwrap();
					}
					BufferData::U8(v) => {
						buffer.write_u8(v[i]).unwrap();
					}
				}
			}
		}

		let indices: &Vec<u32> = geom.indices.as_ref().unwrap();

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
				&indices[0] as *const u32 as *const c_void,
				gl::STATIC_DRAW,
			);
		});

		let mut byte_offset = 0;
		for i in 0..buffers.len() {
			let ref buffer_data = buffers[i];
			let vals = buffer_data.data.item_size() as i32;
			let val_type;

			match buffer_data.data {
				BufferData::Matrix2(_) => {
					val_type = gl::FLOAT;
				}
				BufferData::Matrix3(_) => {
					val_type = gl::FLOAT;
				}
				BufferData::Matrix4(_) => {
					val_type = gl::FLOAT;
				}
				BufferData::Vector2(_) => {
					val_type = gl::FLOAT;
				}
				BufferData::Vector3(_) => {
					val_type = gl::FLOAT;
				}
				BufferData::Vector4(_) => {
					val_type = gl::FLOAT;
				}
				BufferData::F32(_) => {
					val_type = gl::FLOAT;
				}
				BufferData::I32(_) => {
					val_type = gl::INT;
				}
				BufferData::U32(_) => {
					val_type = gl::UNSIGNED_INT;
				}
				BufferData::I16(_) => {
					val_type = gl::SHORT;
				}
				BufferData::U16(_) => {
					val_type = gl::UNSIGNED_SHORT;
				}
				BufferData::I8(_) => {
					val_type = gl::BYTE;
				}
				BufferData::U8(_) => {
					val_type = gl::UNSIGNED_BYTE;
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

			byte_offset += buffer_data.data.elem_byte_len();
		}

		Buffers {
			vertex_array,
			array_buffer,
			element_array_buffer,
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
