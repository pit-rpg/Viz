use core::{BufferGeometry, BufferData, BufferType};
use math::{Vector2, Vector3, Vector};
use std::f32::consts::PI;

#[allow(dead_code)]
pub fn param_sphere (radius: f32, width_segments: usize, height_segments: usize, phi_start: f32, phi_length: f32, theta_start: f32, theta_length: f32) -> BufferGeometry {

	let theta_end = theta_start + theta_length;

	let mut index = 0;
	let mut grid = Vec::new();

	let len = (width_segments * height_segments) as usize;

	// buffers
	let mut indices = Vec::with_capacity(len);
	let mut vertices = Vec::with_capacity(len);
	let mut normals = Vec::with_capacity(len);
	let mut uvs = Vec::with_capacity(len);

	// generate vertices, normals and uvs
	for iy in 0..=height_segments {

		let mut vertices_row = Vec::with_capacity(width_segments as usize);

		let v = iy as f32  / height_segments as f32;

		for ix in 0..=width_segments {

			let u = ix as f32 / width_segments as f32;

			let mut vertex = Vector3::new_zero();
			let mut normal = Vector3::new_zero();

			// vertex
			vertex.x = - radius * ( phi_start + u * phi_length ).cos() * ( theta_start + v * theta_length ).sin();
			vertex.y = radius * ( theta_start + v * theta_length ).cos();
			vertex.z = radius * ( phi_start + u * phi_length ).sin() * ( theta_start + v * theta_length ).sin();

			// normal
			normal.set( vertex.x, vertex.y, vertex.z ).normalize();

			vertices.push( vertex );
			normals.push( normal );

			// uv

			uvs.push(Vector2::new(u, 1.0 - v));

			vertices_row.push(index);
			index +=1;
		}

		grid.push( vertices_row );

	}

	// indices
	for iy in 0..(height_segments) {

		for ix in 0..(width_segments) {
			let iy1 = iy as usize;
			let ix1 = ix as usize;

			if iy != 0 || theta_start > 0.0 {
				let a = grid[ iy1 ][ ix1 + 1 ];
				let b = grid[ iy1 ][ ix1 ];
				let d = grid[ iy1 + 1 ][ ix1 + 1 ];
				indices.push( a );
				indices.push( b );
				indices.push( d );
			};

			if iy != height_segments - 1 || theta_end < PI {
				let b = grid[ iy1 ][ ix1 ];
				let c = grid[ iy1 + 1 ][ ix1 ];
				let d = grid[ iy1 + 1 ][ ix1 + 1 ];
				indices.push( b );
				indices.push( c );
				indices.push( d );
			};
		}
	}

	// build geometry
	let mut geom = BufferGeometry::new();
	geom.create_buffer_attribute(BufferType::Position, BufferData::Vector3(vertices));
	geom.create_buffer_attribute(BufferType::Normal, BufferData::Vector3(normals));
	geom.create_buffer_attribute(BufferType::UV(0), BufferData::Vector2(uvs));
	geom.set_indices(indices);
	geom
}


#[allow(dead_code)]
pub fn sphere (radius: f32, width_segments: usize, height_segments: usize) -> BufferGeometry {
	param_sphere(radius, width_segments, height_segments, 0.0, PI*2.0, 0.0, PI)
}


#[allow(dead_code)]
pub fn param_box(width: f32, height: f32, depth: f32, width_segments: usize, height_segments: usize, depth_segments: usize ) -> BufferGeometry {

	// buffers
	let mut indices = Vec::new();
	let mut vertices = Vec::new();
	let mut normals = Vec::new();
	let mut uvs = Vec::new();

	// helper variables
	let mut number_of_vertices = 0;
	let mut group_start = 0;

	{
		let mut build_plane = |u: char, v: char, w: char, udir: f32, vdir: f32, width: f32, height: f32, depth: f32, grid_x: usize, grid_y: usize | {

			let segment_width = width / grid_x as f32;
			let segment_height = height / grid_y as f32;

			let width_half = width / 2.0;
			let height_half = height / 2.0;
			let depth_half = depth / 2.0;

			let grid_x1 = grid_x + 1;
			let grid_y1 = grid_y + 1;

			let mut vertex_counter = 0;
			let mut group_count = 0;

			// let ix; let iy;

			// generate vertices, normals and uvs
			for iy in 0..grid_y1 {

				let y = iy as f32 * segment_height - height_half;

				for ix in 0..grid_x1 {

					let x = ix as f32 * segment_width - width_half;

					let mut vector = Vector3::new_zero();
					// set values to correct vector component
					vector[ u ] = x * udir;
					vector[ v ] = y * vdir;
					vector[ w ] = depth_half;

					// now apply vector to vertex buffer
					vertices.push( vector.clone() );

					// set values to correct vector component
					vector[ u ] = 0.0;
					vector[ v ] = 0.0;
					vector[ w ] = if depth > 0.0 {1.0} else {- 1.0};

					// now apply vector to normal buffer
					normals.push( vector );

					// uvs
					uvs.push( Vector2::new(ix as f32/grid_x as f32, 1.0-(iy as f32/grid_y as f32)) );

					// counters
					vertex_counter += 1;
				}

			}

			// indices

			// 1. you need three indices to draw a single face
			// 2. a single segment consists of two faces
			// 3. so we need to generate six (2*3) indices per segment

			for iy in 0..grid_y {
				for ix in 0..grid_x {

					let a = (number_of_vertices + ix + grid_x1 * iy) as u32;
					let b = (number_of_vertices + ix + grid_x1 * ( iy + 1 )) as u32;
					let c = (number_of_vertices + ( ix + 1 ) + grid_x1 * ( iy + 1 )) as u32;
					let d = (number_of_vertices + ( ix + 1 ) + grid_x1 * iy) as u32;

					// faces
					indices.push( a );
					indices.push( b );
					indices.push( d );

					indices.push( b );
					indices.push( c );
					indices.push( d );

					// increase counter
					group_count += 6;
				}
			}


			// add a group to the geometry. this will ensure multi material support
			// scope.addGroup( group_start, group_count, material_index );

			// calculate new start value for groups
			group_start += group_count;

			// update total number of vertices
			number_of_vertices += vertex_counter;
		};

		// build each side of the box geometry
		build_plane( 'z', 'y', 'x', - 1.0, - 1.0, depth, height, width, depth_segments, height_segments); // px
		build_plane( 'z', 'y', 'x', 1.0, - 1.0, depth, height, - width, depth_segments, height_segments); // nx
		build_plane( 'x', 'z', 'y', 1.0, 1.0, width, depth, height, width_segments, depth_segments); // py
		build_plane( 'x', 'z', 'y', 1.0, - 1.0, width, depth, - height, width_segments, depth_segments); // ny
		build_plane( 'x', 'y', 'z', 1.0, - 1.0, width, height, depth, width_segments, height_segments); // pz
		build_plane( 'x', 'y', 'z', - 1.0, - 1.0, width, height, - depth, width_segments, height_segments); // nz
	}

	let mut geom = BufferGeometry::new();
	geom.create_buffer_attribute(BufferType::Position, BufferData::Vector3(vertices));
	geom.create_buffer_attribute(BufferType::Normal, BufferData::Vector3(normals));
	geom.create_buffer_attribute(BufferType::UV(0), BufferData::Vector2(uvs));
	geom.set_indices(indices);
	geom
}


#[allow(dead_code)]
pub fn box_geometry(width: f32, height: f32, depth: f32 ) -> BufferGeometry {
	param_box(width, height, depth, 1, 1, 1 )
}


#[allow(dead_code)]
pub fn plane_buffer_geometry( width: f32, height: f32, grid_x: usize, grid_y: usize ) -> BufferGeometry {

	let width_half = width / 2.0;
	let height_half = height / 2.0;

	let grid_x1 = grid_x + 1;
	let grid_y1 = grid_y + 1;

	let segment_width = width / grid_x as f32;
	let segment_height = height / grid_y as f32;

	// buffers
	let mut indices = Vec::new();
	let mut vertices = Vec::new();
	let mut normals = Vec::new();
	let mut uvs = Vec::new();

	// generate vertices, normals and uvs
	for iy in 0..grid_y1  {

		let y = iy as f32 * segment_height - height_half;

		for ix in 0..grid_x1 {
			let ix = ix as f32;
			let iy = iy as f32;
			let grid_x = grid_x as f32;
			let grid_y = grid_y as f32;

			let x = ix * segment_width - width_half;

			vertices.push( Vector3::new(x, - y, 0.0) );
			normals.push( Vector3::new(0.0, 0.0, 1.0) );
			uvs.push( Vector2::new(ix / grid_x, 1.0 - ( iy / grid_y) ));
		}
	}

	// indices
	for iy in 0..grid_y {
		for ix in 0..grid_x {
			let a = ( ix + grid_x1 * iy ) as u32;
			let b = ( ix + grid_x1 * ( iy + 1 ) ) as u32;
			let c = ( ( ix + 1 ) + grid_x1 * ( iy + 1 ) ) as u32;
			let d = ( ( ix + 1 ) + grid_x1 * iy ) as u32;

			// faces
			indices.push( a );
			indices.push( b );
			indices.push( d );

			indices.push( b );
			indices.push( c );
			indices.push( d );
		}
	}


	// build geometry
	let mut geom = BufferGeometry::new();
	geom.create_buffer_attribute(BufferType::Position, BufferData::Vector3(vertices));
	geom.create_buffer_attribute(BufferType::Normal, BufferData::Vector3(normals));
	geom.create_buffer_attribute(BufferType::UV(0), BufferData::Vector2(uvs));
	geom.set_indices(indices);
	geom
}


#[allow(dead_code)]
pub fn simple_plane() -> BufferGeometry {

	// buffers
	let vertices = vec![
		Vector3::new(-1.0, -1.0, 0.0),
		Vector3::new(-1.0, 1.0, 0.0),
		Vector3::new(1.0, 1.0, 0.0),
		Vector3::new(1.0, -1.0, 0.0),
	];

	let indices = vec![
		0,1,2,
		0,2,3
	];

	// build geometry
	let mut geom = BufferGeometry::new();
	geom.create_buffer_attribute(BufferType::Position, BufferData::Vector3(vertices));
	geom.set_indices(indices);
	geom
}
