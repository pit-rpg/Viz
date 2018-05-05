extern crate gl;
extern crate glutin;
extern crate rand;

use super::super::Renderer;
use self::glutin::{
	EventsLoop,
	GlWindow,
	GlContext,
};
use self::gl::{
	GetString,
};
use std::ffi::{
	CStr,
	// CString,
};


#[allow(dead_code)]
pub struct GLRenderer {
	window: GlWindow,
	events_loop: EventsLoop,
}


impl Renderer for GLRenderer {
	fn new() -> Self {
		let events_loop = glutin::EventsLoop::new();
		let window = glutin::WindowBuilder::new()
			.with_title("Hello, world!")
			.with_dimensions(1024, 768);

		let context = glutin::ContextBuilder::new().with_vsync(true);

		let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

		unsafe {
			gl_window.make_current().unwrap();
		}

		gl_call!({
			gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
			gl::ClearColor(0.0, 0.2, 0.2, 1.0);
		});

		print_gl_version();

		GLRenderer{
			window: gl_window,
			events_loop,
		}
	}

	fn render<N>(node: N) {}
	fn clear() {}
}


fn print_gl_version() {
    gl_call!({
        let version = GetString(gl::VERSION) as *const i8;
        println!("{:?}", CStr::from_ptr(version));
    });
}
