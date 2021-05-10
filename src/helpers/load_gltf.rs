extern crate gltf;
extern crate byteorder;
extern crate regex;

use std::string::ToString;
use std::path::PathBuf;
use std::boxed::Box;
use std::error::Error as StdError;
use math::{
	Vector,
	Vector2,
	Vector3,
	Vector4,
	Matrix4,
};

use self::gltf::{
	mesh::{
		Semantic,
		util::ReadTexCoords,
		util::ReadColors,
	},
	image,
	Document,
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
	ShaderDef,
	Blending,
	TextureData,
	TextureColorType,
	Wrapping,
	MagFilter,
	MinFilter,
	SharedTexture2D,
	UniformName,
	TextureDataSource,
	Node,
	NodeData,
};

struct Context {
	doc: Document,
	buffers: Vec<gltf::buffer::Data>,
	materials: Vec<SharedMaterial>,
	default_material: SharedMaterial,
}

pub fn load_gltf(path: PathBuf, name: &str) -> Result<Node, Box<dyn StdError>> {
	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");

	let (doc, buffers, images) = gltf::import(path.clone())?;

	let images: Vec<TextureData> = images
		.iter()
		.map(|e| TextureData::from(e) )
		.collect();

	let textures: Vec<SharedTexture2D> = doc.textures()
		.map(|e|{
			let index = e.source().index();
			let mut texture = Texture2D::from(e);
			texture.set_texture_data( Some(images[ index ].clone()) );
			SharedTexture2D::new(texture)
		})
		.collect();

	println!("<><><><><>==========++++++==========<><><><><>");
	let materials = doc.materials()
		.map(|in_mat| {
			let pbr = in_mat.pbr_metallic_roughness();
			let mut mat = Material::new_mesh_standard();

			let color_f = pbr.base_color_factor();
			let diffuse = Vector3::new_from_array(&color_f);
			let emissive = Vector3::new_from_array(&in_mat.emissive_factor());

			if color_f[3] != 0.0 {
				mat.blending = Blending::Mix;
				mat.add_definition(ShaderDef::Transparent, "".to_string());
			}

			mat.set_uniform(UniformName::Color, diffuse);
			mat.set_uniform(UniformName::Roughness, pbr.roughness_factor());
			mat.set_uniform(UniformName::Metalness, pbr.metallic_factor());
			mat.set_uniform(UniformName::Alpha, color_f[3]);
			mat.set_uniform(UniformName::Emissive, emissive);

			mat.name = in_mat.name().unwrap_or("gltf_material").to_string();

			if let Some(map) = pbr.base_color_texture() {
				let texture = textures[ map.texture().index() ].clone();
				mat.set_uniform(UniformName::MapColor, (Some(texture), map.tex_coord()));
			}

			if let Some(map) = in_mat.normal_texture() {
				let texture = textures[ map.texture().index() ].clone();
				mat.set_uniform(UniformName::MapNormal, (Some(texture), map.tex_coord()));
				mat.set_uniform(UniformName::NormalScale, map.scale());
			}

			if let Some(map) = in_mat.emissive_texture() {
				let texture = textures[ map.texture().index() ].clone();
				mat.set_uniform(UniformName::MapEmissive, (Some(texture), map.tex_coord()));
			}

			if let Some(map) = in_mat.occlusion_texture() {
				let texture = textures[ map.texture().index() ].clone();
				mat.set_uniform(UniformName::MapOcclusion, (Some(texture), map.tex_coord()));
			}

			mat.to_shared()
		})
		.collect();

	println!("<><><><><>==========++++++==========<><><><><>");

	let context = Context {
		default_material: Material::new_normal().to_shared(),
		materials,
		doc,
		buffers,
	};

	let root = NodeData::new(name).to_shared();

	for scene in context.doc.scenes() {
		print!("Scene {}", scene.index());
		print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
		println!();
		for node in scene.nodes() {
			load_node(&node, &context, 1, &root);
		}
	}

	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");

	Ok(root)
}

fn load_node(gltf_node: &gltf::Node, context: &Context, depth: i32, parent: &Node) {
	let mut current_node_data = NodeData::new(gltf_node.name().unwrap_or("gltf_node"));

	print!(" Node {}", gltf_node.index());
	print!(" ({})", current_node_data.name);
	println!();


	// Transform
	let matrix = Matrix4::from_column_row_array( gltf_node.transform().matrix() );
	current_node_data.transform = Transform::from_matrix(matrix);
	// / Transform

	let current_node = current_node_data.to_shared();
	parent.add_child(current_node.clone());

	if let Some(mesh) = gltf_node.mesh() {
		println!(" -> Mesh: {} {}", mesh.index(), mesh.name().unwrap_or("<Unnamed>"));

		mesh
			.primitives()
			.map(|primitive| {
				println!();

				let reader = primitive.reader(|buffer| Some(&context.buffers[buffer.index()]));

				let indices = reader
					.read_indices()
					.map(|read_indices| {
						read_indices.into_u32().collect::<Vec<_>>()
					});


				let attributes: Vec<_> = primitive.attributes()
					.map(|(semantic, accessor)|{
						println!("ATTRIBUTES {:?} =================================", semantic);

						let data = match semantic {
							Semantic::Positions => {
								let positions: Vec<_> = reader.read_positions()
									.expect("cant find positions")
									.map(|v| Vector3::new( v[0], v[1], v[2] ) )
									.collect();
								BufferData::Vector3(positions)
							}
							Semantic::Normals => {
								let normals: Vec<_> = reader.read_normals()
									.expect("cant find normals")
									.map(|v| Vector3::new( v[0], v[1], v[2] ) )
									.collect();
								BufferData::Vector3(normals)
							}
							Semantic::TexCoords(n) => {
								let en = reader.read_tex_coords(n).expect("cant find uv");

								let uv: Vec<_> = match en {
									ReadTexCoords::U8(iter)=>{
										iter.map(|e| Vector2::new(e[0] as f32, e[1] as f32) ).collect()
									}
									ReadTexCoords::U16(iter)=>{
										iter.map(|e| Vector2::new(e[0] as f32, e[1] as f32) ).collect()
									}
									ReadTexCoords::F32(iter)=>{
										iter.map(|e| Vector2::new(e[0], e[1]) ).collect()
									}
								};
								BufferData::Vector2(uv)
							}
							Semantic::Colors(n) => {
								let en = reader.read_colors(n).expect("cant find colors");

								match en {
									ReadColors::RgbU8(iter) => {
										let color: Vec<_> = iter.map(|e| Vector3::new(e[0] as f32, e[1] as f32, e[2] as f32) ).collect();
										BufferData::Vector3(color)
									},
									ReadColors::RgbU16(iter) => {
										let color: Vec<_> = iter.map(|e| Vector3::new(e[0] as f32, e[1] as f32, e[2] as f32) ).collect();
										BufferData::Vector3(color)
									},
									ReadColors::RgbF32(iter) => {
										let color: Vec<_> = iter.map(|e| Vector3::new( e[0], e[1], e[2]) ).collect();
										BufferData::Vector3(color)
									},
									ReadColors::RgbaU8(iter) => {
										let color: Vec<_> = iter.map(|e| Vector4::new( e[0] as f32, e[1] as f32, e[2] as f32, e[3] as f32 ) ).collect();
										BufferData::Vector4(color)
									},
									ReadColors::RgbaU16(iter) => {
										let color: Vec<_> = iter.map(|e| Vector4::new( e[0] as f32, e[1] as f32, e[2] as f32, e[3] as f32 ) ).collect();
										BufferData::Vector4(color)
									},
									ReadColors::RgbaF32(iter) => {
										let color: Vec<_> = iter.map(|e| Vector4::new( e[0], e[1], e[2], e[3] ) ).collect();
										BufferData::Vector4(color)
									},
								}
							}
							Semantic::Joints(_) => {unimplemented!()}
							Semantic::Tangents => {
								let tangents: Vec<_> = reader.read_tangents()
									.expect("cant find tangents")
									.map(|v| Vector3::new( v[0], v[1], v[2] ) )
									.collect();
								BufferData::Vector3(tangents)
							}
							Semantic::Weights(_) => {unimplemented!()}
						};

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
				indices.map(|data| {geom.set_indices(data)});

				let shard_mat = match primitive.material().index() {
					None => context.default_material.clone(),
					Some(index) => context.materials[index].clone(),
				};

				current_node.add_child(
					NodeData::new("gltf_sub_node")
						.set_material(shard_mat)
						.set_geometry(geom.to_shared())
						.to_shared()
				);
			})
			.collect()
	}

	for child in gltf_node.children() {
		load_node(&child, context, depth + 1, &current_node);
	}
}


trait SemanticToBufferType {
	fn tp_buffer_type(&self) -> BufferType;
}

impl SemanticToBufferType for Semantic {
	fn tp_buffer_type(&self) -> BufferType {
		match self {
			Semantic::Positions => BufferType::Position,
			Semantic::Normals => BufferType::Normal,
			Semantic::Tangents => BufferType::Tangent,
			Semantic::TexCoords(i) => BufferType::UV(*i as usize),
			Semantic::Colors(i) => BufferType::Color(*i as usize),
			Semantic::Joints(i) => BufferType::Joint(*i as usize),
			Semantic::Weights(i) => BufferType::Weight(*i as usize),
		}
	}
}

impl From<&image::Data> for TextureData {
	fn from(data: &image::Data) -> Self {
		let color_type = match data.format {
			image::Format::R8 => TextureColorType::R8,
			image::Format::R8G8 => TextureColorType::RG8,
			image::Format::R8G8B8 => TextureColorType::RGB8,
			image::Format::R8G8B8A8 => TextureColorType::RGBA8,
			_ => unimplemented!()
		};

		TextureData{
			width: data.width,
			height: data.height,
			color_type,
			data: TextureDataSource::Raw(data.pixels.clone()),
		}
	}
}

impl From<gltf::texture::WrappingMode> for Wrapping {
	fn from(data: gltf::texture::WrappingMode) -> Self {
		match data {
			gltf::texture::WrappingMode::ClampToEdge => Wrapping::ClampToEdge,
			gltf::texture::WrappingMode::MirroredRepeat => Wrapping::MirroredRepeat,
			gltf::texture::WrappingMode::Repeat => Wrapping::Repeat,
		}
	}
}

impl From<gltf::texture::MagFilter> for MagFilter {
	fn from(data: gltf::texture::MagFilter) -> Self {
		match data {
			gltf::texture::MagFilter::Linear => MagFilter::Linear,
			gltf::texture::MagFilter::Nearest => MagFilter::Nearest,
		}
	}
}

impl From<gltf::texture::MinFilter> for MinFilter {
	fn from(data: gltf::texture::MinFilter) -> Self {
		match data {
			gltf::texture::MinFilter::Nearest => MinFilter::Nearest,
			gltf::texture::MinFilter::NearestMipmapLinear => MinFilter::NearestMipmapLinear,
			gltf::texture::MinFilter::NearestMipmapNearest => MinFilter::NearestMipmapNearest,
			gltf::texture::MinFilter::Linear => MinFilter::Linear,
			gltf::texture::MinFilter::LinearMipmapLinear => MinFilter::LinearMipmapLinear,
			gltf::texture::MinFilter::LinearMipmapNearest => MinFilter::LinearMipmapNearest,
		}
	}
}

impl From<Option<gltf::texture::MinFilter>> for MinFilter {
	fn from(data: Option<gltf::texture::MinFilter>) -> Self {
		data.map_or(MinFilter::LinearMipmapLinear, |e| e.into())
	}
}

impl From<Option<gltf::texture::MagFilter>> for MagFilter {
	fn from(data: Option<gltf::texture::MagFilter>) -> Self {
		data.map_or(MagFilter::Linear, |e| e.into())
	}
}

impl From<gltf::Texture<'_>> for Texture2D {
	fn from(data: gltf::texture::Texture) -> Self {
		let sampler = data.sampler();

		let mut elem = Texture2D::default();
		elem.wrapping_x = Wrapping::from(sampler.wrap_s());
		elem.wrapping_y = Wrapping::from(sampler.wrap_t());
		elem.min_filter = sampler.min_filter().into();
		elem.mag_filter = sampler.mag_filter().into();
		elem.path = data.name().map(|e| e.to_string() );

		elem
	}
}