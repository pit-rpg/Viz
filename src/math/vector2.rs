// mod vector;
//
// use math::vector::Vector;

use helpers::Nums;
use super::Vector;

// #[repr(C)]
#[derive(Clone, Debug)]
pub struct Vector2<T>
where T: Nums
{
    pub x: T,
    pub y: T,
}

#[allow(dead_code)]
impl <T> Vector2<T>
where T: Nums
{
    pub fn new (x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn set(&mut self, x: T, y: T) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
}


impl <T> Vector<T> for Vector2<T>
where T:Nums
        {

            fn new_zero() -> Self {
                Self { x: T::zero(), y: T::zero() }
            }

            fn new_one() -> Self {
                Self { x: T::one(), y: T::one() }
            }

            fn copy (&mut self, v: Self) -> &mut Self {
                self.set(v.x, v.y)
            }

            fn multiply_scalar(&mut self, s: T) -> &mut Self {
                self.x *= s;
                self.y *= s;
                self
            }

            fn length(&self) -> T {
                (self.x * self.x + self.y * self.y).sqrt()
            }

            fn length_sq(&self) -> T {
                self.x * self.x + self.y * self.y
            }

            fn manhattan_length(&mut self) -> T {
                (self.x).abs() + (self.y).abs()
            }

            fn set_scalar(&mut self, s: T) -> &mut Self {
                self.x = s;
                self.y = s;
                self
            }

            fn add_scalar(&mut self, s: T) -> &mut Self {
                self.x += s;
                self.y += s;
                self
            }

            fn sub_scalar(&mut self, s: T) -> &mut Self {
                self.x -= s;
                self.y -= s;
                self
            }

            fn add(&mut self, v: &Self) -> &mut Self {
                self.x += v.x;
                self.y += v.y;
                self
            }

            fn sub(&mut self, v: &Self) -> &mut Self {
                self.x -= v.x;
                self.y -= v.y;
                self
            }

            fn multiply(&mut self, v: &Self) -> &mut Self {
                self.x *= v.x;
                self.y *= v.y;
                self
            }

            fn divide(&mut self, v: &Self) -> &mut Self {
                self.x /= v.x;
                self.y /= v.y;
                self
            }

            fn add_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
                self.x = a.x + b.x;
                self.y = a.y + b.y;
                self
            }

            fn sub_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
                self.x = a.x - b.x;
                self.y = a.y - b.y;
                self
            }

            fn multiply_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
                self.x = a.x * b.x;
                self.y = a.y * b.y;
                self
            }

            fn negate(&mut self) -> &mut Self {
                self.x = -self.x;
                self.y = -self.y;
                self
            }

            fn min(&mut self, v: &Self) -> &mut Self {
                self.x = (self.x).min(v.x);
                self.y = (self.y).min(v.y);
                self
            }

            fn max(&mut self, v: &Self) -> &mut Self {
                self.x = (self.x).max(v.x);
                self.y = (self.y).max(v.y);
                self
            }

            fn dot(&mut self, v: &Self) -> T {
                self.x * v.x + self.y * v.y
            }

            fn round(&mut self) -> &mut Self {
                self.x = self.x.round();
                self.y = self.y.round();
                self
            }

            fn floor(&mut self) -> &mut Self {
                self.x = self.x.floor();
                self.y = self.y.floor();
                self
            }

            fn ceil(&mut self) -> &mut Self {
                self.x = self.x.ceil();
                self.y = self.y.ceil();
                self
            }

            fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self {
        		self.x = min.x.max(max.x.min(self.x));
        		self.y = min.y.max(max.y.min(self.y));
        		self
            }

            fn lerp (&mut self, v: &Self,  alpha:T )-> &mut Self {
                self.x += ( v.x - self.x ) * alpha;
                self.y += ( v.y - self.y ) * alpha;
        		self
            }

            fn zero() -> Self {
                Self{x: T::zero(), y: T::zero()}
            }

            fn set(&mut self, x: T, y: T, z: T) -> &mut Self {
                self.x = x;
                self.y = y;
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







// impl Vector for Vector2 {

//     fn multiplyScalar(&mut self, s: f64) -> &mut Self {
//         self.x *= s;
//         self.y *= s;
//         self
//     }

//     fn length(&self) -> f64 {
//         (self.x * self.x + self.y * self.y).sqrt()
//         // return Math.sqrt( this.x * this.x + this.y * this.y + this.z * this.z );
//     }

//     fn lengthSq(&self) -> f64 {
//         self.x * self.x + self.y * self.y
//     }

//     fn manhattanLength(&mut self) -> f64 {
//         (self.x).abs() + (self.y).abs()
//     }


//     fn setScalar(&mut self, s: f64) -> &mut Self {
//         self.x = s;
//         self.y = s;
//         self
//     }

//     fn addScalar(&mut self, s: f64) -> &mut Self {
//         self.x += s;
//         self.y += s;
//         self
//     }

//     fn subScalar(&mut self, s: f64) -> &mut Self {
//         self.x -= s;
//         self.y -= s;
//         self
//     }

//     fn add(&mut self, v: &Self) -> &mut Self {
//         self.x += v.x;
//         self.y += v.y;
//         self
//     }

//     fn sub(&mut self, v: &Self) -> &mut Self {
//         self.x -= v.x;
//         self.y -= v.y;
//         self
//     }

//     fn multiply(&mut self, v: &Self) -> &mut Self {
//         self.x *= v.x;
//         self.y *= v.y;
//         self
//     }

//     fn divide(&mut self, v: &Self) -> &mut Self {
//         self.x /= v.x;
//         self.y /= v.y;
//         self
//     }

//     fn addVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
//         self.x = a.x + b.x;
//         self.y = a.y + b.y;
//         self
//     }

//     fn subVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
//         self.x = a.x - b.x;
//         self.y = a.y - b.y;
//         self
//     }

//     fn multiplyVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
//         self.x = a.x * b.x;
//         self.y = a.y * b.y;
//         self
//     }

//     fn negate(&mut self) -> &mut Self {
//         self.x = -self.x;
//         self.y = -self.y;
//         self
//     }

//     fn min(&mut self, v: &Self) -> &mut Self {
//         self.x = (self.x).min(v.x);
//         self.y = (self.y).min(v.y);
//         self
//     }

//     fn max(&mut self, v: &Self) -> &mut Self {
//         self.x = (self.x).max(v.x);
//         self.y = (self.y).max(v.y);
//         self
//     }

//     fn dot(&mut self, v: &Self) -> f64 {
//         self.x * v.x + self.y * v.y
//     }

//     fn round(&mut self) -> &mut Self {
//         self.x = self.x.round();
//         self.y = self.y.round();
//         self
//     }

//     fn floor(&mut self) -> &mut Self {
//         self.x = self.x.floor();
//         self.y = self.y.floor();
//         self
//     }

//     fn ceil(&mut self) -> &mut Self {
//         self.x = self.x.ceil();
//         self.y = self.y.ceil();
//         self
//     }

//     fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self {
// 		self.x = min.x.max(max.x.min(self.x));
// 		self.y = min.y.max(max.y.min(self.y));
// 		self
//     }

//     fn lerp (&mut self, v: &Self,  alpha:f64 )-> &mut Self {
//         self.x += ( v.x - self.x ) * alpha;
//         self.y += ( v.y - self.y ) * alpha;
// 		self
//     }

//     fn zero() -> Self {
//         Vector2{x:0.0,y:0.0}
//     }
// }
