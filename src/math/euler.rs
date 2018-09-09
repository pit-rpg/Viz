use helpers::Nums;
use math::{Matrix4, Quaternion};

#[allow(dead_code)]
#[derive(Clone, Debug, Copy)]
pub enum RotationOrders {
	XYZ,
	YZX,
	ZXY,
	XZY,
	YXZ,
	ZYX,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Euler<T>
where
	T: Nums,
{
	pub x: T,
	pub y: T,
	pub z: T,
	pub rotation_order: RotationOrders,
}

#[allow(dead_code)]
impl<T> Euler<T>
where
	T: Nums,
{
	pub fn new(x: T, y: T, z: T, rotation_order: RotationOrders) -> Self {
		Self {
			x,
			y,
			z,
			rotation_order,
		}
	}

	pub fn set_from_rotation_matrix(&mut self, m: &Matrix4<T>) -> &mut Self {
		// assumes the upper 3x3 of m is a pure rotation matrix (i.e, unscaled)

		let te = m.elements;

		let m11 = te[0];
		let m12 = te[4];
		let m13 = te[8];
		let m21 = te[1];
		let m22 = te[5];
		let m23 = te[9];
		let m31 = te[2];
		let m32 = te[6];
		let m33 = te[10];

		{
			let one = T::one();
			let zero = T::zero();

			match self.rotation_order {
				RotationOrders::XYZ => {
					self.y = m13.clamp(-one, one).sin();
					if m13.abs() < T::from_f32(0.99999) {
						self.x = T::atan2(-m23, m33);
						self.z = T::atan2(-m12, m11);
					} else {
						self.x = T::atan2(m32, m22);
						self.z = zero;
					}
				}

				RotationOrders::YXZ => {
					self.x = (-(m23.clamp(-one, one))).asin();
					if m23.abs() < T::from_f32(0.99999) {
						self.y = T::atan2(m13, m33);
						self.z = T::atan2(m21, m22);
					} else {
						self.y = T::atan2(-m31, m11);
						self.z = zero;
					}
				}

				RotationOrders::ZXY => {
					self.x = m32.clamp(-one, one).asin();
					if m32.abs() < T::from_f32(0.99999) {
						self.y = T::atan2(-m31, m33);
						self.z = T::atan2(-m12, m22);
					} else {
						self.y = zero;
						self.z = T::atan2(m21, m11);
					}
				}

				RotationOrders::ZYX => {
					self.y = -(m31.clamp(-one, one)).asin();
					if m31.abs() < T::from_f32(0.99999) {
						self.x = T::atan2(m32, m33);
						self.z = T::atan2(m21, m11);
					} else {
						self.x = zero;
						self.z = T::atan2(-m12, m22);
					}
				}

				RotationOrders::YZX => {
					self.z = m21.clamp(-one, one).asin();
					if m21.abs() < T::from_f32(0.99999) {
						self.x = T::atan2(-m23, m22);
						self.y = T::atan2(-m31, m11);
					} else {
						self.x = zero;
						self.y = T::atan2(m13, m33);
					}
				}

				RotationOrders::XZY => {
					self.z = (-(m12.clamp(-one, one))).asin();
					if m12.abs() < T::from_f32(0.99999) {
						self.x = T::atan2(m32, m22);
						self.y = T::atan2(m13, m11);
					} else {
						self.x = T::atan2(-m23, m33);
						self.y = zero;
					}
				}
			}
		}
		self
	}

	pub fn set_from_quaternion(&mut self, q: &Quaternion<T>) -> &mut Self {
		let mut matrix = Matrix4::new();
		matrix.make_rotation_from_quaternion(q);
		self.set_from_rotation_matrix(&matrix)
	}
}

impl<T> Default for Euler<T>
where
	T: Nums,
{
	fn default() -> Self {
		Self {
			x: T::zero(),
			y: T::zero(),
			z: T::zero(),
			rotation_order: RotationOrders::XYZ,
		}
	}
}
