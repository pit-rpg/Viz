use helpers::Nums;

pub struct Rect<T>
where T: Nums,
{
	pub width: T,
	pub height: T,
	pub x: T,
	pub y: T,
}