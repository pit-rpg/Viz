extern crate obj;

use core::{BufferGeometry, BufferData, BufferGroup, BufferType};
use math::{Vector3, Vector2};
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
	indices: &mut Vec<u32>,
	obj: &Obj<SimplePolygon>
) {
	if let Some(vertex) = &data_map[index_tuple.0] {
		indices.push(vertex.index_new as u32);
		return;
	}

	let pos = obj.position[index_tuple.0];
	let position = Vector3::new(pos[0], pos[1], pos[2]);

	let n = if let Some(index) = index_tuple.2 {
		let e = obj.normal[index];
		Some(Vector3::new(e[0], e[1], e[2]))
	} else {None};

	let t = if let Some(index) = index_tuple.1 {
		let e = obj.texture[index];
		Some(Vector2::new(e[0], e[1]))
	} else {None};

	let tmp_index = TmpIndex {
		index_old: index_tuple.0,
		index_new: data_order.len(),
		position: position,
		normal: n,
		uv: t,
	};

	indices.push(tmp_index.index_new as u32);
	data_map[index_tuple.0] = Some(tmp_index);
	data_order.push(index_tuple.0);
}


#[allow(dead_code)]
pub fn load_obj( path: &Path ) -> Result<Vec<BufferGeometry>, String>{
	match Obj::<SimplePolygon>::load(path) {
		Ok(obj_data) => {

			let mut result = Vec::new();

			for object in &obj_data.objects  {
				let mut geom = BufferGeometry::new();
				geom.name = object.name.clone();

				let length = obj_data.position.len();
				let mut indices = Vec::with_capacity(length*4);
				let mut data_map = vec![None; length];
				let mut data_order = Vec::with_capacity(length);

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
								add_elem(&poly[0], &mut data_map, &mut data_order, &mut indices, &obj_data);
								add_elem(&poly[1], &mut data_map, &mut data_order, &mut indices, &obj_data);
								add_elem(&poly[2], &mut data_map, &mut data_order, &mut indices, &obj_data);

								add_elem(&poly[2], &mut data_map, &mut data_order, &mut indices, &obj_data);
								add_elem(&poly[3], &mut data_map, &mut data_order, &mut indices, &obj_data);
								add_elem(&poly[0], &mut data_map, &mut data_order, &mut indices, &obj_data);
							}
							3 => {
								add_elem(&poly[0], &mut data_map, &mut data_order, &mut indices, &obj_data);
								add_elem(&poly[1], &mut data_map, &mut data_order, &mut indices, &obj_data);
								add_elem(&poly[2], &mut data_map, &mut data_order, &mut indices, &obj_data);
							}
							_ => {}
						}
					}

					buffer_group.count = indices.len() - buffer_group.start;
					geom.groups.push(buffer_group);
				}

				let positions = data_order.iter().map(|i| data_map[*i].as_ref().unwrap().position.clone() ).collect();
				geom.create_buffer_attribute(BufferType::Position, BufferData::Vector3(positions));

				let elem = data_map[data_order[0]].as_ref().unwrap();

				if elem.normal.is_some() {
					let normal = data_order.iter().map(|i| data_map[*i].as_ref().unwrap().normal.as_ref().unwrap().clone() ).collect();
					geom.create_buffer_attribute(BufferType::Normal, BufferData::Vector3(normal));
				}

				if elem.uv.is_some() {
					let normal = data_order.iter().map(|i| data_map[*i].as_ref().unwrap().uv.as_ref().unwrap().clone() ).collect();
					geom.create_buffer_attribute(BufferType::UV, BufferData::Vector2(normal));
				}

				geom.set_indices(indices);
				result.push(geom);
			}
			Ok(result)
		}

		Err(err) => {Err(format!("{:?}", err))}
	}
}
