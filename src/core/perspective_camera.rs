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
	// view = null;
}


#[allow(dead_code)]
impl PerspectiveCamera {
	pub fn new() -> Self {
		Self {
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
		}
	}

	pub fn set_focal_length (&mut self, focalLength: f32 ) {
		// see http://www.bobatkins.com/photography/technical/field_of_view.html
		let v_extent_slope = 0.5 * self.get_film_height() / focalLength;
		self.fov = (180.0/PI) * 2.0 * v_extent_slope.atan();
		self.update_projection_matrix();
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


	pub fn update_projection_matrix(&mut self) {
		let near = self.near;
		let top = near * ( (PI/180.0) * 0.5 * self.fov ).tan() / self.zoom;
		let height = 2.0 * top;
		let width = self.aspect * height;
		let mut left = - 0.5 * width;

		// let view = self.view;
		// if ( this.view !== null && this.view.enabled ) {

		// 	let fullWidth = view.fullWidth,
		// 		fullHeight = view.fullHeight;

		// 	left += view.offsetX * width / fullWidth;
		// 	top -= view.offsetY * height / fullHeight;
		// 	width *= view.width / fullWidth;
		// 	height *= view.height / fullHeight;

		// }

		let skew = self.film_offset;
		if  skew != 0.0  {
			left += near * skew / self.get_film_width();
		}

		self.matrix_projection.make_perspective( left, left + width, top, top - height, near, self.far );
		self.matrix_projection_inverse.get_inverse( &self.matrix_projection );
	}


	// setViewOffset: function ( fullWidth, fullHeight, x, y, width, height ) {
	// 	this.aspect = fullWidth / fullHeight;
	// 	if ( this.view === null ) {
	// 		this.view = {
	// 			enabled: true,
	// 			fullWidth: 1,
	// 			fullHeight: 1,
	// 			offsetX: 0,
	// 			offsetY: 0,
	// 			width: 1,
	// 			height: 1
	// 		};
	// 	}
	// 	this.view.enabled = true;
	// 	this.view.fullWidth = fullWidth;
	// 	this.view.fullHeight = fullHeight;
	// 	this.view.offsetX = x;
	// 	this.view.offsetY = y;
	// 	this.view.width = width;
	// 	this.view.height = height;
	// 	this.updateProjectionMatrix();
	// },
	// clearViewOffset: function () {
	// 	if ( this.view !== null ) {
	// 		this.view.enabled = false;
	// 	}
	// 	this.updateProjectionMatrix();
	// },

}