pub mod open_gl;


pub trait Renderer {
	fn new () -> Self;
	fn render();
	fn clear(&self);
}

pub trait Render {
	fn render<T:Renderer>(&self, renderer: &mut T);
}