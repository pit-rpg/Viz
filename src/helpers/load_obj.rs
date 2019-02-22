extern crate obj;


use core::{BufferGeometry, BufferType, BufferGroup};
use math::{Vector3, Vector2};
// use math::{Vector2, Vector3, Vector};
// use std::f32::consts::PI;
use self::obj::{Obj, SimplePolygon, IndexTuple};
use std::path::Path;


fn add_elem(
	index_tuple: &IndexTuple,
	data: &mut Vec<(usize, Vector3<f32>, Option<Vector3<f32>>, Option<Vector2<f32>>)>,
	position: &Vec<Vector3<f32>>,
	normal: &Vec<Vector3<f32>>,
	uv: &Vec<Vector2<f32>>,
) -> usize {
	
	if let Some(index) = data.iter().position(|e| e.0 == index_tuple.0) {
		index
	} else {
		let p = position[index_tuple.0].clone();
		let n = if let Some(index) = index_tuple.2 { Some(normal[index].clone()) } else {None};
		let t = if let Some(index) = index_tuple.1 { Some(uv[index].clone()) } else {None};
		
		data.push((
			index_tuple.0,
			p, n, t
		));
		data.len() - 1
	}
}


#[allow(dead_code)]
pub fn load_obj () -> Result<Vec<BufferGeometry>, String>{

	match Obj::<SimplePolygon>::load(&Path::new("models/Predator_OBJ.OBJ")) {
		Ok(obj_data) => {
			let obj_position: Vec<Vector3<f32>> = obj_data.position
				.iter()
				.map(|[x,y,z]| Vector3::new(*x,*y,*z))
				.collect();
			
			let obj_normal: Vec<Vector3<f32>> = obj_data.normal
				.iter()
				.map(|[x,y,z]| Vector3::new(*x,*y,*z))
				.collect();

			let obj_uv: Vec<Vector2<f32>> = obj_data.texture
				.iter()
				.map(|[x,y]| Vector2::new(*x,*y))
				.collect();

			let mut result = Vec::new();
			
			for object in &obj_data.objects  {
				let mut geom = BufferGeometry::new();
				geom.name = object.name.clone();

				let mut indices = Vec::with_capacity(obj_position.len()*4);
				let mut data = Vec::with_capacity(obj_position.len());

				println!("name: {}, groups: {}", object.name, object.groups.len());

				for group in &object.groups {
					println!("name: {}, index: {}, polys: {}", group.name, group.index, group.polys.len());
					let mut buffer_group = BufferGroup {
						name: group.name.clone(),
						count: 0,
						material_index: 0,
						start: indices.len(),
					};

					for poly in &group.polys {
						match poly.len() {
							4 => {
								indices.push( add_elem(&poly[0], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );
								indices.push( add_elem(&poly[1], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );
								indices.push( add_elem(&poly[2], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );

								indices.push( add_elem(&poly[2], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );
								indices.push( add_elem(&poly[3], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );
								indices.push( add_elem(&poly[0], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );
							}
							3 => {
								indices.push( add_elem(&poly[0], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );
								indices.push( add_elem(&poly[1], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );
								indices.push( add_elem(&poly[2], &mut data, &obj_position, &obj_normal, &obj_uv) as i32 );
							}
							_ => {}
						}
					}

					buffer_group.count = indices.len() - buffer_group.start;
					geom.groups.push(buffer_group);
				}

				let positions = data.iter().map(|e| e.1.clone()).collect();
				geom.create_buffer_attribute("positions".to_string(), BufferType::Vector3(positions));

				if data[0].2.is_some() {
					let normal = data.iter().map(|e| e.2.as_ref().unwrap().clone()).collect();
					geom.create_buffer_attribute("normal".to_string(), BufferType::Vector3(normal));
				}

				if data[0].3.is_some() {
					let uv = data.iter().map(|e| e.3.as_ref().unwrap().clone()).collect();
					geom.create_buffer_attribute("uv".to_string(), BufferType::Vector2(uv));
				}

				geom.set_indices(indices);
				result.push(geom);
			}
			Ok(result)
		}

		Err(err) => {Err(format!("{:?}", err))}
	}
}
