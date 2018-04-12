// use std::convert::From;


use std::ops::{Div,AddAssign,SubAssign,MulAssign, Mul, Add, DivAssign, Sub, Neg};


pub fn concatenate_arrays<T: Clone>(x: &[T], y: &[T]) -> Vec<T> {
	let mut concat: Vec<T> = vec![x[0].clone(); x.len()];

	concat.clone_from_slice(x);
	concat.extend_from_slice(y);

	concat
}

fn f64_as_f32(n: f64) -> f32 {
	n as f32
}


pub trait Nums
where Self:Copy+MulAssign+AddAssign+SubAssign+Mul<Output=Self>+Add<Output=Self>+DivAssign+Sub<Output=Self>+Neg<Output=Self>+Clone+Div<Output=Self>+PartialOrd
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
	// fn into<T>(&self) -> T;
}


impl Nums for f32 {
	fn zero() -> Self { 0.0f32 }
	fn one() -> Self { 1.0 }
	fn sqrt(&self) -> Self { self.sqrt() }
	fn abs(&self) -> Self { self.abs() }
	fn round(&self) -> Self { self.round() }
	fn floor(&self) -> Self { self.floor() }
	fn ceil(&self) -> Self { self.ceil() }
	fn cos(&self) -> Self { self.cos() }
	fn sin(&self) -> Self { self.sin() }
	fn min(&self, other:Self) -> Self { self.min(other) }
	fn max(&self, other:Self) -> Self { self.max(other) }
	// fn into<f64>(&self) -> f64 {  let a = f64::from(*self); 1.0f64 }
}

impl Nums for f64 {
	fn zero() -> Self { 0.0 }
	fn one() -> Self { 1.0 }
	fn sqrt(&self) -> Self { self.sqrt() }
	fn abs(&self) -> Self { self.abs() }
	fn round(&self) -> Self { self.round() }
	fn floor(&self) -> Self { self.floor() }
	fn ceil(&self) -> Self { self.ceil() }
	fn cos(&self) -> Self { self.cos() }
	fn sin(&self) -> Self { self.sin() }
	fn min(&self, other:Self) -> Self { self.min(other) }
	fn max(&self, other:Self) -> Self { self.max(other) }
	// fn into<f32>(&self) -> f32 { f64_as_f32(*self) }
}
