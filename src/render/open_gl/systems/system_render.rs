extern crate gl;
extern crate glutin;
extern crate rand;
extern crate rayon;

use std::ffi::CStr;
use std::os::raw::c_void;
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use core::{
	Blending, BufferGeometry, BufferGroup, Light, LightType, FrameBuffer, Material, PerspectiveCamera,
	ShaderTag, SharedFrameBuffer, SharedGeometry, SharedMaterial, Transform, TransformLock, UniformName,
	Node, NodeData,
};

use self::gl::types::*;
use self::gl::GetString;
use self::glutin::dpi::*;
use self::glutin::{
	ContextError, ContextWrapper,
	window::{Window},
	event_loop::{EventLoop},
	PossiblyCurrent
};

use super::super::{
	gl_frame_buffer::{GLFrameBuffer, GLFrameBufferIDs},
	gl_geometry::GLVertexArraysIDs,
	gl_material::GLMaterialIDs,
	gl_render_buffer::GLRenderBufferIDs,
	gl_texture::GLTextureIDs,
	GLGeometry, GLMaterial,
};
use math::{Matrix3, Matrix4, Vector, Vector3, Vector4};

pub struct BindContext<'z, 'x> {
	pub tags: &'z Vec<ShaderTag>,
	pub gl_material_ids: &'z mut GLMaterialIDs,
	pub gl_texture_ids: &'z mut GLTextureIDs,

	pub lights_point_count: usize,
	pub lights_directional_count: usize,
	pub geometry: &'x BufferGeometry,
}

struct DrawGroup {
	buffer_group: BufferGroup,
	matrix_model: Matrix4<f32>,
	matrix_projection: Matrix4<f32>,
	matrix_normal: Matrix3<f32>,
	material: SharedMaterial,
	geometry: SharedGeometry,
	distance: f32,
}

pub struct RenderSystem {
	// pub windowed_context: ContextWrapper<glutin::PossiblyCurrent, Window>,
	// pub events_loop: EventLoop<()>,
	pub timer: Instant,
	pub time: Duration,
	pub delta_time: Duration,
	pub delta_max: Option<Duration>,
	pub clear_color: Option<Vector4<f32>>,
	pub tags: Vec<ShaderTag>,
	pub override_material: Option<SharedMaterial>,

	pub _gl_vertex_arrays_ids: GLVertexArraysIDs,
	pub _gl_material_ids: GLMaterialIDs,
	pub _gl_texture_ids: GLTextureIDs,
	pub _gl_frame_buffer_ids: GLFrameBufferIDs,
	pub _gl_render_buffer_ids: GLRenderBufferIDs,

	depth_test: bool,
	stencil_test: bool,
	blending: bool,

	lights_point_count: usize,
	lights_directional_count: usize,


	blending_state: Blending,

	frame_buffer: Option<SharedFrameBuffer>,
	current_frame_buffer: Option<SharedFrameBuffer>,
}

impl RenderSystem {
	pub fn build(
		depth_test: bool,
		stencil_test: bool,
		blending: bool
	) -> (Self, EventLoop<()>, ContextWrapper<PossiblyCurrent, Window>)
	{
		let events_loop = glutin::event_loop::EventLoop::new();

		let window = glutin::window::WindowBuilder::new()
			.with_title("Hello, world!")
			.with_inner_size(LogicalSize::new(1024.0, 768.0));

		let windowed_context = glutin::ContextBuilder::new()
			.with_vsync(true)
			.build_windowed(window, &events_loop)
			.unwrap();

		let windowed_context = unsafe { windowed_context.make_current().unwrap() };

		gl_call!({
			gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);
			// gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
			gl::ClearColor(0.0, 0.2, 0.2, 1.0);
		});

		// Flags
		gl_call!({
			if depth_test {
				gl::Enable(gl::DEPTH_TEST);
			}
			if stencil_test {
				gl::Enable(gl::STENCIL_TEST);
			}


			gl::Disable(gl::FRAMEBUFFER_SRGB);
			// gl::Enable(gl::FRAMEBUFFER_SRGB);
		});
		// /Flags

		RenderSystem::print_gl_version();

		let render_system = Self {
			// windowed_context: windowed_context,
			timer: Instant::now(),
			time: Duration::new(0, 0),
			delta_time: Duration::new(0, 0),
			delta_max: None,
			clear_color: Some(Vector4::new_zero()),
			tags: Vec::new(),
			override_material: None,
			// render_settings: RenderSettings::default(),
			lights_point_count: 0,
			lights_directional_count: 0,

			depth_test,
			stencil_test,
			blending,

			blending_state: Blending::None,

			frame_buffer: None,
			current_frame_buffer: None,

			_gl_vertex_arrays_ids: GLVertexArraysIDs::new(),
			_gl_material_ids: GLMaterialIDs::new(),
			_gl_texture_ids: GLTextureIDs::new(),
			_gl_frame_buffer_ids: GLFrameBufferIDs::new(),
			_gl_render_buffer_ids: GLRenderBufferIDs::new(),
		};

		(render_system, events_loop, windowed_context)
	}

	pub fn clear(&self) {
		gl_call!({
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
		});
	}

	pub fn gl_clear_error() {
		while unsafe { gl::GetError() } != gl::NO_ERROR {}
	}

	pub fn print_gl_version() {
		gl_call!({
			let version = GetString(gl::VERSION) as *const i8;
			println!("{:?}", CStr::from_ptr(version));
		});
	}

	pub fn get_duration(&self) -> f32 {
		self.time.as_secs() as f32 + self.time.subsec_nanos() as f32 * 1e-9
	}

	pub fn get_delta(&self) -> f32 {
		self.delta_time.as_secs() as f32 + self.delta_time.subsec_nanos() as f32 * 1e-9
	}

	fn update_time(&mut self) {
		let new_now = Instant::now();
		let mut delta = new_now.duration_since(self.timer);
		self.timer = new_now;
		match self.delta_max {
			None => {}
			Some(ref mut max) => {
				if delta > *max {
					delta = max.clone()
				}
			}
		}

		self.time += delta;
	}

	fn set_blending_mode(&mut self, blending: Blending) {
		if !self.blending || self.blending_state == blending {
			return;
		}

		if self.blending_state != Blending::None && blending == Blending::None {
			gl_call!({
				gl::Disable(gl::BLEND);
			});
		} else if self.blending_state == Blending::None && blending != Blending::None {
			gl_call!({
				gl::Enable(gl::BLEND);
			});
		}

		match blending {
			Blending::None => {}
			Blending::Mix => gl_call!({
				gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
			}),
			Blending::Additive => gl_call!({
				gl::BlendFunc(gl::SRC_ALPHA, gl::ONE);
			}),
		}

		self.blending_state = blending;
	}

	fn draw_buffer_group<'x, 'z>(
		&mut self,
		mut groupe: DrawGroup,
	) {
		{
			let geometry = &groupe.geometry.lock().unwrap();
			geometry.bind(&mut self._gl_vertex_arrays_ids);

			let material = &mut groupe.material.lock().unwrap();
			self.set_blending_mode(material.blending);

			let mut bind_context = BindContext {
				gl_texture_ids: &mut self._gl_texture_ids,
				gl_material_ids: &mut self._gl_material_ids,
				tags: &self.tags,
				lights_point_count: self.lights_point_count,
				lights_directional_count: self.lights_directional_count,
				geometry,
			};

			material.set_uniform(UniformName::MatrixModel, groupe.matrix_model);
			material.set_uniform(UniformName::MatrixView, groupe.matrix_projection);
			material.set_uniform(UniformName::MatrixNormal, groupe.matrix_normal);
			material.bind(&mut bind_context);
		}

		let geometry = &mut groupe.geometry.lock().unwrap();

		let len = groupe.buffer_group.count as GLint;
		let start = (groupe.buffer_group.start * geometry.get_vertex_byte_size()) as *const c_void;

		gl_call!({
			gl::DrawElements(gl::TRIANGLES, len, gl::UNSIGNED_INT, start);
		});
	}

	pub fn bind_frame_buffer(&mut self) {
		if self.current_frame_buffer == self.frame_buffer {
			return;
		}

		if let Some(frame_buffer) = &mut self.frame_buffer {
			let mut buffer = frame_buffer.lock().unwrap();
			buffer.bind(&mut self._gl_frame_buffer_ids, &mut self._gl_texture_ids, &mut self._gl_render_buffer_ids);
		} else {
			FrameBuffer::bind_default();
		}

		self.current_frame_buffer = self.frame_buffer.clone();
	}

	pub fn set_frame_buffer(&mut self, frame_buffer: Option<SharedFrameBuffer>) {
		self.frame_buffer = frame_buffer;
	}

	pub fn run<'a>(&mut self, camera: &Node, root: &Node) {
		root.update_transform(false);
		self.render(camera, root);
	}

	pub fn render<'a>(&mut self, camera: &Node, root: &Node) {
		Self::gl_clear_error();

		self.update_time();

		self.bind_frame_buffer();

		if let Some(color) = &self.clear_color {
			gl_call!({
				gl::ClearColor(color.x, color.y, color.z, color.w);
			});
			self.clear();
		}


		let mut matrix_cam_position;
		let matrix_projection;

		{
			let node_data = camera.lock();
			let cam_transform = &node_data.transform;
			matrix_cam_position = Matrix4::new();
			matrix_cam_position.get_inverse(&(cam_transform.matrix_world * cam_transform.matrix_local));

			match &node_data.camera {
				Some(camera) => {matrix_projection = camera.matrix_projection.clone()}
				None => {matrix_projection = Matrix4::new()}
			}
		}

		let mut light_materials_need_update = false;
		let mut lights_point: Vec<(Light, Vector3<f32>)> = vec![];
		let mut lights_direct: Vec<(Light, Vector3<f32>)> = vec![];
		let mut lights_affected_materials = HashMap::new();

		root.traverse(&mut |node, _| {
			let mut node_data = node.lock();
			let transform = &node_data.transform;

			if let Some(light) = &node_data.light {
				match light.light_type {
					LightType::Point => {
						let mut pos = transform.position.clone();
						pos.apply_matrix_4(&(matrix_cam_position * transform.matrix_world * transform.matrix_local));
						lights_point.push((light.clone(), pos));
					}
					LightType::Directional => {
						let mut direction = Vector3::new_down();
						let mut matrix_normal = Matrix3::new();
						matrix_normal.get_normal_matrix(&(matrix_cam_position * transform.matrix_world * transform.matrix_local));
						direction.apply_matrix_3(&matrix_normal);
						direction.normalize();
						lights_direct.push((light.clone(), direction));
					}
				}
			}

			node_data.materials
				.iter_mut()
				.for_each(|material| {
					let is_lighted = {material.lock().unwrap().has_tag(ShaderTag::Lighting)};
					if is_lighted {
						lights_affected_materials.insert(material.uuid(), material.clone());
					}
				})
		});

		if lights_point.len() != self.lights_point_count {
			self.lights_point_count = lights_point.len();
			light_materials_need_update = true;
		}

		if lights_direct.len() != self.lights_directional_count {
			self.lights_directional_count = lights_direct.len();
			light_materials_need_update = true;
		}

		lights_affected_materials
			.iter_mut()
			.for_each(|(_, material)| {
				let mut mat = material.lock().unwrap();

				lights_point.iter().enumerate().for_each(|(i, (light, pos))| {
					let mut col = light.color.clone();
					col.multiply_scalar(light.power);

					mat.set_uniform(UniformName::Other(format!("pointLights[{}].position", i)), pos.clone());
					mat.set_uniform(UniformName::Other(format!("pointLights[{}].color", i)), col);
					mat.set_uniform(UniformName::Other(format!("pointLights[{}].distance", i)), light.distance);
					mat.set_uniform(UniformName::Other(format!("pointLights[{}].decay", i)), light.decay);
				});

				lights_direct.iter().enumerate().for_each(|(i, (light, direction))| {
					let mut col = light.color.clone();
					col.multiply_scalar(light.power);

					mat.set_uniform(UniformName::Other(format!("directionalLights[{}].color", i)), col);
					mat.set_uniform(
						UniformName::Other(format!("directionalLights[{}].direction", i)),
						direction.clone(),
					);
				});

				if light_materials_need_update {
					mat.need_update();
				}
			});

		let mut render_queue = vec![];

		root.traverse(&mut |node, _| {
			let mut node_data = node.lock();

			if node_data.materials.len() == 0 || node_data.geometry.is_none() {return}

			let matrix_model = {
				let transform = &node_data.transform;
				let mut matrix_model = matrix_cam_position * transform.matrix_world * transform.matrix_local;

				match transform.lock {
					TransformLock::Rotation => {
						let (pos, mut rot, scale) = matrix_model.decompose_to_new();
						rot.copy(&transform.quaternion);
						matrix_model.compose(&pos, &rot, &scale);
					}
					TransformLock::Scale => {
						let (pos, rot, mut scale) = matrix_model.decompose_to_new();
						let length = pos.length();
						scale.multiply_scalar(length);
						matrix_model.compose(&pos, &rot, &scale);
					}
					TransformLock::RotationScale => {
						let (pos, mut rot, mut scale) = matrix_model.decompose_to_new();
						let length = pos.length();
						scale.multiply_scalar(length);
						rot.copy(&transform.quaternion);
						matrix_model.compose(&pos, &rot, &scale);
					}
					TransformLock::None => {}
				}
				matrix_model
			};

			let mut matrix_normal = Matrix3::new();
			matrix_normal.get_normal_matrix(&(matrix_cam_position * node_data.transform.matrix_world * node_data.transform.matrix_local));

			let mut groups = {
				let geometry = node_data.geometry
					.as_mut()
					.unwrap()
					.lock()
					.unwrap();

				if geometry.groups.len() == 0 {vec![geometry.get_default_group()]} else {geometry.groups.clone()}
			};

			groups.drain(..).for_each(|buffer_group| {
				let material_index = node_data.materials.len().min(buffer_group.material_index);
				let mut material = node_data.materials[material_index].clone();

				let need_sorting = { self.blending && material.lock().unwrap().blending != Blending::None };

				let mut groupe = DrawGroup {
					buffer_group,
					matrix_model,
					matrix_projection,
					matrix_normal,
					material,
					geometry: node_data.geometry.as_ref().unwrap().clone(),
					distance: 0.0,
				};

				if need_sorting {
					let transform = &mut node_data.transform;

					let mut pos = Vector3::zero();
					(matrix_cam_position * transform.matrix_world * transform.matrix_local).get_position(&mut pos);
					groupe.distance = pos.z;

					render_queue.push(groupe);
					return;
				}

				self.draw_buffer_group(groupe);
			});

		});

		if render_queue.len() > 0 {
			render_queue.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
			render_queue.drain(..).for_each(|groupe| {
				self.draw_buffer_group(groupe);
			});
		}
	}
}
