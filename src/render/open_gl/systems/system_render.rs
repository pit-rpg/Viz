extern crate gl;
extern crate glutin;
extern crate rand;
extern crate rayon;
extern crate specs;
extern crate uuid;

use std::ffi::CStr;
use std::os::raw::c_void;
use std::time::{Duration, Instant};

use core::{
	Blending, BufferGeometry, BufferGroup, DirectionalLight, EntityRelations, FrameBuffer, Material, PerspectiveCamera,
	PointLight, ShaderTag, SharedFrameBuffer, SharedGeometry, SharedMaterials, Transform, TransformLock, UniformName,
};

use self::gl::types::*;
use self::gl::GetString;
use self::glutin::dpi::*;
use self::glutin::{ContextError, ContextWrapper, EventsLoop, Window};
use self::specs::prelude::*;

use super::super::{
	gl_frame_buffer::{GLFrameBuffer, GLFrameBufferIDs},
	gl_geometry::VertexArraysIDs,
	gl_material::GLMaterialIDs,
	gl_render_buffer::GLRenderBufferIDs,
	gl_texture::GLTextureIDs,
	GLGeometry, GLMaterial,
};
use math::{Matrix3, Matrix4, Vector, Vector3, Vector4};
use std::sync::{Arc, Mutex};

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
	material: Arc<Mutex<Material>>,
	geometry: SharedGeometry,
	time: f32,
	distance: f32,
}

pub struct RenderSystem {
	pub camera: Option<Entity>,
	pub windowed_context: ContextWrapper<glutin::PossiblyCurrent, Window>,
	pub events_loop: EventsLoop,
	pub timer: Instant,
	pub time: Duration,
	pub delta_time: Duration,
	pub delta_max: Option<Duration>,
	pub clear_color: Vector4<f32>,
	pub clear_color_need_update: bool,
	pub tags: Vec<ShaderTag>,
	pub override_material: Option<SharedMaterials>,

	lights_point_count: usize,
	lights_directional_count: usize,
	render_queue: Vec<DrawGroup>,

	depth_test: bool,
	stencil_test: bool,
	blending: bool,

	blending_state: Blending,

	frame_buffer: Option<SharedFrameBuffer>,
	current_frame_buffer: Option<SharedFrameBuffer>,
}

impl RenderSystem {
	pub fn new(world: &mut World, depth_test: bool, stencil_test: bool, blending: bool) -> Self {
		// TODO: ensure once
		world.insert(VertexArraysIDs::new());
		world.insert(GLMaterialIDs::new());
		world.insert(GLTextureIDs::new());
		world.insert(GLRenderBufferIDs::new());
		world.insert(GLFrameBufferIDs::new());
		// TODO: ensure once /

		let events_loop = glutin::EventsLoop::new();

		let window = glutin::WindowBuilder::new()
			.with_title("Hello, world!")
			.with_dimensions(LogicalSize::new(1024.0, 768.0));

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
		});
		// /Flags

		RenderSystem::print_gl_version();

		let render_system = Self {
			camera: None,
			// window: window,
			windowed_context: windowed_context,
			events_loop,
			timer: Instant::now(),
			time: Duration::new(0, 0),
			delta_time: Duration::new(0, 0),
			delta_max: None,
			clear_color: Vector4::new_zero(),
			clear_color_need_update: true,
			tags: Vec::new(),
			override_material: None,
			// render_settings: RenderSettings::default(),
			lights_point_count: 0,
			lights_directional_count: 0,
			render_queue: vec![],

			depth_test,
			stencil_test,
			blending,

			blending_state: Blending::None,

			frame_buffer: None,
			current_frame_buffer: None,
			// override_material: None,
		};
		render_system
	}

	pub fn clear(&self) {
		gl_call!({
			gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
		});
	}

	pub fn swap_buffers(&self) -> Result<(), ContextError> {
		self.windowed_context.swap_buffers()
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
		mut gl_material_ids: &'z mut GLMaterialIDs,
		mut gl_texture_ids: &'z mut GLTextureIDs,
		mut vertex_arrays_ids: &'z mut VertexArraysIDs,
	) {
		{
			let geometry = &groupe.geometry.lock().unwrap();
			geometry.bind(&mut vertex_arrays_ids);

			let material = &mut groupe.material.lock().unwrap();
			self.set_blending_mode(material.blending);

			let mut bind_context = BindContext {
				gl_texture_ids: &mut gl_texture_ids,
				gl_material_ids: &mut gl_material_ids,
				tags: &self.tags,
				lights_point_count: self.lights_point_count,
				lights_directional_count: self.lights_directional_count,
				geometry,
			};

			material.set_uniform(UniformName::MatrixModel, groupe.matrix_model);
			material.set_uniform(UniformName::MatrixView, groupe.matrix_projection);
			material.set_uniform(UniformName::MatrixNormal, groupe.matrix_normal);
			material.set_uniform(UniformName::Time, groupe.time);
			material.bind(&mut bind_context);
		}

		let geometry = &mut groupe.geometry.lock().unwrap();

		let len = groupe.buffer_group.count as GLint;
		let start = (groupe.buffer_group.start * geometry.get_vertex_byte_size()) as *const c_void;

		gl_call!({
			gl::DrawElements(gl::TRIANGLES, len, gl::UNSIGNED_INT, start);
		});
	}

	pub fn bind_frame_buffer(
		&mut self,
		texture_ids: &mut GLTextureIDs,
		frame_buffer_ids: &mut GLFrameBufferIDs,
		render_buffer_ids: &mut GLRenderBufferIDs,
	) {
		if self.current_frame_buffer == self.frame_buffer {
			return;
		}

		if let Some(frame_buffer) = &mut self.frame_buffer {
			let mut buffer = frame_buffer.lock().unwrap();
			buffer.bind(frame_buffer_ids, texture_ids, render_buffer_ids);
		} else {
			FrameBuffer::bind_default();
		}

		self.current_frame_buffer = self.frame_buffer.clone();
	}

	pub fn set_frame_buffer(&mut self, frame_buffer: Option<SharedFrameBuffer>) {
		self.frame_buffer = frame_buffer;
	}

	pub fn run<'a>(&mut self, world: &'a mut World, root: Entity) {
		let entities = world.get_all_children_entities(root);
		self.render(world, &entities);
	}

	pub fn render<'a>(&mut self, world: &'a mut World, entities: &[Entity]) {
		Self::gl_clear_error();

		// let mut entities = world.get_all_children_entities(root);
		let (
			camera_coll,
			transform_coll,
			mut geometry_coll,
			mut material_coll,
			light_point_coll,
			light_direct_coll,
			mut vertex_arrays_ids,
			mut gl_material_ids,
			mut gl_texture_ids,
			mut gl_frame_buffer_ids,
			mut gl_render_buffer_ids,
		): (
			ReadStorage<PerspectiveCamera>,
			ReadStorage<Transform>,
			WriteStorage<SharedGeometry>,
			WriteStorage<SharedMaterials>,
			WriteStorage<PointLight>,
			WriteStorage<DirectionalLight>,
			Write<VertexArraysIDs>,
			Write<GLMaterialIDs>,
			Write<GLTextureIDs>,
			Write<GLFrameBufferIDs>,
			Write<GLRenderBufferIDs>,
		) = SystemData::fetch(&world);

		self.bind_frame_buffer(&mut gl_texture_ids, &mut gl_frame_buffer_ids, &mut gl_render_buffer_ids);

		if self.clear_color_need_update {
			gl_call!({
				gl::ClearColor(self.clear_color.x, self.clear_color.y, self.clear_color.z, self.clear_color.w);
			});
			self.clear_color_need_update = false;
		}
		self.clear();

		// Time
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

		let time = self.get_duration();

		let mut matrix_cam_position;
		let matrix_projection;

		match self.camera {
			None => {
				matrix_cam_position = Matrix4::new();
				matrix_projection = Matrix4::new();
			}
			Some(ref cam) => {
				let cam_transform = transform_coll.get(*cam).unwrap();
				let camera = camera_coll.get(*cam).unwrap();
				matrix_cam_position = Matrix4::new();
				matrix_cam_position.get_inverse(&(cam_transform.matrix_world * cam_transform.matrix_local));
				matrix_projection = camera.matrix_projection.clone();
			}
		}

		let mut light_materials_need_update = false;

		let lights_point: Vec<_> = (&light_point_coll, &transform_coll)
			.join()
			.map(|(light, transform)| {
				let mut pos = transform.position.clone();
				pos.apply_matrix_4(&(matrix_cam_position * transform.matrix_world * transform.matrix_local));
				(light, pos)
			})
			.collect();

		let lights_direct: Vec<_> = (&light_direct_coll, &transform_coll)
			.join()
			.map(|(light, transform)| {
				let mut direction = light.direction.clone();
				let mut matrix_normal = Matrix3::new();
				matrix_normal.get_normal_matrix(&(matrix_cam_position * transform.matrix_world * transform.matrix_local));
				direction.apply_matrix_3(&matrix_normal);
				direction.normalize();
				(light, direction)
			})
			.collect();

		if lights_point.len() != self.lights_point_count {
			self.lights_point_count = lights_point.len();
			light_materials_need_update = true;
		}

		if lights_direct.len() != self.lights_directional_count {
			self.lights_directional_count = lights_direct.len();
			light_materials_need_update = true;
		}

		entities.iter().for_each(|entity| {
			// TODO: optimize
			let mut override_material = self.override_material.clone();
			let shared_materials = override_material.as_mut().or(material_coll.get_mut(*entity));
			let transform = transform_coll.get(*entity);
			let geometry = geometry_coll.get_mut(*entity);

			if transform.is_none() || geometry.is_none() || shared_materials.is_none() {
				return;
			}

			let transform = transform.unwrap();
			let geometry = geometry.unwrap();
			let shared_materials = shared_materials.unwrap();

			shared_materials.iter_mut().for_each(|shared_material| {
				let material = &mut shared_material.lock().unwrap();

				if !material.has_tag(ShaderTag::Lighting) {
					return;
				}

				lights_point.iter().enumerate().for_each(|(i, (light, pos))| {
					let mut col = light.color.clone();
					col.multiply_scalar(light.power);

					material.set_uniform(UniformName::Other(format!("pointLights[{}].position", i)), pos.clone());
					material.set_uniform(UniformName::Other(format!("pointLights[{}].color", i)), col);
					material.set_uniform(UniformName::Other(format!("pointLights[{}].distance", i)), light.distance);
					material.set_uniform(UniformName::Other(format!("pointLights[{}].decay", i)), light.decay);
				});
				lights_direct.iter().enumerate().for_each(|(i, (light, direction))| {
					let mut col = light.color.clone();
					col.multiply_scalar(light.power);

					material.set_uniform(UniformName::Other(format!("directionalLights[{}].color", i)), col);
					material.set_uniform(
						UniformName::Other(format!("directionalLights[{}].direction", i)),
						direction.clone(),
					);
				});

				if light_materials_need_update {
					material.need_update();
				}
			});

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

			let mut matrix_normal = Matrix3::new();
			matrix_normal.get_normal_matrix(&(matrix_cam_position * transform.matrix_world * transform.matrix_local));

			let mut groups = {
				let geom = geometry.lock().unwrap();
				if geom.groups.len() == 0 {
					vec![BufferGroup {
						count: geom.indices.len(),
						start: 0,
						material_index: 0,
						name: None,
					}]
				} else {
					geom.groups.clone()
				}
			};

			groups.drain(..).for_each(|buffer_group| {
				let material_index = shared_materials.len().min(buffer_group.material_index);
				let material = shared_materials.clone_material(material_index);

				let need_sorting = { self.blending && material.lock().unwrap().blending != Blending::None };

				let mut groupe = DrawGroup {
					buffer_group,
					matrix_model,
					matrix_projection,
					matrix_normal,
					material,
					geometry: geometry.clone(),
					time,
					distance: 0.0,
				};

				if need_sorting {
					let mut pos = Vector3::zero();
					(matrix_cam_position * transform.matrix_world * transform.matrix_local).get_position(&mut pos);
					groupe.distance = pos.z;

					self.render_queue.push(groupe);
					return;
				}

				self.draw_buffer_group(groupe, &mut gl_material_ids, &mut gl_texture_ids, &mut vertex_arrays_ids);
			});
		});

		if self.render_queue.len() > 0 {
			self.render_queue.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
			{
				let mut render_queue: Vec<DrawGroup> = self.render_queue.drain(..).collect();

				render_queue.drain(..).for_each(|groupe| {
					self.draw_buffer_group(groupe, &mut gl_material_ids, &mut gl_texture_ids, &mut vertex_arrays_ids);
				});
			}
		}

		self.swap_buffers().unwrap();
	}
}
