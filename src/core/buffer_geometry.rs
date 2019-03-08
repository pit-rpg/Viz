extern crate uuid;
use self::uuid::Uuid;
use std::vec::Vec;
use std::fmt;
use std::sync::{Arc,Mutex, LockResult, MutexGuard};
use std::mem;
use std::error::Error;


use math::{
	Vector,
	Vector2,
	Vector3,
	Vector4,
	Matrix2,
	Matrix3,
	Matrix4,
};

use core::{
	BBox3,
};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum BufferData {
	Matrix2(Vec<Matrix2<f32>>),
	Matrix3(Vec<Matrix3<f32>>),
	Matrix4(Vec<Matrix4<f32>>),
	Vector2(Vec<Vector2<f32>>),
	Vector3(Vec<Vector3<f32>>),
	Vector4(Vec<Vector4<f32>>),
	F32(Vec<f32>),
	I32(Vec<i32>),
	U32(Vec<u32>),
	I16(Vec<i16>),
	U16(Vec<u16>),
	I8(Vec<i8>),
	U8(Vec<u8>),
}

impl BufferData {
	pub fn item_size(&self) -> usize {
		match self {
			BufferData::Matrix2(_) => 4,
			BufferData::Matrix3(_) => 9,
			BufferData::Matrix4(_) => 16,
			BufferData::Vector2(_) => 2,
			BufferData::Vector3(_) => 3,
			BufferData::Vector4(_) => 4,
			BufferData::F32(_) => 1,
			BufferData::I32(_) => 1,
			BufferData::U32(_) => 1,
			BufferData::I16(_) => 1,
			BufferData::U16(_) => 1,
			BufferData::I8(_) => 1,
			BufferData::U8(_) => 1,
		}
	}

	pub fn len(&self) -> usize {
		match self {
			BufferData::Matrix2(a) => a.len(),
			BufferData::Matrix3(a) => a.len(),
			BufferData::Matrix4(a) => a.len(),
			BufferData::Vector2(a) => a.len(),
			BufferData::Vector3(a) => a.len(),
			BufferData::Vector4(a) => a.len(),
			BufferData::F32(a) => a.len(),
			BufferData::I32(a) => a.len(),
			BufferData::U32(a) => a.len(),
			BufferData::I16(a) => a.len(),
			BufferData::U16(a) => a.len(),
			BufferData::I8(a) => a.len(),
			BufferData::U8(a) => a.len(),
		}
	}

	pub fn elem_byte_len(&self) -> usize {
		let bytes = match self {
			BufferData::Matrix2(_) => mem::size_of::<f32>(),
			BufferData::Matrix3(_) => mem::size_of::<f32>(),
			BufferData::Matrix4(_) => mem::size_of::<f32>(),
			BufferData::Vector2(_) => mem::size_of::<f32>(),
			BufferData::Vector3(_) => mem::size_of::<f32>(),
			BufferData::Vector4(_) => mem::size_of::<f32>(),
			BufferData::F32(_) => mem::size_of::<f32>(),
			BufferData::I32(_) => mem::size_of::<i32>(),
			BufferData::U32(_) => mem::size_of::<u32>(),
			BufferData::I16(_) => mem::size_of::<i16>(),
			BufferData::U16(_) => mem::size_of::<u16>(),
			BufferData::I8(_) => mem::size_of::<i8>(),
			BufferData::U8(_) => mem::size_of::<u8>(),
		};
		self.item_size() * bytes
	}
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BufferType {
	Position,
	Normal,
	UV,
	Color,
	Tangent,
	Joint,
	Weight,
	Other(String),
}


#[derive(Clone, Debug)]
pub struct BufferAttribute {
	pub data: BufferData,
	pub buffer_type: BufferType,
	pub dynamic: bool,
	pub normalized: bool,
	// pub version: usize,
}


impl BufferAttribute {
	pub fn count(&self) -> usize {
		let l = self.len();
		l / self.item_size()
	}

	pub fn item_size(&self) -> usize {
		self.data.item_size()
	}

	pub fn len(&self) -> usize {
		self.data.len()
	}

	pub fn set_normalized(&mut self, normalized: bool) -> &mut Self {
		self.normalized = normalized;
		self
	}

	pub fn set_dynamic(&mut self, dynamic: bool) -> &mut Self {
		self.dynamic = dynamic;
		self
	}
}

#[allow(dead_code)]
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct BufferGroup {
	pub start: usize,
	pub material_index: usize,
	pub count: usize,
	pub name: String,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct BufferGeometry {
	pub uuid: Uuid,
	pub name: String,
	pub groups: Vec<BufferGroup>,
	pub indices: Option<Vec<u32>>,
	pub attributes: Vec<BufferAttribute>,
	pub buffer_order: Vec<BufferType>,
	pub b_box: Option<BBox3<f32>>,
	callbacks: Vec<fn(&mut BufferGeometry)>,
}


impl fmt::Debug for BufferGeometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "=====================
BufferGeometry: {}
uuid: {}
groups: {:?}
b_box: {:?}
callbacks: {}
indices: {:?}
attributes: {:?}
=====================",
		self.name,
		self.uuid,
		self.groups,
		self.b_box,
		self.callbacks.len(),
		self.indices,
		self.attributes,
		)
    }
}




extern crate specs;
use self::specs::{Component, VecStorage};


#[allow(dead_code)]
impl BufferGeometry {
	pub fn new() -> Self {
		Self {
			attributes: Vec::new(),
			groups: Vec::new(),
			indices: None,
			uuid: Uuid::new_v4(),
			callbacks: Vec::new(),
			name: "".to_string(),
			b_box: None,
			buffer_order: vec![BufferType::Position, BufferType::Normal, BufferType::UV, BufferType::Joint, BufferType::Weight],
		}
	}

	pub fn iter_attributes<'a>(&'a self) -> impl Iterator<Item= &'a BufferAttribute> {
		self.buffer_order.iter()
			.map(move |e| self.get_attribute(e.clone()) )
			.filter(|e| e.is_some() )
			.map(|e| e.unwrap() )
	}

	// pub fn iter_attributes_mut<'a>(&'a mut self) -> impl Iterator<Item= &'a mut BufferAttribute> {
	// 	self.buffer_order.iter()
	// 		.map(move |e| self.get_attribute_mut(e.clone()) )
	// 		.filter(|e| e.is_some() )
	// 		.map(|e| e.unwrap() )
	// }

	pub fn set_indices(&mut self, indices: Vec<u32>) -> &mut Self {
		self.indices = Some(indices);
		self
	}

	pub fn gen_indices(&mut self) -> Result<(), &str> {
		let mut len = 0;

		match self.get_attribute(BufferType::Position) {
			None => {
				return Err("BufferGeometry: cant find position");
			}
			Some(positions) => {
				len = positions.len();
			}
		};

		let indices = (0..len as u32).collect();

		self.set_indices(indices);

		Ok(())
	}

	pub fn add_buffer_attribute(
		&mut self,
		buffer_attribute: BufferAttribute,
	) -> &mut BufferAttribute {
		let index = self.attributes.iter().position( |attr| attr.buffer_type == buffer_attribute.buffer_type);

		if let Some(index) = index {
			self.attributes.remove(index);
		}
		self.attributes.push(buffer_attribute);

		if !self.attributes.iter().all( |e| e.len() == self.attributes[0].len() ) {
			panic!("BufferGeometry: different buffer length: {}", self.name);
		}

		let i = self.attributes.len() - 1;
		&mut self.attributes[i]
	}

	pub fn create_buffer_attribute(
		&mut self,
		buffer_type: BufferType,
		data: BufferData,
	) -> &mut BufferAttribute {
		let buffer_attribute = BufferAttribute {
			buffer_type,
			data,
			normalized: false,
			dynamic: false,
			// version: 0,
		};

		self.add_buffer_attribute(buffer_attribute)
	}

	pub fn on_drop(&mut self, cb: fn(&mut BufferGeometry)) {
		self.callbacks.push(cb);
	}

	pub fn get_attribute(&self, buffer_type: BufferType) -> Option<&BufferAttribute> {
		self.attributes.iter().find(|e| e.buffer_type == buffer_type)
	}

	pub fn has_attribute(&self, buffer_type: BufferType) -> bool {
		self.attributes.iter().any(|e| e.buffer_type == buffer_type)
	}

	pub fn get_attribute_mut(&mut self, buffer_type: BufferType) -> Option<&mut BufferAttribute> {
		self.attributes.iter_mut().find(|e| e.buffer_type == buffer_type)
	}


	pub fn generate_normals(&mut self) {
		let mut normals = None;
		{
			let attribute = self.get_attribute(BufferType::Position).unwrap();
			if let BufferData::Vector3(data) = &attribute.data {
				let mut calc_normals = vec![Vec::new(); data.len()];
				let indices = self.indices.as_ref().unwrap();

				let il = indices.len();
				let mut i = 0;
				while i < il {
					let a = &data[ indices[i]   as usize];
					let b = &data[ indices[i+1] as usize];
					let c = &data[ indices[i+2] as usize];

					let mut cb = c - b;
					let ab = a - b;
					cb.cross(&ab);
					cb.normalize();

					calc_normals[ indices[i]   as usize ].push(cb.clone());
					calc_normals[ indices[i+1] as usize ].push(cb.clone());
					calc_normals[ indices[i+2] as usize ].push(cb);

					i+=3;
				}

				let calc_normals = calc_normals
					.iter()
					.map(|items|{
						if items.len() == 1 {
							return items[0].clone();
						}
						let mut res = Vector3::add_all_vectors(items);
						res.normalize();
						res
					})
					.collect();
				normals = Some(calc_normals);
			}
		}

		if let Some(normal) = normals {
			self.create_buffer_attribute(BufferType::Normal, BufferData::Vector3(normal));
		}
	}

	pub fn duplicate(&self) -> Self {
		let mut data = self.clone();
		data.uuid = Uuid::new_v4();
		data
	}

	pub fn update_box3 (&mut self) -> Result <(), Box<Error>> {
		let mut b_box = None;
		if let Some(attr) = self.get_attribute(BufferType::Position) {
			if let BufferData::Vector3(positions) = &attr.data {
				let mut b = BBox3::new_empty();
				b.set_from_array(&positions[..]);
				b_box = Some(b);
			}
		}
		if b_box.is_none() {return Err( Box::from("cant update b_box") ); }
		self.b_box = b_box;
		Ok(())
	}

	pub fn get_b_box(&mut self) -> Result<BBox3<f32>, Box<Error>> {
		if self.b_box.is_some() {
			return Ok(self.b_box.as_ref().unwrap().clone())
		}

		self.update_box3()?;
		Ok(self.b_box.as_ref().unwrap().clone())
	}

	pub fn scale_positions_by_vec(&mut self, v: &Vector3<f32>) -> Option<()> {
		if let Some(attr) = self.get_attribute_mut(BufferType::Position) {
			if let BufferData::Vector3(positions) = &mut attr.data {
				positions
					.iter_mut()
					.for_each(|e| {
						e.multiply(v);
					});
				return Some(());
			}
			return None;
		}
		None
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



#[derive(Clone)]
pub struct SharedGeometry (Arc<Mutex<BufferGeometry>>);

impl SharedGeometry {
	pub fn new(g: BufferGeometry) -> Self {
		SharedGeometry(Arc::new(Mutex::new(g)))
	}

	pub fn lock(&mut self) -> LockResult<MutexGuard<BufferGeometry>> {
		self.0.lock()
	}
}


impl Component for SharedGeometry {
	type Storage = VecStorage<Self>;
}
