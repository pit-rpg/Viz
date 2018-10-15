extern crate uuid;
use self::uuid::Uuid;

use helpers::Nums;
use math::{Vector, Vector2, Vector3, Vector4};


#[allow(dead_code)]
#[derive(Clone)]
pub enum Light<T>
where T: Nums
{
	Point(PointLight<T>),
	Directional(DirectionalLight<T>),
	Ambient(AmbientLight<T>),
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct PointLight<T>
where T: Nums
{
	pub uuid: Uuid,
	pub color: Vector3<T>,
	pub power: T,

	pub constant: T,
	pub linear: T,
	pub quadratic: T,
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct DirectionalLight<T>
where T: Nums
{
	pub uuid: Uuid,
	pub color: Vector3<T>,
	pub power: T,
}


#[allow(dead_code)]
#[derive(Clone)]
pub struct AmbientLight<T>
where T: Nums
{
	pub uuid: Uuid,
	pub color: Vector3<T>,
	pub power: T,
}


impl <T> PointLight<T>
where T:Nums
{
	pub fn new (color: Vector3<T>, power: T) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color,
			power,
			constant: T::from_f32(1.0),
			linear: T::from_f32(0.7),
			quadratic: T::from_f32(1.8),
		}
	}
}

impl <T> AmbientLight<T>
where T:Nums
{
	pub fn new (color: Vector3<T>, power: T) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			color,
			power,
		}
	}
}


#[allow(dead_code)]
impl <T> Light<T>
where T: Nums
{
	pub fn new_point(color: Vector3<T>, power: T) -> Self {
		Light::Point(PointLight::new(color, power))
	}

	pub fn new_ambient_light(color: Vector3<T>, power: T) -> Self {
		Light::Ambient(AmbientLight::new(color, power))
	}
}