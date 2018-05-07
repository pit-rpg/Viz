extern crate gl;
extern crate uuid;
extern crate byteorder;

// #[macro_use]
// use render::render_gl::macros;
use std::mem;
use self::gl::types::*;
use self::byteorder::{BigEndian, WriteBytesExt, LittleEndian};
use std::os::raw::c_void;
use core::{BufferGeometry, BufferType, BufferAttribute};
use std::collections::HashMap;
use self::uuid::Uuid;


pub type VartexArraysIDs = HashMap<Uuid, Buffers>;


#[derive(Debug)]
pub struct Buffers {
	vertex_array: GLuint,
	array_buffer: GLuint,
	element_array_buffer: GLuint,
}


impl Drop for Buffers {
	fn drop(&mut self) {
		gl_call!({
			gl::DeleteVertexArrays(1, &self.vertex_array);
        	gl::DeleteBuffers(1, &self.array_buffer);
        	gl::DeleteBuffers(1, &self.element_array_buffer);
		});
	}
}


impl Default for Buffers {
	fn default() -> Self {
		Buffers{
			vertex_array: 0,
			array_buffer: 0,
			element_array_buffer: 0,
		}
	}
}




#[allow(dead_code)]
pub trait GLGeometry {
	fn bind(&self, hash_map: &mut VartexArraysIDs);
	fn un_bind(&self);

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
			.fold(0, |a,b| a+b);

		let vertex_byte_len = geom.attributes
			.iter()
			.map(|e| {
				Self::elem_byte_len(e)
			})
			.fold(0, |a,b| a+b);


		let mut buffer: Vec<u8> = Vec::with_capacity(buffer_size);

		let positions_len = geom.get_attribute("positions").unwrap().len();

		for i in 0..positions_len {
			for buffer_data in geom.attributes.iter() {
				match &buffer_data.data {
					&BufferType::Vector3f32(ref v) => {
						buffer.write_f32::<LittleEndian>(v[i].x).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].y).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].z).unwrap();
					},
					&BufferType::Vector3f64(ref v) => {
						buffer.write_f64::<LittleEndian>(v[i].x).unwrap();
						buffer.write_f64::<LittleEndian>(v[i].y).unwrap();
						buffer.write_f64::<LittleEndian>(v[i].z).unwrap();
					},
					&BufferType::Colorf32(ref v) => {
						buffer.write_f32::<LittleEndian>(v[i].r).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].g).unwrap();
						buffer.write_f32::<LittleEndian>(v[i].b).unwrap();
					},
					&BufferType::Colorf64(ref v) => {
						buffer.write_f64::<LittleEndian>(v[i].r).unwrap();
						buffer.write_f64::<LittleEndian>(v[i].g).unwrap();
						buffer.write_f64::<LittleEndian>(v[i].b).unwrap();
					},
				}
			}
		}

		let  indices: &Vec<i32> = geom.indices.as_ref().unwrap();

		let mut vertex_array = 0;
		let mut array_buffer = 0;
		let mut element_array_buffer = 0;

		// gl_call!({});
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
				gl::DYNAMIC_DRAW
			);

			gl::BufferData(
				gl::ELEMENT_ARRAY_BUFFER,
				(mem::size_of::<GLint>() * indices.len()) as GLsizeiptr,
				&indices[0] as *const i32 as *const c_void,
				gl::STATIC_DRAW
			);
		});

		let mut byte_offset = 0;
		for i in 0..geom.attributes.len() {
			let ref buffer_data = geom.attributes[i];
			let vals;
			let val_type;

			match buffer_data.data {
					BufferType::Vector3f32(_) => {
						vals = 3;
						val_type = gl::FLOAT;
					},
					BufferType::Vector3f64(_) => {
						vals = 3;
						val_type = gl::DOUBLE;
					},
					BufferType::Colorf32(_) => {
						vals = 3;
						val_type = gl::FLOAT;
					},
					BufferType::Colorf64(_) => {
						vals = 3;
						val_type = gl::DOUBLE;
					},
				}

			println!("=>VertexAttribPointer index:{}, vals:{}, val_type:{}, vertex_byte_len:{} byte_offset:{}", i,vals,val_type, vertex_byte_len, byte_offset );
			println!("Capacyty {}", buffer.len());
			gl_call!({
				gl::VertexAttribPointer( i as GLuint, vals, val_type, gl::FALSE, vertex_byte_len as GLsizei, byte_offset as *const c_void );
				gl::EnableVertexAttribArray( i as GLuint );
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
			&BufferType::Vector3f32(_) 	=> { mem::size_of::<f32>() * 3 }
			&BufferType::Vector3f64(_) 	=> { mem::size_of::<f64>() * 3 }
			&BufferType::Colorf32(_) 	=> { mem::size_of::<f32>() * 3 }
			&BufferType::Colorf64(_) 	=> { mem::size_of::<f64>() * 3 }
		}
	}
}


impl GLGeometry for BufferGeometry {

	fn bind(&self, hash_map: &mut VartexArraysIDs) {
		match hash_map.get_mut(&self.uuid) {
			None => {},
			Some(ref buffers) => {
				gl_call!({ gl::BindVertexArray(buffers.vertex_array); });
				return;
			}
		}

		let buffers = Self::alloc_gl_gom(self);
		hash_map.insert(self.uuid, buffers);

		self.bind(hash_map);
	}

	fn un_bind(&self){
		gl_call!({ gl::BindVertexArray(0); });
	}
}
