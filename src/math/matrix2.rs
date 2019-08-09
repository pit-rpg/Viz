use helpers::Nums;

use std::ops::{
	Index,
	IndexMut,
};

#[repr(C)]
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix2<T> {
	pub elements: [T; 4],
}

#[allow(dead_code)]
impl<T> Matrix2<T>
where
	T: Nums,
{
	pub fn new() -> Self {
		Self {
			elements: [
				Nums::one(),
				Nums::zero(),
				Nums::zero(),
				Nums::one(),
			],
		}
	}

	pub fn set( &mut self, n1: T, n2: T, n3: T, n4: T ) -> &mut Self {
		{
			let te = &mut self.elements;
			te[0] = n1;
			te[1] = n2;
			te[2] = n3;
			te[3] = n4;
		}
		self
	}


	pub fn identity(&mut self) -> &mut Self {
		self.set(
			Nums::one(),
			Nums::zero(),
			Nums::zero(),
			Nums::one(),
		);
		self
	}


	pub fn copy( &mut self, m: &Self ) -> &mut Self {
		let me = m.elements;
		{
			let te = &mut self.elements;

			te[ 0 ] = me[ 0 ]; te[ 1 ] = me[ 1 ];
			te[ 2 ] = me[ 2 ]; te[ 3 ] = me[ 3 ];
		}
		self
	}


	pub fn equals (&self, matrix: &Matrix2<T> ) -> bool {
		let te = & self.elements;
		let me = matrix.elements;
		for i in 0..9  {
			if te[ i ] != me[ i ] {return false};
		}

		true
	}

	pub fn from_array(data: &[T]) -> Self {
		Self {
			elements: [
				data[0],
				data[1],

				data[2],
				data[3],
			]
		}
	}
}


impl <T> Index<usize> for Matrix2<T>
where T: Nums
{
    type Output = T;

    fn index(&self, c: usize) -> &T {
        &self.elements[c]
    }
}

impl <T> IndexMut<usize> for Matrix2<T>
where T: Nums
{
    // type Output = T;

    fn index_mut(&mut self, c: usize) -> &mut T {
        &mut self.elements[c]
    }
}