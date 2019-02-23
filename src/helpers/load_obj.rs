extern crate obj;


use core::{BufferGeometry, BufferType, BufferGroup};
use math::{Vector3, Vector2};
// use math::{Vector2, Vector3, Vector};
// use std::f32::consts::PI;
use self::obj::{Obj, SimplePolygon, IndexTuple};
use std::path::Path;

#[derive(Clone, Debug)]
struct TmpIndex {
	index_old: usize,
	index_new: usize,
	position: Vector3<f32>,
	normal: Option<Vector3<f32>>,
	uv: Option<Vector2<f32>>,
}


fn add_elem (
	index_tuple: &IndexTuple,
	data_map: &mut Vec<Option<TmpIndex>>,
	data_order: &mut Vec<usize>,
	indices: &mut Vec<i32>,
	position: &Vec<Vector3<f32>>,
	normal: &Vec<Vector3<f32>>,
	uv: &Vec<Vector2<f32>>,
) {
	if let Some(vertex) = &data_map[index_tuple.0] {
		indices.push(vertex.index_new as i32);
		return;
	}

	let n = if let Some(index) = index_tuple.2 { Some(normal[index].clone()) } else {None};
	let t = if let Some(index) = index_tuple.1 { Some(uv[index].clone()) } else {None};
	let tmp_index = TmpIndex {
		index_old: index_tuple.0,
		index_new: data_order.len(),
		position: position[index_tuple.0].clone(),
		normal: n,
		uv: t,
	};
	indices.push(tmp_index.index_new as i32);
	data_map[index_tuple.0] = Some(tmp_index);
	data_order.push(index_tuple.0);
}


#[allow(dead_code)]
pub fn load_obj () -> Result<Vec<BufferGeometry>, String>{

	match Obj::<SimplePolygon>::load(&Path::new("models/Predator.obj")) {
	// match Obj::<SimplePolygon>::load(&Path::new("models/Predator_OBJ.OBJ")) {
	// match Obj::<SimplePolygon>::load(&Path::new("models/untitled.obj")) {
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
				let mut data_map = vec![None; obj_position.len()];
				let mut data_order = Vec::with_capacity(obj_position.len());

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
								add_elem(&poly[0], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);
								add_elem(&poly[1], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);
								add_elem(&poly[2], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);

								add_elem(&poly[2], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);
								add_elem(&poly[3], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);
								add_elem(&poly[0], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);
							}
							3 => {
								add_elem(&poly[0], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);
								add_elem(&poly[1], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);
								add_elem(&poly[2], &mut data_map, &mut data_order, &mut indices, &obj_position, &obj_normal, &obj_uv);
							}
							_ => {}
						}
					}

					buffer_group.count = indices.len() - buffer_group.start;
					geom.groups.push(buffer_group);
				}

				let positions = data_order.iter().map(|i| data_map[*i].as_ref().unwrap().position.clone() ).collect();
				geom.create_buffer_attribute("positions".to_string(), BufferType::Vector3(positions));

				let elem = data_map[data_order[0]].as_ref().unwrap();
				
				if elem.normal.is_some() {
					let normal = data_order.iter().map(|i| data_map[*i].as_ref().unwrap().normal.as_ref().unwrap().clone() ).collect();
					geom.create_buffer_attribute("normal".to_string(), BufferType::Vector3(normal));
				}

				if elem.uv.is_some() {
					let normal = data_order.iter().map(|i| data_map[*i].as_ref().unwrap().uv.as_ref().unwrap().clone() ).collect();
					geom.create_buffer_attribute("uv".to_string(), BufferType::Vector2(normal));
				}

				geom.set_indices(indices);
				result.push(geom);
			}
			Ok(result)
		}

		Err(err) => {Err(format!("{:?}", err))}
	}
}
