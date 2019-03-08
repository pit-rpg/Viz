extern crate gltf;
extern crate specs;
extern crate byteorder;
extern crate regex;

use std::io::Cursor;
use self::byteorder::{LittleEndian, ReadBytesExt};

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::string::ToString;
use std::{
	fs,
	// io
};
use std::path::Path;
use std::path::PathBuf;
use std::boxed::Box;
use std::ops::Range;
use std::error::Error as StdError;
use math::{
	Vector,
	Vector2,
	Vector3,
	Vector4,
	Matrix2,
	Matrix3,
	Matrix4,
};


use self::gltf::{
	accessor::{
		Accessor,
		DataType,
		Dimensions,
	},
	mesh::{
		Semantic
	},
	buffer::{
		Source,
	},
	// material::{
	// 	Material,
	// },
	// image,
	image::Image,
};

use self::specs::{
	World,
	Builder,
};

use core::{
	Transform,
	BufferData,
	BufferAttribute,
	BufferGeometry,
	BufferType,
	Texture2D,
	Material,
	SharedMaterial,
	SharedGeometry,
};

struct Context {
	blob: Option<Vec<u8>>,
	path: PathBuf,
	uris: Vec<(String, Vec<u8>)>,
	images: Vec<Texture2D>,
	material: SharedMaterial,
}


pub fn load_gltf(world: &mut World, path: PathBuf) -> Result<(), Box<StdError>> {
	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");

	let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut gltf = gltf::Gltf::from_reader(reader)?;

	let mut context = Context{
		blob: gltf.blob.take(),
		path: path.clone(),
		uris: vec![],
		images: vec![],
		material: SharedMaterial::new(Material::new_normal()),
	};


	// preload uris
	gltf.buffers().for_each(|buffer| {
		if let Source::Uri(uri) = buffer.source() {
			let bin = read_file( &path.parent().unwrap().join(uri) );
			context.uris.push((uri.to_string(), bin));
		}
	});
	// preload uris

	// preload images
	context.images = gltf.images().map(|img| load_image(&img, &context) ).collect();
	// /preload images

    for scene in gltf.scenes() {
        print!("Scene {}", scene.index());
        // #[cfg(feature = "names")]
        print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
        println!();
        for node in scene.nodes() {
            load_node(world, &node, &context, 1);
            // print_tree(&node, 1);
        }
    }

	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");

	Ok(())
}


fn load_node(world: &mut World, node: &gltf::Node, context: &Context, depth: i32) {
    print!(" Node {}", node.index());
    print!(" ({})", node.name().unwrap_or("<dimensions {:?}>"));


	// Transform
	let matrix = Matrix4::from_column_row_array( node.transform().matrix() );
	let transform = Transform::from_matrix(matrix);
	// / Transform

	// Mesh
	let meshes = node.mesh().map(|mesh| {
		println!(" -> Mesh: {} {}", mesh.index(), mesh.name().unwrap_or("<Unnamed>"));

		let primitives: Vec<_> = mesh.primitives()
			.map(|primitive| {
				println!();
				let indices = primitive.indices()
					.map(|accessor|{
						println!("INDICES =================================");
						let data: Vec<u32> = match load_accessor(&accessor, context) {

							BufferData::U32(items) => { items }
							BufferData::I32(items) => { items.into_iter().map(|e| e as u32 ).collect() }
							BufferData::I16(items) => { items.into_iter().map(|e| e as u32 ).collect() }
							BufferData::U16(items) => { items.into_iter().map(|e| e as u32 ).collect() }
							BufferData::I8(items) => { items.into_iter().map(|e| e as u32 ).collect() }
							BufferData::U8(items) => { items.into_iter().map(|e| e as u32 ).collect() }

							_=> {panic!(format!("wrong indices: {}", context.path.to_str().unwrap()))}
						};
						data
					});

				let attributes: Vec<_> = primitive.attributes()
					.map(|(semantic, accessor)|{
						println!("ATTRIBUTES {:?} =================================", semantic);
						let data = load_accessor(&accessor, context);

						BufferAttribute {
							data,
							buffer_type: semantic.tp_buffer_type(),
							dynamic: false,
							normalized: accessor.normalized(),
						}
					})
					.collect();

				println!();

				let mut geom = BufferGeometry::new();
				attributes.into_iter().for_each(|e| {geom.add_buffer_attribute(e);} );
				indices.map(|data| {geom.set_indices(data)} );
				geom
			})
			.collect();
			primitives
	});
	// /Mesh

	if let Some(meshes) = meshes {
		println!("++++++++++++++++++++++++++++++++++++++++++");
		println!("++++++++++++++++++++++++++++++++++++++++++");
		meshes.iter().for_each(|mesh|{
			mesh.attributes.iter().for_each(|attr|{
				println!("NAME: {:?}", attr.buffer_type);
			});
		});
		println!("++++++++++++++++++++++++++++++++++++++++++");
		println!("++++++++++++++++++++++++++++++++++++++++++");


		for mesh in meshes {
			let e = world.create_entity()
				.with(Transform::default())
				.with(context.material.clone())
				.with(SharedGeometry::new(mesh))
				.build();
		}
	}

    // println!();

    for child in node.children() {
        load_node(world, &child, context, depth + 1);
    }
}



fn load_image(img: &Image, context: &Context) -> Texture2D {
	match img.source() {
		gltf::image::Source::Uri {uri, ..}  => {
			let img_path = context.path.parent().unwrap().join(uri);
			let bin = read_file(&img_path);

			Texture2D::new_from_bytes(Some(img_path.to_str().unwrap().to_string()), &bin)
		}
		gltf::image::Source::View {view, ..} => {
			let offset = view.offset();
			let length = view.length();
			let buffer = view.buffer();
			if let Some(_) = view.stride() { unimplemented!(); }
			if let Some(_) = view.target() { unimplemented!(); }
			let slice = get_buffer_slice(offset..offset+length, buffer, context);

			Texture2D::new_from_bytes(Some(view.name().unwrap_or("<Unnamed>").to_string()), slice)
		}
	}
}


// fn load_material(material: &Material, context: &Context) {

// }

fn read_file(path: &Path) -> Vec<u8> {
	let mut f = File::open(path).unwrap();
	let mut bin = Vec::new();
	f.read_to_end(&mut bin).unwrap();
	bin
}


fn get_buffer_slice<'a>(range: Range<usize>, buffer: gltf::Buffer, context: &'a Context) -> &'a [u8] {
	match buffer.source() {
		Source::Bin => {
			&context.blob
				.as_ref()
				.unwrap()
				[range]
		}
		Source::Uri(uri) => {
			&context.uris.iter()
				.find(|(uri_path, _)| uri_path == uri )
				.unwrap()
				.1[range]
		}
	}
}


fn load_accessor( accessor: &Accessor, context: &Context ) -> BufferData {
	let view = accessor.view();
	let buffer = view.buffer();

	println!("ACCESSOR: name: {}, dimensions: {:?}, data_type: {:?}, count: {}, size: {}, offset:{}, sparse: {}, normalized: {}, extras {:?}",
		accessor.name().unwrap_or("<Unnamed>"),
		accessor.dimensions().multiplicity(),
		accessor.data_type(),
		accessor.count(),
		accessor.size(),
		accessor.offset(),
		accessor.sparse().is_some(),
		accessor.normalized(),
		accessor.extras(),
	);

	println!("VIEW: name: {}, offset:{}, length:{}, stride:{:?}, target:{:?}, extras {:?}",
		view.name().unwrap_or("<Unnamed>"),
		view.offset(),
		view.length(),
		view.stride(),
		view.target(),
		view.extras(),
	);

	println!("BUFFER: name: {}, length:{}, extras {:?}",
		buffer.name().unwrap_or("<Unnamed>"),
		buffer.length(),
		buffer.extras(),
	);

	let offset = accessor.offset() + view.offset();
	let count = accessor.count();
	let data_type = accessor.data_type();
	let dimensions = accessor.dimensions();
	let multiplicity = dimensions.multiplicity();
	let size = accessor.size();
	// let slice = &context.blob[offset..offset+(count*size)];
	let slice = get_buffer_slice(offset..offset+(count*size), buffer, context);

	let mut rdr = Cursor::new(&slice);

	let mut data = match data_type {
		DataType::F32 => {
			let data: Vec<_> = (0..count*multiplicity).step_by(1)
				.map(|_|{ rdr.read_f32::<LittleEndian>().unwrap() })
				.collect();
			BufferData::F32(data)
		}
		DataType::U32 => {
			let data: Vec<_> = (0..count*multiplicity).step_by(1)
				.map(|_|{ rdr.read_u32::<LittleEndian>().unwrap() })
				.collect();
			BufferData::U32(data)
		}
		DataType::I16 => {
			let data: Vec<_> = (0..count*multiplicity).step_by(1)
				.map(|_|{ rdr.read_i16::<LittleEndian>().unwrap() })
				.collect();
			BufferData::I16(data)
		}
		DataType::U16 => {
			let data: Vec<_> = (0..count*multiplicity).step_by(1)
				.map(|_|{ rdr.read_u16::<LittleEndian>().unwrap() })
				.collect();
			BufferData::U16(data)
		}
		DataType::I8 => {
			let data: Vec<_> = (0..count*multiplicity).step_by(1)
				.map(|_|{ rdr.read_i8().unwrap() })
				.collect();
			BufferData::I8(data)
		}
		DataType::U8 => {
			let data: Vec<_> = (0..count*multiplicity).step_by(1)
				.map(|_|{ rdr.read_u8().unwrap() })
				.collect();
			BufferData::U8(data)
		}
	};

	{
		if let BufferData::F32(floats) = data {
			match dimensions {
				Dimensions::Vec2 => {
					let vectors: Vec<Vector2<f32>> = floats.chunks(2).map(|c|{
						Vector2::new_from_array(c)
					})
					.collect();
					data = BufferData::Vector2(vectors);
				}
				Dimensions::Vec3 => {
					let vectors: Vec<Vector3<f32>> = floats.chunks(3).map(|c|{
						Vector3::new_from_array(c)
					})
					.collect();
					data = BufferData::Vector3(vectors);
				}
				Dimensions::Vec4 => {
					let vectors: Vec<Vector4<f32>> = floats.chunks(4).map(|c|{
						Vector4::new_from_array(c)
					})
					.collect();
					data = BufferData::Vector4(vectors);
				}
				Dimensions::Mat2 => {
					let mats: Vec<Matrix2<f32>> = floats.chunks(4).map(|c|{
						Matrix2::from_array(c)
					})
					.collect();
					data = BufferData::Matrix2(mats);
				}
				Dimensions::Mat3 => {
					let mats: Vec<Matrix3<f32>> = floats.chunks(9).map(|c|{
						Matrix3::from_array(c)
					})
					.collect();
					data = BufferData::Matrix3(mats);
				}
				Dimensions::Mat4 => {
					let mats: Vec<Matrix4<f32>> = floats.chunks(16).map(|c|{
						Matrix4::from_array(c)
					})
					.collect();
					data = BufferData::Matrix4(mats);
				}
				_=>{ data = BufferData::F32(floats) }
			};
		};
	}

	data
}



trait SemanticToBufferType {
	fn tp_buffer_type(&self) -> BufferType;
}

impl SemanticToBufferType for Semantic {
	fn tp_buffer_type(&self) -> BufferType {
		match self {
			Semantic::Positions => BufferType::Position,
			Semantic::Normals => BufferType::Normal,
			Semantic::TexCoords(_) => BufferType::UV,
			Semantic::Tangents => BufferType::Tangent,
			Semantic::Colors(_) => BufferType::Color,
			Semantic::Joints(_) => BufferType::Joint,
			Semantic::Weights(_) => BufferType::Weight,
		}
	}
}