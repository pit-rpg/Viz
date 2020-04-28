extern crate uuid;
#[macro_use]
extern crate project;

use std::f64::consts::PI as PI_f64;

use project::{
	core::{
		Blending, Material, PerspectiveCamera, NodeData, Node,
		ShaderTag, SharedGeometry,
		SharedTexture2D, Transform, TransformLock, UniformName,
	},
	glutin,
	glutin::MouseScrollDelta,
	helpers::geometry_generators,
	math::{Vector, Vector3},
	render,
};

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct WindowState {
	pub pointer_pos: (f64, f64),
	pub pointer_pressed: (bool, bool, bool),
	pub pointer_wheel: f32,
	pub window_size: (f64, f64),
}

fn main() {
	let emojis = vec![
		"res/emoji/angry-face_1f620.png",
		"res/emoji/astonished-face_1f632.png",
		"res/emoji/confused-face_1f615.png",
		"res/emoji/dizzy-face_1f635.png",
		"res/emoji/drooling-face_1f924.png",
		"res/emoji/extraterrestrial-alien_1f47d.png",
		"res/emoji/eyes_1f440.png",
		"res/emoji/face-savouring-delicious-food_1f60b.png",
		"res/emoji/face-with-cowboy-hat_1f920.png",
		"res/emoji/face-with-finger-covering-closed-lips_1f92b.png",
		"res/emoji/face-with-monocle_1f9d0.png",
		"res/emoji/face-with-one-eyebrow-raised_1f928.png",
		"res/emoji/face-with-open-mouth-vomiting_1f92e.png",
		"res/emoji/face-with-rolling-eyes_1f644.png",
		"res/emoji/face-with-tears-of-joy_1f602.png",
		"res/emoji/face-with-uneven-eyes-and-wavy-mouth_1f974.png",
		"res/emoji/flushed-face_1f633.png",
		"res/emoji/ghost_1f47b.png",
		"res/flash/1.jpg",
		"res/flash/203565_preview.png",
		"res/flash/266371335012212.png",
		"res/flash/burst.jpg",
		"res/flash/eb07a72e2a175be326a53cacac303139.png",
		"res/flash/lolo.png",
		"res/emoji/grimacing-face_1f62c.png",
		"res/emoji/grinning-face-with-one-large-and-one-small-eye_1f92a.png",
		"res/emoji/grinning-face-with-smiling-eyes_1f601.png",
		"res/emoji/grinning-face-with-star-eyes_1f929.png",
		"res/emoji/hugging-face_1f917.png",
		"res/emoji/money-mouth-face_1f911.png",
		"res/emoji/nerd-face_1f913.png",
		"res/emoji/neutral-face_1f610.png",
		"res/emoji/persevering-face_1f623.png",
		"res/emoji/pouting-face_1f621.png",
		"res/emoji/rolling-on-the-floor-laughing_1f923.png",
		"res/emoji/see-no-evil-monkey_1f648.png",
		"res/emoji/serious-face-with-symbols-covering-mouth_1f92c.png",
		"res/emoji/shocked-face-with-exploding-head_1f92f.png",
		"res/emoji/sleeping-face_1f634.png",
		"res/emoji/sleepy-face_1f62a.png",
		"res/emoji/smiling-face-with-halo_1f607.png",
		"res/emoji/smiling-face-with-heart-shaped-eyes_1f60d.png",
		"res/emoji/smiling-face-with-horns_1f608.png",
		"res/emoji/smiling-face-with-open-mouth_1f603.png",
		"res/emoji/smiling-face-with-open-mouth-and-cold-sweat_1f605.png",
		"res/emoji/smiling-face-with-open-mouth-and-smiling-eyes_1f604.png",
		"res/emoji/smiling-face-with-open-mouth-and-tightly-closed-eyes_1f606.png",
		"res/emoji/smiling-face-with-smiling-eyes_1f60a.png",
		"res/emoji/smiling-face-with-smiling-eyes-and-hand-covering-mouth_1f92d.png",
		"res/emoji/smiling-face-with-smiling-eyes-and-three-hearts_1f970.png",
		"res/emoji/smiling-face-with-sunglasses_1f60e.png",
		"res/emoji/smirking-face_1f60f.png",
		"res/emoji/sneezing-face_1f927.png",
		"res/emoji/thinking-face_1f914.png",
		"res/emoji/unamused-face_1f612.png",
		"res/emoji/upside-down-face_1f643.png",
		"res/emoji/winking-face_1f609.png",
		"res/emoji/zipper-mouth-face_1f910.png",
	];

	let root = Node::new("root");

	let up = Vector3::new_up();
	let center = Vector3::new_zero();
	let mut radius = 20.0;
	let zoom_speed = 0.5;
	let mut running = true;

	let camera = {
		let mut camera = PerspectiveCamera::new();
		let mut transform_camera = Transform::default();
		transform_camera.position.z = 6.0;
		camera.view.enabled = false;

		root.add_child(
			NodeData::new("camera")
				.set_camera(camera)
				.set_transform(transform_camera)
				.to_shared()
		)
	};


	let base_mat = Material::new_mesh_standard();
	let geom_plane = SharedGeometry::new(geometry_generators::plane_buffer_geometry(1.0, 1.0, 1, 1));

	emojis.iter().for_each(|item| {
		let mut pos = Vector3::random();
		pos.multiply_scalar(10.0);
		pos.sub_scalar(5.0);

		let mut mat = base_mat.clone();
		let texture = SharedTexture2D::new_from_path(item);
		mat.set_uniform(UniformName::MapColor, texture);
		mat.set_uniform(UniformName::Alpha, 1.0);

		if item.find("emoji").is_some() {
			mat.blending = Blending::Mix;
		} else {
			mat.blending = Blending::Additive;
		}

		mat.add_tag(ShaderTag::Shadeless);
		mat.add_tag(ShaderTag::Transparent);

		let material = mat.to_shared();
		let mut transform = Transform::from_position(pos);
		transform.lock = TransformLock::RotationScale;
		transform.scale.multiply_scalar(0.1);

		root.add_child(
			NodeData::new("emojy")
				.set_material(material.clone())
				.set_geometry(geom_plane.clone())
				.set_transform(transform)
				.to_shared()
		);
	});

	let mut render_system = render::open_gl::system_render::RenderSystem::new(camera.clone(), true, true, true);

	render_system.windowed_context.window().set_resizable(true);

	let hidpi_factor = render_system
		.windowed_context
		.window()
		.get_hidpi_factor()
		.round();
	let mut window_state = WindowState::default();


	while running {
		{
			let windowed_context = &render_system.windowed_context;
			use self::glutin::WindowEvent::*;

			render_system.events_loop.poll_events(|event| match event {
				glutin::Event::WindowEvent { event, .. } => match event {
					glutin::WindowEvent::CloseRequested => running = false,
					glutin::WindowEvent::Resized(logical_size) => {
						window_state.window_size.0 = logical_size.width;
						window_state.window_size.1 = logical_size.height;

						let dpi_factor = windowed_context.window().get_hidpi_factor();
						windowed_context.resize(logical_size.to_physical(dpi_factor));

						gl_call!({
							gl::Viewport(
								0,
								0,
								(logical_size.width * dpi_factor) as i32,
								(logical_size.height * dpi_factor) as i32,
							);
						});
						println!(
							"logical_size: {:?}, dpi_factor: {:?}",
							logical_size, dpi_factor
						);
					}
					glutin::WindowEvent::MouseWheel { delta, .. } => match delta {
						MouseScrollDelta::LineDelta(_, y) => {
							if y > 0.0 {
								radius -= zoom_speed
							} else {
								radius += zoom_speed
							};
						}
						MouseScrollDelta::PixelDelta(_) => {}
					},
					CursorMoved { position: pos, .. } => {
						window_state.pointer_pos = pos
							.to_physical(windowed_context.window().get_hidpi_factor())
							.to_logical(hidpi_factor)
							.into();
					}
					_ => (),
				},
				_ => (),
			});
		}

		{
			let mut node_data = camera.lock();
			{
				let transform_camera = &mut node_data.transform;
				let x_prog = window_state.pointer_pos.0 / window_state.window_size.0;
				let y_prog = window_state.pointer_pos.1 / window_state.window_size.1;
				transform_camera.position.z = ((x_prog * (PI_f64 * 2.0)).sin() * radius) as f32;
				transform_camera.position.x = ((x_prog * (PI_f64 * 2.0)).cos() * radius) as f32;
				transform_camera.position.y = ((y_prog * radius - radius / 2.0) * -2.0) as f32;
				transform_camera.look_at(&center, &up);
			}

			if let Some(camera) = &mut node_data.camera {
				let aspect = window_state.window_size.0 / window_state.window_size.1;
				camera.aspect = aspect as f32;
				camera.update_projection_matrix();
			}
		}

		render_system.run(&root);
	}
}
