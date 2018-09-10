use core::{BufferGeometry, BufferType};
use math::{Vector2, Vector3, Vector};
use std::f32::consts::PI;


pub fn param_sphere (radius: f32, widthSegments: i32, heightSegments: i32, phiStart: f32, phiLength: f32, thetaStart: f32, thetaLength: f32) -> BufferGeometry {

	let theta_end = thetaStart + thetaLength;

	let mut index = 0;
	let mut grid = Vec::new();

	let len = (widthSegments * heightSegments) as usize;

	// buffers
	let mut indices = Vec::<i32>::with_capacity(len);
	let mut vertices = Vec::with_capacity(len);
	let mut normals = Vec::with_capacity(len);
	let mut uvs = Vec::with_capacity(len);

	// generate vertices, normals and uvs
	for iy in 0..heightSegments {

		let mut vertices_row = Vec::with_capacity(widthSegments as usize);

		let v = iy as f32  / heightSegments as f32;

		for ix in 0..widthSegments {

			let u = ix as f32 / widthSegments as f32;

			let mut vertex = Vector3::new_zero();
			let mut normal = Vector3::new_zero();

			// vertex
			vertex.x = - radius * ( phiStart + u * phiLength ).cos() * ( thetaStart + v * thetaLength ).sin();
			vertex.y = radius * ( thetaStart + v * thetaLength ).cos();
			vertex.z = radius * ( phiStart + u * phiLength ).sin() * ( thetaStart + v * thetaLength ).sin();

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

	println!("{:?}", grid);
	println!("{:?}", grid.len());
	println!("{:?}", grid[0].len());

	// indices
	for iy in 0..(heightSegments-1) {

		for ix in 0..(widthSegments-1) {
			let iy1 = iy as usize;
			let ix1 = ix as usize;
			println!("{}, {}", iy1, ix1);

			if iy != 0 || thetaStart > 0.0 {
				let a = grid[ iy1 ][ ix1 + 1 ];
				let b = grid[ iy1 ][ ix1 ];
				let d = grid[ iy1 + 1 ][ ix1 + 1 ];
				indices.push( a );
				indices.push( b );
				indices.push( d );
			};

			if iy != heightSegments - 1 || theta_end < PI {
				let b = grid[ iy1 ][ ix1 ];
				let c = grid[ iy1 + 1 ][ ix1 ];
				let d = grid[ iy1 + 1 ][ ix1 + 1 ];
				indices.push( b );
				indices.push( c );
				indices.push( d );
			};
		}
	}

	println!("=======================================");
	println!("{:?}", vertices);

	// build geometry
	let mut geom = BufferGeometry::new();
	geom.create_buffer_attribute("positions".to_string(), BufferType::Vector3(vertices));
	geom.create_buffer_attribute("normal".to_string(), BufferType::Vector3(normals));
	geom.create_buffer_attribute("uv".to_string(), BufferType::Vector2(uvs));
	geom.set_indices(indices);
	geom
}

pub fn sphere (radius: f32, widthSegments: i32, heightSegments: i32) -> BufferGeometry {
	param_sphere(radius, widthSegments, heightSegments, 0.0, PI*2.0, 0.0, PI)
}