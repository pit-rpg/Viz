// use std::collections::HashMap;
extern crate uuid;
use self::uuid::Uuid;
use std::vec::Vec;
use std::mem;
// use std::vec::*;
// use std::ops::IndexMut;

#[allow(dead_code)]
pub enum BufferType {
	F32(Vec<f32>),
	// F64(Vec<f64>),
	Usize(Vec<usize>),
}


#[allow(dead_code)]
pub struct  BufferAttribute {
	pub data: BufferType,
	pub name: String,
	pub item_size: usize,
	pub dynamic: bool,
	pub normalized: bool,
	pub version: usize,
}


#[allow(dead_code)]
impl BufferAttribute {
	pub fn count(&self) -> usize {
		let l = self.len();
		l / self.item_size
	}

	pub fn len(&self) -> usize {
		// let data = &self.data;
		match &self.data {
			&BufferType::F32(ref a) => a.len(),
			&BufferType::Usize(ref a) => a.len(),
			// &BufferType::F64(ref a) => a.len(),
		}
	}

	pub fn elem_byte_len(&self) -> usize {
		match self.data {
			BufferType::F32(_) => {mem::size_of::<f32>()}
			BufferType::Usize(_) => {mem::size_of::<usize>()}
		}
	}

	pub fn byte_len(&self) -> usize {
		self.elem_byte_len() * self.len()
	}

	pub fn set_normalized(&mut self, normalized:bool) -> &mut Self {
		self.normalized = normalized;
		self
	}

	pub fn set_version(&mut self, version:usize) -> &mut Self {
		self.version = version;
		self
	}

	pub fn set_dynamic(&mut self, dynamic:bool) -> &mut Self {
		self.dynamic = dynamic;
		self
	}
}


// gl::BufferData(
// 				gl::ARRAY_BUFFER,
// 				(mem::size_of::<GLfloat>() * positions.len()) as GLsizeiptr,
// 				&positions[0] as *const f32 as *const c_void,
// 				gl::DYNAMIC_DRAW
// 			);





#[allow(dead_code)]
pub struct BufferGroup {
	pub start: u32,
	pub material_index: u32,
	pub count: u32,
}


#[allow(dead_code)]
pub struct BufferGeometry {
	pub attributes: Vec<BufferAttribute>,
	pub groups: Vec<BufferGroup>,
	pub indices: Option<Vec<usize>>,
	pub uuid: Uuid,
}

impl BufferGeometry {
	pub fn new() -> Self{
		Self {
			attributes: Vec::new(),
			groups: Vec::new(),
			indices: None,
			uuid: Uuid::new_v4(),
		}
	}


	pub fn set_indices(&mut self, indices:Vec<usize>) -> &mut Self {
		self.indices = Some(indices);
		self
	}


	pub fn create_buffer_attribute(&mut self, name:String, data:Vec<f32>, item_size:usize ) -> &mut BufferAttribute {
		let data = BufferType::F32(data);

		let bufferattribute = BufferAttribute{
			name, data, item_size,
			normalized: false,
			dynamic: false,
			version: 0,
		};

		self.attributes.push(bufferattribute);

		let i = self.attributes.len() - 1;
		&mut self.attributes[i]
		// a
		// &mut self.attributes[self.attributes.len()-1]
	}
}