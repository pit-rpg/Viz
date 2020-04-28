use math::{Matrix4};
use std::f32::consts::PI;


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PerspectiveCamera {
	pub fov: f32,
	pub zoom: f32,
	pub near: f32,
	pub far: f32,
	pub focus: f32,
	pub aspect: f32,
	pub film_gauge: f32,
	pub film_offset: f32,
	pub matrix_projection: Matrix4<f32>,
	pub matrix_projection_inverse: Matrix4<f32>,
	pub view: CameraView,
}


#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CameraView {
	pub enabled: bool,
	pub full_width: f32,
	pub full_height: f32,
	pub offset_x: f32,
	pub offset_y: f32,
	pub width: f32,
	pub height: f32,
}


impl Default for CameraView {
	fn default() -> Self {
		Self {
			enabled: true,
			full_width: 1.0,
			full_height: 1.0,
			offset_x: 0.0,
			offset_y: 0.0,
			width: 1.0,
			height: 1.0,
		}
	}
}


#[allow(dead_code)]
impl PerspectiveCamera {
	pub fn new() -> Self {
		let mut cam = Self {
			fov: 50.0,
			zoom: 1.0,
			near: 0.1,
			far: 2000.0,
			focus: 10.0,
			aspect: 1.0,

			film_gauge: 35.0,	// width of the film (default in millimeters)
			film_offset: 0.0,

			matrix_projection: Matrix4::new(),
			matrix_projection_inverse: Matrix4::new(),
			view: CameraView::default(),
		};
		cam.update_projection_matrix();
		cam
	}

	pub fn set_focal_length (&mut self, focal_length: f32 ) -> &mut Self {
		// see http://www.bobatkins.com/photography/technical/field_of_view.html
		let v_extent_slope = 0.5 * self.get_film_height() / focal_length;
		self.fov = (180.0/PI) * 2.0 * v_extent_slope.atan();
		self.update_projection_matrix()
	}

	// /**
	//  * Calculates the focal length from the current .fov and .film_gauge.
	//  */
	pub fn get_focal_length(&self) -> f32 {
		let v_extent_slope = ( (PI/180.0) * 0.5 * self.fov ).tan();
		return 0.5 * self.get_film_height() / v_extent_slope;
	}

	pub fn get_effective_fov(&self) -> f32 {
		return (180.0/PI) * 2.0 * ( ( (PI/180.0) * 0.5 * self.fov ).tan() / self.zoom ).atan();
	}

	pub fn get_film_width(&self) -> f32 {
		// film not completely covered in portrait format (aspect < 1)
		self.film_gauge * self.aspect.min(1.0)
	}


	pub fn get_film_height(&self) -> f32 {
		// film not completely covered in landscape format (aspect > 1)
		self.film_gauge / self.aspect.max(1.0)
	}


	pub fn update_projection_matrix(&mut self) -> &mut Self {
		let near = self.near;
		let mut top = near * ( (PI/180.0) * 0.5 * self.fov ).tan() / self.zoom;
		let mut height = 2.0 * top;
		let mut width = self.aspect * height;
		let mut left = - 0.5 * width;

		{
			let view = &mut self.view;
			if view.enabled {
				let full_width = view.full_width;
				let full_height = view.full_height;

				left += view.offset_x * width / full_width;
				top -= view.offset_y * height / full_height;
				width *= view.width / full_width;
				height *= view.height / full_height;
			}
		}

		let skew = self.film_offset;
		if  skew != 0.0  {
			left += near * skew / self.get_film_width();
		}

		self.matrix_projection.make_perspective( left, left + width, top, top - height, near, self.far );
		self.matrix_projection_inverse.get_inverse( &self.matrix_projection );
		self
	}


	pub fn set_view_offset (&mut self, full_width: f32, full_height: f32, x: f32, y: f32, width: f32, height: f32 ) -> &mut Self {
		self.aspect = full_width / full_height;
		{
			let view = &mut self.view;
			view.enabled = true;
			view.full_width = full_width;
			view.full_height = full_height;
			view.offset_x = x;
			view.offset_y = y;
			view.width = width;
			view.height = height;
		}
		self.update_projection_matrix()
	}

	pub fn clear_view_offset(&mut self) -> &mut Self {
		self.view.enabled = false;
		self.update_projection_matrix()
	}
}