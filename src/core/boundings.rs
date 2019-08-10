use helpers::Nums;
use math::{Vector, Vector3};

pub struct Rect<T>
where T: Nums,
{
	pub width: T,
	pub height: T,
	pub x: T,
	pub y: T,
}


#[derive(Clone, Debug)]
pub struct BBox3<T>
where
	T:Nums
{
	min: Vector3<T>,
	max: Vector3<T>,
}

// pub struct Box2<T>
// where T: Nums,
// {
// 	min: Vector2<T>,
// 	max: Vector2<T>,
// }

impl <T> BBox3<T>
where
	T:Nums
	// V: Vector<T>,
{
	pub fn new(min: Vector3<T>, max: Vector3<T> ) -> Self {
		Self {min, max}
	}

	pub fn new_empty() -> Self {
		Self { min: Vector3::new_zero(), max: Vector3::new_zero() }
	}

	pub fn set(&mut self, min: Vector3<T>, max: Vector3<T>) -> &mut Self {
		self.min = min;
		self.max = max;
		self
	}

	pub fn set_from_array (&mut self, data: &[Vector3<T>] ) -> &mut Self {
		let mut max = Vector3::new_max();
		let mut min = Vector3::new_max();
		max.negate();

		data
			.iter()
			.for_each( |e| {
				max.max(e);
				min.min(e);
			});

		self.set(min, max)
	}

	pub fn is_empty(&self) -> bool {
		self.min.is_zero() && self.max.is_zero()
	}

	pub fn diff(&self, other: &Self) -> Vector3<T>  {
		let mut d1 = self.max.clone();
		let mut d2 = other.max.clone();
		d1.sub(&self.min);
		d2.sub(&other.min);
		d1.divide(&d2);
		d1
	}
}



pub struct BSphare<T>
where
	T:Nums
{
	center: Vector3<T>,
	radius: T,
}

impl <T> BSphare<T>
where
	T:Nums
{
	pub fn new(center: Vector3<T>, radius: T) -> Self {
		Self{center, radius}
	}
}