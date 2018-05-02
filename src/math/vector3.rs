// mod vector;
//
// use math::Vector;

// extern crate num_traits;
// use math::Matrix4;
// use math::Matrix3;
// use std::cmp::{ Eq};
// use std::ops::{Div,AddAssign,SubAssign,MulAssign, Mul, Add, DivAssign, Sub, Neg};
use std::ops::{Sub};
// use std::ops::{Div};
// use self::num_traits::Float;
use helpers::Nums;

// #[repr(C)]
#[derive(Clone, Debug)]
pub struct Vector3<T>
where T:Nums {
    pub x: T,
    pub y: T,
    pub z: T,
}
//
// #[derive(Clone, Debug)]
// pub struct Vector64 {
//     pub x: f64,
//     pub y: f64,
//     pub z: f64,
// }

// pub trait FloatDefaults<T> {
//     fn zero() -> T;
//     fn one() -> T;
// }
//
// impl FloatDefaults<f32> for f32 {
//     fn zero() -> f32 { 0.0 }
//     fn one() -> f32 { 1.0 }
// }
//
// impl FloatDefaults<f64> for f64 {
//     fn zero () -> f64 { 0.0 }
//     fn one () -> f64 { 1.0 }
// }



pub trait Vector<T>
where T: Nums {
    // fn clone(v: &Self) -> Self;
    fn new() -> Self;
    fn new_from(x: T,y: T,z: T) -> Self;
    fn multiply_scalar(&mut self, s: T) -> &mut Self;
    fn length(&self) -> T;
    fn length_sq(&self) -> T;
    fn manhattan_length(&mut self) -> T;
    fn set_scalar(&mut self, s: T) -> &mut Self;
    fn add_scalar(&mut self, s: T) -> &mut Self;
    fn sub_scalar(&mut self, s: T) -> &mut Self;
    fn add(&mut self, v: &Self) -> &mut Self;
    fn sub(&mut self, v: &Self) -> &mut Self;
    fn multiply(&mut self, v: &Self) -> &mut Self;
    fn divide(&mut self, v: &Self) -> &mut Self;
    fn add_vectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn multiply_vectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn negate(&mut self) -> &mut Self;
    fn min(&mut self, v: &Self) -> &mut Self;
    fn max(&mut self, v: &Self) -> &mut Self;
    fn dot(&mut self, v: &Self) -> T;
    fn round(&mut self) -> &mut Self;
    fn floor(&mut self) -> &mut Self;
    fn ceil(&mut self) -> &mut Self;
    fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self;
    fn lerp (&mut self, v: &Self,  alpha:T )-> &mut Self;
    fn zero () -> Self;
    fn cross_vectors ( &mut self, a: &Self, b: &Self ) -> &mut Self;
    fn cross (&mut self, v: &Self )-> &mut Self;
    fn set(&mut self, x: T, y: T, z: T) -> &mut Self;
    // fn set_from_matrix_column (&mut self, m: &Matrix4<T>, index: usize ) -> &mut Self;
    fn from_array (&mut self, array: &[T] ) -> &mut Self;
    // fn apply_matrix_4 (&mut self, m: &Matrix4<T> ) -> &mut Self;
    // fn apply_matrix_3 (&mut self, m: &Matrix3<T> ) -> &mut Self;

    fn divide_scalar(&mut self, s: T) -> &mut Self {
        return self.multiply_scalar(T::one() / s);
    }

    fn normalize(&mut self) -> &mut Self {
        let mut l = self.length();
        if l == T::zero() {
            l = T::one()
        };
        self.divide_scalar(l);
        self
    }

    fn set_length(&mut self, length: T) -> &mut Self {
        self.normalize().multiply_scalar(length)
    }

    fn clamp_length (&mut self, min:T, max:T )-> &mut Self {
        let mut l = self.length();
        if l == T::zero() {l = T::one()};
        self.divide_scalar( l ).multiply_scalar(min.min( max.max(l)))
        // self.divide_scalar( l ).multiply_scalar(min(min1, max(max1, l)))
    }

    fn lerp_vectors (&mut self, v1: &Self, v2: &Self, alpha:T )-> &mut Self {
        self.sub_vectors( v2, v1 ).multiply_scalar( alpha ).add( v1 )
    }
}

// #[macro_export]
// macro_rules! Vector3 {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }

impl <T> Vector<T> for Vector3<T>
where T:Nums
        {

            fn new() -> Self {
                Self { x: T::zero(), y: T::zero(), z: T::zero() }
            }

            fn new_from(x: T,y: T,z: T) -> Self {
                Self { x, y, z }
            }

            fn multiply_scalar(&mut self, s: T) -> &mut Self {
                self.x *= s;
                self.y *= s;
                self.z *= s;
                self
            }

            fn length(&self) -> T {
                (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
            }

            fn length_sq(&self) -> T {
                self.x * self.x + self.y * self.y + self.z * self.z
            }

            fn manhattan_length(&mut self) -> T {
                (self.x).abs() + (self.y).abs() + (self.z).abs()
            }

            fn set_scalar(&mut self, s: T) -> &mut Self {
                self.x = s;
                self.y = s;
                self.z = s;
                self
            }

            fn add_scalar(&mut self, s: T) -> &mut Self {
                self.x += s;
                self.y += s;
                self.z += s;
                self
            }

            fn sub_scalar(&mut self, s: T) -> &mut Self {
                self.x -= s;
                self.y -= s;
                self.z -= s;
                self
            }

            fn add(&mut self, v: &Self) -> &mut Self {
                self.x += v.x;
                self.y += v.y;
                self.z += v.z;
                self
            }

            fn sub(&mut self, v: &Self) -> &mut Self {
                self.x -= v.x;
                self.y -= v.y;
                self.z -= v.z;
                self
            }

            fn multiply(&mut self, v: &Self) -> &mut Self {
                self.x *= v.x;
                self.y *= v.y;
                self.z *= v.z;
                self
            }

            fn divide(&mut self, v: &Self) -> &mut Self {
                self.x /= v.x;
                self.y /= v.y;
                self.z /= v.z;
                self
            }

            fn add_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
                self.x = a.x + b.x;
                self.y = a.y + b.y;
                self.z = a.z + b.z;
                self
            }

            fn sub_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
                self.x = a.x - b.x;
                self.y = a.y - b.y;
                self.z = a.z - b.z;
                self
            }

            fn multiply_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
                self.x = a.x * b.x;
                self.y = a.y * b.y;
                self.z = a.z * b.z;
                self
            }

            fn negate(&mut self) -> &mut Self {
                self.x = -self.x;
                self.y = -self.y;
                self.z = -self.z;
                self
            }

            fn min(&mut self, v: &Self) -> &mut Self {
                self.x = (self.x).min(v.x);
                self.y = (self.y).min(v.y);
                self.z = (self.z).min(v.z);
                self
            }

            fn max(&mut self, v: &Self) -> &mut Self {
                self.x = (self.x).max(v.x);
                self.y = (self.y).max(v.y);
                self.z = (self.z).max(v.z);
                self
            }

            fn dot(&mut self, v: &Self) -> T {
                self.x * v.x + self.y * v.y + self.z * v.z
            }

            fn round(&mut self) -> &mut Self {
                self.x = self.x.round();
                self.y = self.y.round();
                self.z = self.z.round();
                self
            }

            fn floor(&mut self) -> &mut Self {
                self.x = self.x.floor();
                self.y = self.y.floor();
                self.z = self.z.floor();
                self
            }

            fn ceil(&mut self) -> &mut Self {
                self.x = self.x.ceil();
                self.y = self.y.ceil();
                self.z = self.z.ceil();
                self
            }

            fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self {
        		self.x = min.x.max(max.x.min(self.x));
        		self.y = min.y.max(max.y.min(self.y));
        		self.z = min.z.max(max.z.min(self.z));
        		self
            }

            fn lerp (&mut self, v: &Self,  alpha:T )-> &mut Self {
                self.x += ( v.x - self.x ) * alpha;
                self.y += ( v.y - self.y ) * alpha;
                self.z += ( v.z - self.z ) * alpha;
        		self
            }

            fn zero() -> Self {
                Self{x: T::zero(), y: T::zero(), z: T::zero()}
            }

            fn cross_vectors ( &mut self, a: &Self, b: &Self ) -> &mut Self {
                let ax = a.x;
                let ay = a.y;
                let az = a.z;
                let bx = b.x;
                let by = b.y;
                let bz = b.z;
                self.x = ay * bz - az * by;
                self.y = az * bx - ax * bz;
                self.z = ax * by - ay * bx;
                self
            }

            fn cross (&mut self, v: &Self )-> &mut Self {
                let c = Self::clone(self);
                self.cross_vectors(&c , v )
            }

            fn set(&mut self, x: T, y: T, z: T) -> &mut Self {
                self.x = x;
                self.y = y;
                self.z = z;
                self
            }

        	// fn set_from_matrix_column (&mut self, m: &Matrix4<T>, index: usize ) -> &mut Self {
            //     let i = index * 4;
            //     self.from_array( &m.elements[i..i*4] );
        	// 	self
        	// }

        	fn from_array (&mut self, array: &[T] ) -> &mut Self {
        		self.x = array[ 0 ];
        		self.y = array[ 1 ];
        		self.z = array[ 2 ];
                self
        	}

        	// fn apply_matrix_4 (&mut self, m: &Matrix4<T> ) -> &mut Self {
            //     let one:T = Nums::one();

            //     let x = self.x; let y = self.y; let z = self.z;
        	// 	let e = m.elements;
        	// 	let w = one / ( e[ 3 ] * x + e[ 7 ] * y + e[ 11 ] * z + e[ 15 ] );

            //     self.x = ( e[ 0 ] * x + e[ 4 ] * y + e[ 8 ] * z + e[ 12 ] ) * w;
        	// 	self.y = ( e[ 1 ] * x + e[ 5 ] * y + e[ 9 ] * z + e[ 13 ] ) * w;
        	// 	self.z = ( e[ 2 ] * x + e[ 6 ] * y + e[ 10 ] * z + e[ 14 ] ) * w;

            //     self
        	// }


        	// fn apply_matrix_3 (&mut self, m: &Matrix3<T> ) -> &mut Self {
        	// 	let x = self.x; let y = self.y; let z = self.z;
        	// 	let e = m.elements;
        	// 	self.x = e[ 0 ] * x + e[ 3 ] * y + e[ 6 ] * z;
        	// 	self.y = e[ 1 ] * x + e[ 4 ] * y + e[ 7 ] * z;
        	// 	self.z = e[ 2 ] * x + e[ 5 ] * y + e[ 8 ] * z;
        	// 	self
        	// }
        }



impl <T> Sub for Vector3<T>
where T:Nums
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut v = Vector3::new();
        v.sub_vectors(&self, &rhs);
        v
    }

}

impl <'a,'b, T> Sub for &'a Vector3<T>
where T:Nums
{
    type Output = Vector3<T>;
    fn sub(self, rhs: &'a Vector3<T>) -> Vector3<T> {
        let mut v = Vector3::new();
        v.sub_vectors(&self, &rhs);
        v
    }
}

// deriveVector!(Vector3_64, f64);

// impl Vector<f64> for Vector3
// {
// // where T:Mul+MulAssign+Ord+FloatDefaults<f64>+Div<Output=T>{
//
//     fn new() -> Self {
//         Self { x: 0.0, y: 0.0, z: 0.0 }
//     }
//
//     fn multiply_scalar(&mut self, s: f64) -> &mut Self {
//         self.x *= s;
//         self.y *= s;
//         self.z *= s;
//         self
//     }
//
//     fn length(&self) -> f64 {
//         (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
//         // return Math.sqrt( this.x * this.x + this.y * this.y + this.z * this.z );
//     }
//
//     fn length_sq(&self) -> f64 {
//         self.x * self.x + self.y * self.y + self.z * self.z
//     }
//
//     fn manhattan_length(&mut self) -> f64 {
//         (self.x).abs() + (self.y).abs() + (self.z).abs()
//     }
//
//     fn set_scalar(&mut self, s: f64) -> &mut Self {
//         self.x = s;
//         self.y = s;
//         self.z = s;
//         self
//     }
//
//     fn add_scalar(&mut self, s: f64) -> &mut Self {
//         self.x += s;
//         self.y += s;
//         self.z += s;
//         self
//     }
//
//     fn sub_scalar(&mut self, s: f64) -> &mut Self {
//         self.x -= s;
//         self.y -= s;
//         self.z -= s;
//         self
//     }
//
//     fn add(&mut self, v: &Self) -> &mut Self {
//         self.x += v.x;
//         self.y += v.y;
//         self.z += v.z;
//         self
//     }
//
//     fn sub(&mut self, v: &Self) -> &mut Self {
//         self.x -= v.x;
//         self.y -= v.y;
//         self.z -= v.z;
//         self
//     }
//
//     fn multiply(&mut self, v: &Self) -> &mut Self {
//         self.x *= v.x;
//         self.y *= v.y;
//         self.z *= v.z;
//         self
//     }
//
//     fn divide(&mut self, v: &Self) -> &mut Self {
//         self.x /= v.x;
//         self.y /= v.y;
//         self.z /= v.z;
//         self
//     }
//
//     fn add_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
//         self.x = a.x + b.x;
//         self.y = a.y + b.y;
//         self.z = a.z + b.z;
//         self
//     }
//
//     fn sub_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
//         self.x = a.x - b.x;
//         self.y = a.y - b.y;
//         self.z = a.z - b.z;
//         self
//     }
//
//     fn multiply_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
//         self.x = a.x * b.x;
//         self.y = a.y * b.y;
//         self.z = a.z * b.z;
//         self
//     }
//
//     fn negate(&mut self) -> &mut Self {
//         self.x = -self.x;
//         self.y = -self.y;
//         self.z = -self.z;
//         self
//     }
//
//     fn min(&mut self, v: &Self) -> &mut Self {
//         self.x = (self.x).min(v.x);
//         self.y = (self.y).min(v.y);
//         self.z = (self.z).min(v.z);
//         self
//     }
//
//     fn max(&mut self, v: &Self) -> &mut Self {
//         self.x = (self.x).max(v.x);
//         self.y = (self.y).max(v.y);
//         self.z = (self.z).max(v.z);
//         self
//     }
//
//     fn dot(&mut self, v: &Self) -> f64 {
//         self.x * v.x + self.y * v.y + self.z * v.z
//     }
//
//     fn round(&mut self) -> &mut Self {
//         self.x = self.x.round();
//         self.y = self.y.round();
//         self.z = self.z.round();
//         self
//     }
//
//     fn floor(&mut self) -> &mut Self {
//         self.x = self.x.floor();
//         self.y = self.y.floor();
//         self.z = self.z.floor();
//         self
//     }
//
//     fn ceil(&mut self) -> &mut Self {
//         self.x = self.x.ceil();
//         self.y = self.y.ceil();
//         self.z = self.z.ceil();
//         self
//     }
//
//     fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self {
// 		self.x = min.x.max(max.x.min(self.x));
// 		self.y = min.y.max(max.y.min(self.y));
// 		self.z = min.z.max(max.z.min(self.z));
// 		self
//     }
//
//     fn lerp (&mut self, v: &Self,  alpha:f64 )-> &mut Self {
//         self.x += ( v.x - self.x ) * alpha;
//         self.y += ( v.y - self.y ) * alpha;
//         self.z += ( v.z - self.z ) * alpha;
// 		self
//     }
//
//     fn zero() -> Self {
//         Self{x:0.0,y:0.0,z:0.0}
//     }
//
//     fn cross_vectors ( &mut self, a: &Self, b: &Self ) -> &mut Self {
//         let ax = a.x;
//         let ay = a.y;
//         let az = a.z;
//         let bx = b.x;
//         let by = b.y;
//         let bz = b.z;
//         self.x = ay * bz - az * by;
//         self.y = az * bx - ax * bz;
//         self.z = ax * by - ay * bx;
//         self
//     }
//
//     fn cross (&mut self, v: &Self )-> &mut Self {
//         let c = Self::clone(self);
//         self.cross_vectors(&c , v )
//     }
//
//     fn set(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
//         self.x = x;
//         self.y = y;
//         self.z = z;
//         self
//     }
//
// 	fn set_from_matrix_column (&mut self, m: &Matrix4, index: usize ) -> &mut Self {
//         let i = index * 4;
//         self.from_array( &m.elements[i..i*4] );
// 		self
// 	}
//
//     // 	fromArray: function ( array, offset ) {
//     // 		if ( offset === undefined ) offset = 0;
//     // 		this.x = array[ offset ];
//     // 		this.y = array[ offset + 1 ];
//     // 		this.z = array[ offset + 2 ];
//     // 		return this;
//     // 	},
// 	fn from_array (&mut self, array: &[f64] ) -> &mut Self {
// 		self.x = array[ 0 ];
// 		self.y = array[ 1 ];
// 		self.z = array[ 2 ];
//         self
// 	}
//
// 	fn apply_matrix_4 (&mut self, m: &Matrix4 ) -> &mut Self {
// 		let x = self.x; let y = self.y; let z = self.z;
// 		let e = m.elements;
// 		let w = 1.0 / ( e[ 3 ] * x + e[ 7 ] * y + e[ 11 ] * z + e[ 15 ] );
//
//         self.x = ( e[ 0 ] * x + e[ 4 ] * y + e[ 8 ] * z + e[ 12 ] ) * w;
// 		self.y = ( e[ 1 ] * x + e[ 5 ] * y + e[ 9 ] * z + e[ 13 ] ) * w;
// 		self.z = ( e[ 2 ] * x + e[ 6 ] * y + e[ 10 ] * z + e[ 14 ] ) * w;
//
//         self
// 	}
//
//
// 	fn apply_matrix_3 (&mut self, m: &Matrix3 ) -> &mut Self {
// 		let x = self.x; let y = self.y; let z = self.z;
// 		let e = m.elements;
// 		self.x = e[ 0 ] * x + e[ 3 ] * y + e[ 6 ] * z;
// 		self.y = e[ 1 ] * x + e[ 4 ] * y + e[ 7 ] * z;
// 		self.z = e[ 2 ] * x + e[ 5 ] * y + e[ 8 ] * z;
// 		self
// 	}
// }
































// 	clampScalar: function () {
// 		var min = new Vector3();
// 		var max = new Vector3();
// 		return function clampScalar( minVal, maxVal ) {
// 			min.set( minVal, minVal, minVal );
// 			max.set( maxVal, maxVal, maxVal );
// 			return this.clamp( min, max );
// 		};
// 	}(),

// 	roundToZero: function () {
// 		this.x = ( this.x < 0 ) ? Math.ceil( this.x ) : Math.floor( this.x );
// 		this.y = ( this.y < 0 ) ? Math.ceil( this.y ) : Math.floor( this.y );
// 		this.z = ( this.z < 0 ) ? Math.ceil( this.z ) : Math.floor( this.z );
// 		return this;
// 	},






// 	projectOnVector: function ( vector ) {
// 		var scalar = vector.dot( this ) / vector.length_sq();
// 		return this.copy( vector ).multiply_scalar( scalar );
// 	},
// 	projectOnPlane: function () {
// 		var v1 = new Vector3();
// 		return function projectOnPlane( planeNormal ) {
// 			v1.copy( this ).projectOnVector( planeNormal );
// 			return this.sub( v1 );
// 		};
// 	}(),
// 	reflect: function () {
// 		// reflect incident vector off plane orthogonal to normal
// 		// normal is assumed to have unit length
// 		var v1 = new Vector3();
// 		return function reflect( normal ) {
// 			return this.sub( v1.copy( normal ).multiply_scalar( 2 * this.dot( normal ) ) );
// 		};
// 	}(),
// 	angleTo: function ( v ) {
// 		var theta = this.dot( v ) / ( Math.sqrt( this.length_sq() * v.length_sq() ) );
// 		// clamp, to handle numerical problems
// 		return Math.acos( _Math.clamp( theta, - 1, 1 ) );
// 	},
// 	distanceTo: function ( v ) {
// 		return Math.sqrt( this.distanceToSquared( v ) );
// 	},
// 	distanceToSquared: function ( v ) {
// 		var dx = this.x - v.x, dy = this.y - v.y, dz = this.z - v.z;
// 		return dx * dx + dy * dy + dz * dz;
// 	},
// 	manhattanDistanceTo: function ( v ) {
// 		return Math.abs( this.x - v.x ) + Math.abs( this.y - v.y ) + Math.abs( this.z - v.z );
// 	},
// 	setFromSpherical: function ( s ) {
// 		var sinPhiRadius = Math.sin( s.phi ) * s.radius;
// 		this.x = sinPhiRadius * Math.sin( s.theta );
// 		this.y = Math.cos( s.phi ) * s.radius;
// 		this.z = sinPhiRadius * Math.cos( s.theta );
// 		return this;
// 	},
// 	setFromCylindrical: function ( c ) {
// 		this.x = c.radius * Math.sin( c.theta );
// 		this.y = c.y;
// 		this.z = c.radius * Math.cos( c.theta );
// 		return this;
// 	},
// 	setFromMatrixPosition: function ( m ) {
// 		var e = m.elements;
// 		this.x = e[ 12 ];
// 		this.y = e[ 13 ];
// 		this.z = e[ 14 ];
// 		return this;
// 	},
// 	setFromMatrixScale: function ( m ) {
// 		var sx = this.setFromMatrixColumn( m, 0 ).length();
// 		var sy = this.setFromMatrixColumn( m, 1 ).length();
// 		var sz = this.setFromMatrixColumn( m, 2 ).length();
// 		this.x = sx;
// 		this.y = sy;
// 		this.z = sz;
// 		return this;
// 	},

// 	equals: function ( v ) {
// 		return ( ( v.x === this.x ) && ( v.y === this.y ) && ( v.z === this.z ) );
// 	},

// 	toArray: function ( array, offset ) {
// 		if ( array === undefined ) array = [];
// 		if ( offset === undefined ) offset = 0;
// 		array[ offset ] = this.x;
// 		array[ offset + 1 ] = this.y;
// 		array[ offset + 2 ] = this.z;
// 		return array;
// 	},
// 	fromBufferAttribute: function ( attribute, index, offset ) {
// 		if ( offset !== undefined ) {
// 			console.warn( 'THREE.Vector3: offset has been removed from .fromBufferAttribute().' );
// 		}
// 		this.x = attribute.getX( index );
// 		this.y = attribute.getY( index );
// 		this.z = attribute.getZ( index );
// 		return this;
// 	}
// } );
// // 	setComponent: function ( index, value ) {
// 		switch ( index ) {
// 			case 0: this.x = value; break;
// 			case 1: this.y = value; break;
// 			case 2: this.z = value; break;
// 			default: throw new Error( 'index is out of range: ' + index );
// 		}
// 		return this;
// 	},
// 	getComponent: function ( index ) {
// 		switch ( index ) {
// 			case 0: return this.x;
// 			case 1: return this.y;
// 			case 2: return this.z;
// 			default: throw new Error( 'index is out of range: ' + index );
// 		}
// 	},

// 	// 	addScaledVector: function ( v, s ) {
// 		this.x += v.x * s;
// 		this.y += v.y * s;
// 		this.z += v.z * s;
// 		return this;
// 	},
// 	// 	applyEuler: function () {
// 		var quaternion = new Quaternion();
// 		return function applyEuler( euler ) {
// 			if ( ! ( euler && euler.isEuler ) ) {
// 				console.error( 'THREE.Vector3: .applyEuler() now expects an Euler rotation rather than a Vector3 and order.' );
// 			}
// 			return this.applyQuaternion( quaternion.setFromEuler( euler ) );
// 		};
// 	}(),
// 	applyAxisAngle: function () {
// 		var quaternion = new Quaternion();
// 		return function applyAxisAngle( axis, angle ) {
// 			return this.applyQuaternion( quaternion.setFromAxisAngle( axis, angle ) );
// 		};
// 	}(),


// 	applyQuaternion: function ( q ) {
// 		var x = this.x, y = this.y, z = this.z;
// 		var qx = q.x, qy = q.y, qz = q.z, qw = q.w;
// 		// calculate quat * vector
// 		var ix = qw * x + qy * z - qz * y;
// 		var iy = qw * y + qz * x - qx * z;
// 		var iz = qw * z + qx * y - qy * x;
// 		var iw = - qx * x - qy * y - qz * z;
// 		// calculate result * inverse quat
// 		this.x = ix * qw + iw * - qx + iy * - qz - iz * - qy;
// 		this.y = iy * qw + iw * - qy + iz * - qx - ix * - qz;
// 		this.z = iz * qw + iw * - qz + ix * - qy - iy * - qx;
// 		return this;
// 	},
// 	project: function () {
// 		var matrix = new Matrix4();
// 		return function project( camera ) {
// 			matrix.multiplyMatrices( camera.projectionMatrix, matrix.getInverse( camera.matrixWorld ) );
// 			return this.apply_matrix_4( matrix );
// 		};
// 	}(),
// 	unproject: function () {
// 		var matrix = new Matrix4();
// 		return function unproject( camera ) {
// 			matrix.multiplyMatrices( camera.matrixWorld, matrix.getInverse( camera.projectionMatrix ) );
// 			return this.apply_matrix_4( matrix );
// 		};
// 	}(),
// 	transformDirection: function ( m ) {
// 		// input: THREE.Matrix4 affine matrix
// 		// vector interpreted as a direction
// 		var x = this.x, y = this.y, z = this.z;
// 		var e = m.elements;
// 		this.x = e[ 0 ] * x + e[ 4 ] * y + e[ 8 ] * z;
// 		this.y = e[ 1 ] * x + e[ 5 ] * y + e[ 9 ] * z;
// 		this.z = e[ 2 ] * x + e[ 6 ] * y + e[ 10 ] * z;
// 		return this.normalize();
// 	},
