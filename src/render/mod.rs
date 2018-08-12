pub mod open_gl;
use helpers::Nums;
// use core::Node;


pub trait Renderer {
	fn new () -> Self;
	fn render();
	fn clear(&self);
}

pub trait Render {
	fn render<T:Renderer>(&self, Renderer: &mut T);
}