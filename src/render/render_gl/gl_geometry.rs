use core::BufferGeometry;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
extern crate uuid;
use self::uuid::Uuid;
// let mut book_reviews = HashMap::new();

pub type VartexArrays<'a> = HashMap<&'a Uuid, usize>;

pub static mut VARTEX_ARRAYS:  Option<Mutex<VartexArrays>> = None;
// pub static mut VARTEX_ARRAYS: Option<HashMap<i32, i32>> = None;

#[allow(dead_code)]
pub trait GLGeometry {
	fn bind(&self, hash_map: &mut HashMap<&Uuid, usize>);
	fn un_bind(&self);
	fn alloc_gl_gom(&self, hash_map: &mut VartexArrays) -> &usize;
}



#[allow(dead_code)]
pub fn init() {
	unsafe {
		match VARTEX_ARRAYS {
			None => { VARTEX_ARRAYS = Some(Mutex::new(HashMap::new())); }
			Some(_) =>{}
		}
	}
}

impl GLGeometry for BufferGeometry {

	fn alloc_gl_gom(&self, hash_map: &mut VartexArrays) -> &usize {
		&1
	}

	fn bind(&self, hash_map: &mut VartexArrays) {
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