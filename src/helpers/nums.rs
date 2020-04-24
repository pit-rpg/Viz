extern crate rand;
use std::ops::{Div,AddAssign,SubAssign,MulAssign, Mul, Add, DivAssign, Sub, Neg};
use std::marker::{Sync, Send};
use std::fmt::Debug;
// use self::rand::R;

pub trait Nums
where Self:
Copy+
Clone+
Sync+
Send+
MulAssign+
AddAssign+
SubAssign+
DivAssign+
PartialOrd+
Debug+
PartialEq+
Mul<Output=Self>+
Add<Output=Self>+
Sub<Output=Self>+
Neg<Output=Self>+
Div<Output=Self>
{
	fn zero() -> Self;
	fn one() -> Self;
	fn two() -> Self;
	fn random() -> Self;
	fn sqrt(&self) -> Self;
	fn abs(&self) -> Self;
	fn round(&self) -> Self;
	fn floor(&self) -> Self;
	fn ceil(&self) -> Self;
	fn cos(&self) -> Self;
	fn sin(&self) -> Self;
	fn acos(&self) -> Self;
	fn asin(&self) -> Self;
	fn min(&self, other:Self) -> Self;
	fn max(&self, other:Self) -> Self;
	fn max_val() -> Self;
	fn min_val() -> Self;
	fn atan2(y:Self, x:Self) -> Self;
	fn epsilon() -> Self;
	fn clamp(&self, min: Self, max: Self) -> Self;
	fn from_f32(n: f32) -> Self;
	fn from_f64(n: f64) -> Self;
	fn as_u8(&self) -> u8;
}


impl Nums for f32 {
	#[inline] fn zero() -> Self { 0.0f32 }
	#[inline] fn one() -> Self { 1.0 }
	#[inline] fn two() -> Self { 2.0 }
	#[inline] fn random() -> Self { rand::random::<f32>() }
	#[inline] fn sqrt(&self) -> Self { f32::sqrt(*self) }
	#[inline] fn abs(&self) -> Self { f32::abs(*self) }
	#[inline] fn round(&self) -> Self { f32::round(*self) }
	#[inline] fn floor(&self) -> Self { f32::floor(*self) }
	#[inline] fn ceil(&self) -> Self { f32::ceil(*self) }
	#[inline] fn cos(&self) -> Self { f32::cos(*self) }
	#[inline] fn sin(&self) -> Self { f32::sin(*self) }
	#[inline] fn acos(&self) -> Self { f32::acos(*self) }
	#[inline] fn asin(&self) -> Self { f32::asin(*self) }
	#[inline] fn min(&self, other:Self) -> Self { f32::min(*self, other) }
	#[inline] fn max(&self, other:Self) -> Self { f32::max(*self, other) }
	#[inline] fn max_val() -> Self { std::f32::MAX }
	#[inline] fn min_val() -> Self { std::f32::MIN }
	#[inline] fn atan2(y:Self, x:Self) -> Self { f32::atan2(y, x) }
	#[inline] fn epsilon() -> Self { use std::f32::EPSILON; EPSILON }
	#[inline] fn clamp(&self, min: Self, max: Self) -> Self { self.min(max).max(min) }
	#[inline] fn from_f32(n: f32) -> Self { n }
	#[inline] fn from_f64(n: f64) -> Self { n as f32 }
	fn as_u8(&self) -> u8 { (self * 255.0).max(0.0).min(255.0) as u8 }
}

impl Nums for f64 {
	#[inline] fn zero() -> Self { 0.0 }
	#[inline] fn one() -> Self { 1.0 }
	#[inline] fn two() -> Self { 2.0 }
	#[inline] fn random() -> Self { rand::random::<f64>() }
	#[inline] fn sqrt(&self) -> Self { f64::sqrt(*self) }
	#[inline] fn abs(&self) -> Self { f64::abs(*self) }
	#[inline] fn round(&self) -> Self { f64::round(*self) }
	#[inline] fn floor(&self) -> Self { f64::floor(*self) }
	#[inline] fn ceil(&self) -> Self { f64::ceil(*self) }
	#[inline] fn cos(&self) -> Self { f64::cos(*self) }
	#[inline] fn sin(&self) -> Self { f64::sin(*self) }
	#[inline] fn acos(&self) -> Self { f64::acos(*self) }
	#[inline] fn asin(&self) -> Self { f64::asin(*self) }
	#[inline] fn min(&self, other:Self) -> Self { f64::min(*self, other) }
	#[inline] fn max(&self, other:Self) -> Self { f64::max(*self, other) }
	#[inline] fn max_val() -> Self { std::f64::MAX }
	#[inline] fn min_val() -> Self { std::f64::MIN }
	#[inline] fn atan2(y:Self, x:Self) -> Self { f64::atan2(y, x) }
	#[inline] fn epsilon() -> Self { use std::f64::EPSILON; EPSILON }
	#[inline] fn clamp(&self, min: Self, max: Self) -> Self { self.min(max).max(min) }
	#[inline] fn from_f32(n: f32) -> Self { n as f64 }
	#[inline] fn from_f64(n: f64) -> Self { n }
	fn as_u8(&self) -> u8 { (self * 255.0).max(0.0).min(255.0) as u8 }
}
