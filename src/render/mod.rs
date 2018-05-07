pub mod open_gl;



pub trait Renderer {
	fn new () -> Self;
	fn render<N>(&self, node: N);
	fn clear(&self);
}