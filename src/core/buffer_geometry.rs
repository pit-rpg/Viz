// use std::collections::HashMap;
extern crate uuid;
// extern crate byteorder;
use self::uuid::Uuid;
use std::vec::Vec;
use math::vector3::Vector3;
// use math::vector2::Vector2;
use math::Color;
// use helpers::Nums;
// use std::marker::PhantomData;
// use std::process;
// use std::mem;

#[allow(dead_code)]
pub enum BufferType
{
	Vector3f32(Vec<Vector3<f32>>),
	Vector3f64(Vec<Vector3<f64>>),
	Colorf32(Vec<Color<f32>>),
	Colorf64(Vec<Color<f64>>),
	// Vector2(Vec<Vector2<T>>),
	// F64(Vec<f64>),
	// Usize(Vec<usize>),
}


#[allow(dead_code)]
pub struct BufferAttribute
{
	pub data: BufferType,
	pub name: String,
	pub item_size: usize,
	pub dynamic: bool,
	pub normalized: bool,
	pub version: usize,
}


#[allow(dead_code)]
impl BufferAttribute
{

	pub fn count(&self) -> usize {
		let l = self.len();
		l / self.item_size
	}

	pub fn len(&self) -> usize {
		// let data = &self.data;
		match &self.data {
			&BufferType::Vector3f32(ref a) 	=> a.len(),
			&BufferType::Vector3f64(ref a) 	=> a.len(),
			&BufferType::Colorf32(ref a) 	=> a.len(),
			&BufferType::Colorf64(ref a) 	=> a.len(),
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
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct BufferGroup {
	pub start: u32,
	pub material_index: u32,
	pub count: u32,
}



#[allow(dead_code)]
pub struct BufferGeometry
{
	pub groups: Vec<BufferGroup>,
	pub indices: Option<Vec<i32>>,
	pub uuid: Uuid,
	pub attributes: Vec<BufferAttribute>,
	callbacks: Vec<fn(&mut BufferGeometry)>
}


#[allow(dead_code)]
impl BufferGeometry
{

	pub fn new() -> Self{
		Self {
			attributes: Vec::new(),
			groups: Vec::new(),
			indices: None,
			uuid: Uuid::new_v4(),
			callbacks: Vec::new(),
		}
	}


	pub fn set_indices(&mut self, indices:Vec<i32>) -> &mut Self {
		self.indices = Some(indices);
		self
	}


	pub fn add_buffer_attribute(&mut self, bufferattribute: BufferAttribute) -> &mut BufferAttribute  {
		if self.attributes.len() > 0 {
			let len = bufferattribute.len();
			let prev_len = self.attributes[0].len();
			if len != prev_len {
				panic!("BufferGeometry: diffrent buffer length {}:{}, {}:{}", bufferattribute.name, len, self.attributes[0].name, prev_len);
			}
		}

		self.attributes.push(bufferattribute);

		let i = self.attributes.len() - 1;
		&mut self.attributes[i]
	}


	pub fn create_buffer_attribute(&mut self, name:String, data: BufferType, item_size:usize ) -> &mut BufferAttribute {
		let bufferattribute = BufferAttribute{
			name, data, item_size,
			normalized: false,
			dynamic: false,
			version: 0,
		};

		self.add_buffer_attribute(bufferattribute)
	}


	pub fn on_drop(&mut self, cb: fn(&mut BufferGeometry) ) {
		self.callbacks.push(cb);
	}
}



impl Drop for BufferGeometry {
	fn drop(&mut self) {
		while self.callbacks.len() > 0 {
			let cb = self.callbacks.pop().unwrap();
			cb(self);
		}
	}
}