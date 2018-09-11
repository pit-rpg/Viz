

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PerspectiveCamera {
	pub fov: f32,
	pub zoom: f32,
	pub near: f32,
	pub far: f32,
	pub focus: f32,
	pub aspect: f32,
	// view = null;
}

impl PerspectiveCamera {
	pub fn new() -> Self {
		Self {
			fov: 50.0,
			zoom: 1.0,
			near: 0.1,
			far: 2000.0,
			focus: 10.0,
			aspect: 1.0,
		}
	}
}