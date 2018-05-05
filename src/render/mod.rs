pub mod render_gl;



pub trait Renderer {
	fn new () -> Self;
	fn render<N>(node: N);
	fn clear();
}