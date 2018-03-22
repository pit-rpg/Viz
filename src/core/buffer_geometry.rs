// use std::collections::HashMap;
extern crate uuid;
use self::uuid::Uuid;
use std::vec::Vec;
// use std::vec::*;
// use std::ops::IndexMut;

#[allow(dead_code)]
pub enum BufferType {
	F32(Box<[f32]>),
	F64(Box<[f64]>),
	Usize(Box<[usize]>),
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
			&BufferType::F64(ref a) => a.len(),
			&BufferType::Usize(ref a) => a.len(),
		}
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
	pub indices: Option<Box<[usize]>>,
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


	pub fn set_indices(&mut self, indices:Box<[usize]>) -> &mut Self {
		self.indices = Some(indices);
		self
	}


	pub fn create_buffer_attribute(&mut self, name:String, data:Box<[f32]>, item_size:usize ) -> &mut BufferAttribute {
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