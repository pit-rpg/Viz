use helpers::Nums;
use super::{Vector3, Vector, Matrix4, RotationOrders, Euler};

#[derive(Clone, Debug)]
pub struct Quaternion<T> {
	pub x: T,
	pub y: T,
	pub z: T,
	pub w: T,
}


impl <T> Quaternion<T>
where T: Nums
{
	pub fn new() -> Self {
		Self{ x: Nums::zero(), y: Nums::zero(), z: Nums::zero(), w: Nums::one() }
	}

	pub fn equals(&self, quaternion: &Self ) -> bool {
		return ( quaternion.x == self.x ) && ( quaternion.y == self.y ) && ( quaternion.z == self.z ) && ( quaternion.w == self.w );
	}

	pub fn copy(&mut self, q: &Self ) -> &mut Self {

		self.x = q.x;
		self.y = q.y;
		self.z = q.z;
		self.w = q.w;

		return self;
	}

	pub fn slerp (&mut self, qb: &Quaternion<T>, t: T) -> &mut Quaternion<T> {

		if t == T::zero()	{return self};
		if t == T::one()	{return self.copy( qb )};

		let x = self.x;
		let y = self.y;
		let z = self.z;
		let w = self.w;

		// http://www.euclideanspace.com/maths/algebra/realNormedAlgebra/quaternions/slerp/

		let mut cos_half_theta = w * qb.w + x * qb.x + y * qb.y + z * qb.z;

		if  cos_half_theta < T::zero()  {
			self.w = - qb.w;
			self.x = - qb.x;
			self.y = - qb.y;
			self.z = - qb.z;
			cos_half_theta = - cos_half_theta;
		} else {
			self.copy( qb );
		}

		if cos_half_theta >= T::one() {
			self.w = w;
			self.x = x;
			self.y = y;
			self.z = z;
			return self;
		}

		let sqr_sin_half_theta = T::one() - cos_half_theta * cos_half_theta;

		if sqr_sin_half_theta <= T::EPSILON() {
			let s = T::one() - t;
			self.w = s * w + t * self.w;
			self.x = s * x + t * self.x;
			self.y = s * y + t * self.y;
			self.z = s * z + t * self.z;

			return self.normalize();
		}

		let sin_half_theta = sqr_sin_half_theta.sqrt();
		let half_theta = T::atan2(sin_half_theta,  cos_half_theta );

		let ratio_a = ( ( T::one() - t ) * half_theta ).sin() / sin_half_theta;
		let ratio_b = ( t * half_theta ).sin() / sin_half_theta;

		self.w = w * ratio_a + self.w * ratio_b;
		self.x = x * ratio_a + self.x * ratio_b;
		self.y = y * ratio_a + self.y * ratio_b;
		self.z = z * ratio_a + self.z * ratio_b;

		return self;
	}

	pub fn normalize(&mut self) -> &mut Self {

		let mut l = self.length();

		if l == T::zero() {

			self.x = T::zero();
			self.y = T::zero();
			self.z = T::zero();
			self.w = T::one();

		} else {
			l = T::one() / l;

			self.x = self.x * l;
			self.y = self.y * l;
			self.z = self.z * l;
			self.w = self.w * l;

		}
		self
	}

	pub fn length_sq(&self) -> T {
		self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
	}

	pub fn length(&self) -> T {
		( self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w ).sqrt()
	}

	pub fn set_from_axis_angle(&mut self, axis: &Vector3<T> , angle: T ) -> &mut Self {
		// http://www.euclideanspace.com/maths/geometry/rotations/conversions/angleToQuaternion/index.htm

		// assumes axis is normalized
		let halfAngle = angle / T::two();
		let s = halfAngle.sin();

		self.x = axis.x * s;
		self.y = axis.y * s;
		self.z = axis.z * s;
		self.w = halfAngle.cos();
		self
	}

	pub fn multiply_quaternions(&mut self, a: &Self, b: &Self ) -> &mut Self{
		// from http://www.euclideanspace.com/maths/algebra/realNormedAlgebra/quaternions/code/index.htm

		let qax = a.x; let qay = a.y; let qaz = a.z; let qaw = a.w;
		let qbx = b.x; let qby = b.y; let qbz = b.z; let qbw = b.w;

		self.x = qax * qbw + qaw * qbx + qay * qbz - qaz * qby;
		self.y = qay * qbw + qaw * qby + qaz * qbx - qax * qbz;
		self.z = qaz * qbw + qaw * qbz + qax * qby - qay * qbx;
		self.w = qaw * qbw - qax * qbx - qay * qby - qaz * qbz;
		self
	}

	pub fn multiply (&mut self, q: &Self ) -> &mut Self {
		let c = self.clone();
		self.multiply_quaternions(&c, q)
	}


	pub fn premultiply(&mut self, q: &Self ) -> &mut Self {
		let c = self.clone();
		self.multiply_quaternions( q, &c )
	}


	pub fn inverse(&mut self) -> &mut Self {
		// quaternion is assumed to have unit length
		return self.conjugate()
	}

	pub fn conjugate(&mut self) -> &mut Self {
		self.x *= - T::one();
		self.y *= - T::one();
		self.z *= - T::one();
		self
	}

	pub fn dot (&self, v: &Self ) -> T {
		self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
	}

	pub fn angle_to(&self, q: &Self ) -> T {
		T::two() * self.dot( q ).clamp(- T::one(), T::one()).abs().acos()
	}

	pub fn set_from_rotation_matrix(&mut self, m: Matrix4<T> ) -> &mut Self {
		// http://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/index.htm
		// assumes the upper 3x3 of m is a pure rotation matrix (i.e, unscaled)

		let te = m.elements;

		let m11 = te[ 0 ]; let m12 = te[ 4 ]; let m13 = te[ 8 ];
		let m21 = te[ 1 ]; let m22 = te[ 5 ]; let m23 = te[ 9 ];
		let m31 = te[ 2 ]; let m32 = te[ 6 ]; let m33 = te[ 10 ];

		let trace = m11 + m22 + m33;
		let s;

		if trace > T::zero() {
			s = T::from_f32(0.5) / ( trace + T::one() ).sqrt();
			self.w = T::from_f32(0.25) / s;
			self.x = ( m32 - m23 ) * s;
			self.y = ( m13 - m31 ) * s;
			self.z = ( m21 - m12 ) * s;
		} else if m11 > m22 && m11 > m33 {
			s = T::two() * ( T::one() + m11 - m22 - m33 ).sqrt();
			self.w = ( m32 - m23 ) / s;
			self.x = T::from_f32(0.25) * s;
			self.y = ( m12 + m21 ) / s;
			self.z = ( m13 + m31 ) / s;
		} else if m22 > m33 {
			s = T::two() * ( T::one() + m22 - m11 - m33 ).sqrt();
			self.w = ( m13 - m31 ) / s;
			self.x = ( m12 + m21 ) / s;
			self.y = T::from_f32(0.25) * s;
			self.z = ( m23 + m32 ) / s;
		} else {
			s = T::two() * ( T::one() + m33 - m11 - m22 ).sqrt();
			self.w = ( m21 - m12 ) / s;
			self.x = ( m13 + m31 ) / s;
			self.y = ( m23 + m32 ) / s;
			self.z = T::from_f32(0.25) * s;
		}

		self
	}


	pub fn set_from_euler(&mut self, euler: &Euler<T> ) -> &mut Self {

		// let {x,y,z,rotation_order} = euler;
		let x = euler.x; let y = euler.y; let z = euler.z; let rotation_order = euler.rotation_order;


		// http://www.mathworks.com/matlabcentral/fileexchange/
		// 	20696-function-to-convert-between-dcm-euler-angles-quaternions-and-euler-vectors/
		//	content/SpinCalc.m

		let c1 = ( x / T::two() ).cos();
		let c2 = ( y / T::two() ).cos();
		let c3 = ( z / T::two() ).cos();

		let s1 = ( x / T::two() ).sin();
		let s2 = ( y / T::two() ).sin();
		let s3 = ( z / T::two() ).sin();

		match rotation_order {
			RotationOrders::XYZ => {
				self.x = s1 * c2 * c3 + c1 * s2 * s3;
				self.y = c1 * s2 * c3 - s1 * c2 * s3;
				self.z = c1 * c2 * s3 + s1 * s2 * c3;
				self.w = c1 * c2 * c3 - s1 * s2 * s3;
			}
			RotationOrders::YXZ => {
				self.x = s1 * c2 * c3 + c1 * s2 * s3;
				self.y = c1 * s2 * c3 - s1 * c2 * s3;
				self.z = c1 * c2 * s3 - s1 * s2 * c3;
				self.w = c1 * c2 * c3 + s1 * s2 * s3;
			}
			RotationOrders::ZXY => {
				self.x = s1 * c2 * c3 - c1 * s2 * s3;
				self.y = c1 * s2 * c3 + s1 * c2 * s3;
				self.z = c1 * c2 * s3 + s1 * s2 * c3;
				self.w = c1 * c2 * c3 - s1 * s2 * s3;
			}
			RotationOrders::ZYX => {
				self.x = s1 * c2 * c3 - c1 * s2 * s3;
				self.y = c1 * s2 * c3 + s1 * c2 * s3;
				self.z = c1 * c2 * s3 - s1 * s2 * c3;
				self.w = c1 * c2 * c3 + s1 * s2 * s3;
			}
			RotationOrders::YZX => {
				self.x = s1 * c2 * c3 + c1 * s2 * s3;
				self.y = c1 * s2 * c3 + s1 * c2 * s3;
				self.z = c1 * c2 * s3 - s1 * s2 * c3;
				self.w = c1 * c2 * c3 - s1 * s2 * s3;
			}
			RotationOrders::XZY => {
				self.x = s1 * c2 * c3 - c1 * s2 * s3;
				self.y = c1 * s2 * c3 - s1 * c2 * s3;
				self.z = c1 * c2 * s3 + s1 * s2 * c3;
				self.w = c1 * c2 * c3 + s1 * s2 * s3;
			}
		}
		self
	}



}



// import { _Math } from './Math.js';
// import { Vector3 } from './Vector3.js';

// /**
//  * @author mikael emtinger / http://gomo.se/
//  * @author alteredq / http://alteredqualia.com/
//  * @author WestLangley / http://github.com/WestLangley
//  * @author bhouston / http://clara.io
//  */

// Object.assign( Quaternion, {

// 	slerp: function ( qa, qb, qm, t ) {

// 		return qm.copy( qa ).slerp( qb, t );

// 	},

// 	slerpFlat: function ( dst, dstOffset, src0, srcOffset0, src1, srcOffset1, t ) {

// 		// fuzz-free, array-based Quaternion SLERP operation

// 		var x0 = src0[ srcOffset0 + 0 ],
// 			y0 = src0[ srcOffset0 + 1 ],
// 			z0 = src0[ srcOffset0 + 2 ],
// 			w0 = src0[ srcOffset0 + 3 ],

// 			x1 = src1[ srcOffset1 + 0 ],
// 			y1 = src1[ srcOffset1 + 1 ],
// 			z1 = src1[ srcOffset1 + 2 ],
// 			w1 = src1[ srcOffset1 + 3 ];

// 		if ( w0 !== w1 || x0 !== x1 || y0 !== y1 || z0 !== z1 ) {

// 			var s = 1 - t,

// 				cos = x0 * x1 + y0 * y1 + z0 * z1 + w0 * w1,

// 				dir = ( cos >= 0 ? 1 : - 1 ),
// 				sqrSin = 1 - cos * cos;

// 			// Skip the Slerp for tiny steps to avoid numeric problems:
// 			if ( sqrSin > Number.EPSILON ) {

// 				var sin = Math.sqrt( sqrSin ),
// 					len = Math.atan2( sin, cos * dir );

// 				s = Math.sin( s * len ) / sin;
// 				t = Math.sin( t * len ) / sin;

// 			}

// 			var tDir = t * dir;

// 			x0 = x0 * s + x1 * tDir;
// 			y0 = y0 * s + y1 * tDir;
// 			z0 = z0 * s + z1 * tDir;
// 			w0 = w0 * s + w1 * tDir;

// 			// Normalize in case we just did a lerp:
// 			if ( s === 1 - t ) {

// 				var f = 1 / Math.sqrt( x0 * x0 + y0 * y0 + z0 * z0 + w0 * w0 );

// 				x0 *= f;
// 				y0 *= f;
// 				z0 *= f;
// 				w0 *= f;

// 			}

// 		}

// 		dst[ dstOffset ] = x0;
// 		dst[ dstOffset + 1 ] = y0;
// 		dst[ dstOffset + 2 ] = z0;
// 		dst[ dstOffset + 3 ] = w0;

// 	}

// } );





// 	setFromUnitVectors: function () {

// 		// assumes direction vectors vFrom and vTo are normalized

// 		var v1 = new Vector3();
// 		var r;

// 		var EPS = 0.000001;

// 		return function setFromUnitVectors( vFrom, vTo ) {

// 			if ( v1 === undefined ) v1 = new Vector3();

// 			r = vFrom.dot( vTo ) + 1;

// 			if ( r < EPS ) {

// 				r = 0;

// 				if ( Math.abs( vFrom.x ) > Math.abs( vFrom.z ) ) {

// 					v1.set( - vFrom.y, vFrom.x, 0 );

// 				} else {

// 					v1.set( 0, - vFrom.z, vFrom.y );

// 				}

// 			} else {

// 				v1.crossVectors( vFrom, vTo );

// 			}

// 			this._x = v1.x;
// 			this._y = v1.y;
// 			this._z = v1.z;
// 			this._w = r;

// 			return this.normalize();

// 		};

// 	}(),



// 	rotateTowards: function ( q, step ) {

// 		var angle = this.angleTo( q );

// 		if ( angle === 0 ) return this;

// 		var t = Math.min( 1, step / angle );

// 		this.slerp( q, t );

// 		return this;

// 	},










// 	fromArray: function ( array, offset ) {

// 		if ( offset === undefined ) offset = 0;

// 		this._x = array[ offset ];
// 		this._y = array[ offset + 1 ];
// 		this._z = array[ offset + 2 ];
// 		this._w = array[ offset + 3 ];

// 		this.onChangeCallback();

// 		return this;

// 	},

// 	toArray: function ( array, offset ) {

// 		if ( array === undefined ) array = [];
// 		if ( offset === undefined ) offset = 0;

// 		array[ offset ] = this._x;
// 		array[ offset + 1 ] = this._y;
// 		array[ offset + 2 ] = this._z;
// 		array[ offset + 3 ] = this._w;

// 		return array;

// 	},

// 	onChange: function ( callback ) {

// 		this.onChangeCallback = callback;

// 		return this;

// 	},

// 	onChangeCallback: function () {}

// } );


// export { Quaternion };