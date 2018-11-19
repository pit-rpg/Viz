use helpers::Nums;
use super::Vector;
use super::Vector3;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vector4<T>
where T:Nums
{
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[allow(dead_code)]
impl <T>Vector4<T>
where T:Nums
{
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn set(&mut self, x:T, y:T, z:T, w:T) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self.w = w;
        self
    }

    pub fn from_vector3(&mut self, vec3: &Vector3<T>, w:T) -> &mut Self {
        self.x = vec3.x;
        self.y = vec3.y;
        self.z = vec3.z;
        self.w = w;
        self
    }

	pub fn as_hex_color(&self) -> String {
		format!(
			"#{:X}{:X}{:X}{:X}",
			self.x.as_u8(),
			self.y.as_u8(),
			self.z.as_u8(),
			self.w.as_u8(),
		)
	}
	
	pub fn as_u8_color(&self) -> [u8;4] {
		[	self.x.as_u8(),
			self.y.as_u8(),
			self.z.as_u8(),
			self.w.as_u8()
		]
	}
}


impl <T> Vector<T> for Vector4<T>
where T:Nums
{

    fn new_zero() -> Self {
        Self { x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() }
    }

    fn new_one() -> Self {
        Self { x: T::one(), y: T::one(), z: T::one(), w: T::one() }
    }

    fn random() -> Self {
        Self { x: T::random(), y: T::random(), z: T::random(), w: T::random() }
    }

    fn randomize(&mut self) -> &mut Self {
        self.set(T::random(), T::random(), T::random(), T::random())
    }

    fn copy (&mut self, v: &Self) -> &mut Self {
        self.set(v.x, v.y, v.z, v.w)
    }

    fn multiply_scalar(&mut self, s: T) -> &mut Self {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self.w *= s;
        self
    }

    fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    fn manhattan_length(&mut self) -> T {
        (self.x).abs() + (self.y).abs() + (self.z).abs() + (self.w).abs()
    }

    fn set_scalar(&mut self, s: T) -> &mut Self {
        self.x = s;
        self.y = s;
        self.z = s;
        self.w = s;
        self
    }

    fn add_scalar(&mut self, s: T) -> &mut Self {
        self.x += s;
        self.y += s;
        self.z += s;
        self.w += s;
        self
    }

    fn sub_scalar(&mut self, s: T) -> &mut Self {
        self.x -= s;
        self.y -= s;
        self.z -= s;
        self.w -= s;
        self
    }

    fn add(&mut self, v: &Self) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self.w += v.w;
        self
    }

    fn sub(&mut self, v: &Self) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
        self.w -= v.w;
        self
    }

    fn multiply(&mut self, v: &Self) -> &mut Self {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
        self.w *= v.w;
        self
    }

    fn divide(&mut self, v: &Self) -> &mut Self {
        self.x /= v.x;
        self.y /= v.y;
        self.z /= v.z;
        self.w /= v.w;
        self
    }

    fn add_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        self.z = a.z + b.z;
        self.w = a.w + b.w;
        self
    }

    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x - b.x;
        self.y = a.y - b.y;
        self.z = a.z - b.z;
        self.w = a.w - b.w;
        self
    }

    fn multiply_vectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x * b.x;
        self.y = a.y * b.y;
        self.z = a.z * b.z;
        self.w = a.w * b.w;
        self
    }

    fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
        self
    }

    fn min(&mut self, v: &Self) -> &mut Self {
        self.x = (self.x).min(v.x);
        self.y = (self.y).min(v.y);
        self.z = (self.z).min(v.z);
        self.w = (self.w).min(v.w);
        self
    }

    fn max(&mut self, v: &Self) -> &mut Self {
        self.x = (self.x).max(v.x);
        self.y = (self.y).max(v.y);
        self.z = (self.z).max(v.z);
        self.w = (self.w).max(v.w);
        self
    }

    fn dot(&mut self, v: &Self) -> T {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    fn round(&mut self) -> &mut Self {
        self.x = self.x.round();
        self.y = self.y.round();
        self.z = self.z.round();
        self.w = self.w.round();
        self
    }

    fn floor(&mut self) -> &mut Self {
        self.x = self.x.floor();
        self.y = self.y.floor();
        self.z = self.z.floor();
        self.w = self.w.floor();
        self
    }

    fn ceil(&mut self) -> &mut Self {
        self.x = self.x.ceil();
        self.y = self.y.ceil();
        self.z = self.z.ceil();
        self.w = self.w.ceil();
        self
    }

    fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self {
        self.x = min.x.max(max.x.min(self.x));
        self.y = min.y.max(max.y.min(self.y));
        self.z = min.z.max(max.z.min(self.z));
        self.w = min.w.max(max.w.min(self.w));
        self
    }

    fn lerp (&mut self, v: &Self,  alpha:T )-> &mut Self {
        self.x += ( v.x - self.x ) * alpha;
        self.y += ( v.y - self.y ) * alpha;
        self.z += ( v.z - self.z ) * alpha;
        self.w += ( v.w - self.w ) * alpha;
        self
    }

    fn zero() -> Self {
        Self{x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero()}
    }

	fn equals(&self, v: &Self ) -> bool {
		( v.x == self.x ) && ( v.y == self.y ) && ( v.z == self.z ) && ( v.w == self.w )
	}

    fn from_array (&mut self, array: &[T] ) -> &mut Self {
        self.x = array[ 0 ];
        self.y = array[ 1 ];
        self.z = array[ 2 ];
        self.w = array[ 3 ];
        self
    }
}
