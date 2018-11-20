
use helpers::Nums;
use super::Vector;

#[repr(C)]
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

	fn new_max() -> Self {
        Self { x: T::max_val(), y: T::max_val() }
    }
	
	fn new_min() -> Self{
        Self { x: T::min_val(), y: T::min_val() }
    }


    fn random() -> Self {
        Self { x: T::random(), y: T::random() }
    }

	fn randomize(&mut self) -> &mut Self {
		self.set(T::random(), T::random())
	}

    fn copy (&mut self, v: &Self) -> &mut Self {
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

	fn equals(&self, v: &Self ) -> bool {
		( v.x == self.x ) && ( v.y == self.y )
	}

	fn is_zero(&self) -> bool {
		self.x == T::zero() && self.y == T::zero()
	}

    fn from_array (&mut self, array: &[T] ) -> &mut Self {
        self.x = array[ 0 ];
        self.y = array[ 1 ];
        self
    }
}