use std::f64::consts::PI as PI_f64;
use std::path::PathBuf;
use std::time::Duration;
use std::{thread, thread::JoinHandle};

use crate::{
	core::{
		FrameOutput, Light, LightType, Material, Node, NodeData, PerspectiveCamera, SharedFrameBuffer, SharedGeometry,
		Transform, UniformName,
	},
	glutin::event_loop::{ControlFlow, EventLoop},
	glutin::{
		event::{Event, MouseScrollDelta, WindowEvent},
		window::Window,
		ContextWrapper, PossiblyCurrent,

	},
	helpers::{geometry_generators, load_gltf},
	math::{Vector, Vector3, Vector4},
	render,
	render::open_gl::system_render::RenderSystem,
};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WindowState {
	pub pointer_pos: (f64, f64),
	pub pointer_pressed: (bool, bool, bool),
	pub pointer_wheel: f32,
	pub window_size: (u32, u32),
}

pub struct DemoRunner
// where R: Fn(&mut RenderSystem)
{}

#[allow(dead_code)]
impl DemoRunner {
	pub fn run<R>(render_func: R)
	where
		R: Send + 'static + Fn(&mut RenderSystem),
	{
		let (mut render_system, event_loop, windowed_context) =
			render::open_gl::system_render::RenderSystem::build(true, true, true);

		render_system.clear_color = Some(Vector4::zero());
		windowed_context.window().set_resizable(true);

		let hidpi_factor = windowed_context.window().scale_factor();
		let mut window_state = WindowState::default();

		let mut radius = 20.0;
		let zoom_speed = 0.5;

		event_loop.run(move |event, _, control_flow| {
			*control_flow = ControlFlow::Wait;

			match event {
				Event::WindowEvent { event, .. } => match event {
					WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

					WindowEvent::Resized(physical_scale) => {
						let logical_size = physical_scale.to_logical::<u32>(hidpi_factor);

						window_state.window_size.0 = logical_size.width;
						window_state.window_size.1 = logical_size.height;

						windowed_context.resize(physical_scale);

						gl_call!({
							gl::Viewport(
								0,
								0,
								(logical_size.width as f64 * hidpi_factor) as i32,
								(logical_size.height as f64 * hidpi_factor) as i32,
							);
						});
					}

					WindowEvent::MouseWheel { delta, .. } => match delta {
						MouseScrollDelta::LineDelta(_, y) => {
							if y > 0.0 {
								radius -= zoom_speed
							} else {
								radius += zoom_speed
							};
						}
						MouseScrollDelta::PixelDelta(_) => {}
					},

					WindowEvent::CursorMoved { position: pos, .. } => {
						window_state.pointer_pos = pos.to_logical::<f64>(hidpi_factor).into();
					}
					_ => (),
				},
				Event::Resumed => {

				}
				Event::RedrawRequested(_) => {
					(render_func)(&mut render_system);

					windowed_context.swap_buffers().unwrap();
				}
				_ => (),
			}
		});
	}
}
