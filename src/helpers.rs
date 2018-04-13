// use std::convert::From;

extern crate rand;
use std::ops::{Div,AddAssign,SubAssign,MulAssign, Mul, Add, DivAssign, Sub, Neg};


// pub fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
// 	let mut concat: Vec<T> = vec![x[0].clone(); x.len()];

// 	concat.clone_from_slice(x);
// 	concat.extend_from_slice(y);

// 	concat
// }

// fn f64_as_f32(n: f64) -> f32 {
// 	n as f32
// }


pub trait Nums
where Self:Copy+MulAssign+AddAssign+SubAssign+Mul<Output=Self>+Add<Output=Self>+DivAssign+Sub<Output=Self>+Neg<Output=Self>+Clone+Div<Output=Self>+PartialOrd+rand::Rand
{
	// type T = Self;
	fn zero() -> Self;
	fn one() -> Self;
	fn sqrt(&self) -> Self;
	fn abs(&self) -> Self;
	fn round(&self) -> Self;
	fn floor(&self) -> Self;
	fn ceil(&self) -> Self;
	fn cos(&self) -> Self;
	fn sin(&self) -> Self;
	fn min(&self, other:Self) -> Self;
	fn max(&self, other:Self) -> Self;
}


impl Nums for f32 {
	fn zero() -> Self { 0.0f32 }
	fn one() -> Self { 1.0 }
	fn sqrt(&self) -> Self { f32::sqrt(*self) }
	fn abs(&self) -> Self { f32::abs(*self) }
	fn round(&self) -> Self { f32::round(*self) }
	fn floor(&self) -> Self { f32::floor(*self) }
	fn ceil(&self) -> Self { f32::ceil(*self) }
	fn cos(&self) -> Self { f32::cos(*self) }
	fn sin(&self) -> Self { f32::sin(*self) }
	fn min(&self, other:Self) -> Self { f32::min(*self, other) }
	fn max(&self, other:Self) -> Self { f32::max(*self, other) }
}

impl Nums for f64 {
	fn zero() -> Self { 0.0 }
	fn one() -> Self { 1.0 }
	fn sqrt(&self) -> Self { f64::sqrt(*self) }
	fn abs(&self) -> Self { f64::abs(*self) }
	fn round(&self) -> Self { f64::round(*self) }
	fn floor(&self) -> Self { f64::floor(*self) }
	fn ceil(&self) -> Self { f64::ceil(*self) }
	fn cos(&self) -> Self { f64::cos(*self) }
	fn sin(&self) -> Self { f64::sin(*self) }
	fn min(&self, other:Self) -> Self { f64::min(*self, other) }
	fn max(&self, other:Self) -> Self { f64::max(*self, other) }
}
