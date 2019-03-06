extern crate gltf;
extern crate specs;
use std::{
	fs,
	io
};
use std::boxed::Box;
use std::error::Error as StdError;
use math::{
	Matrix4
};


use self::gltf::{
	scene::{
		// Transform,
	}
};

use self::specs::{
	World,
};

use core::Transform;



pub fn load_gltf(world: &mut World) -> Result<(), Box<StdError>> {
	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");



	let path = "models/girl_speedsculpt/scene.gltf";

	let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    for scene in gltf.scenes() {
        print!("Scene {}", scene.index());
        // #[cfg(feature = "names")]
        print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
        println!();
        for node in scene.nodes() {
            load_node(world, &node, 1);
            // print_tree(&node, 1);
        }
    }

	println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++=");

	Ok(())
}



fn print_tree(node: &gltf::Node, depth: i32) {
    for _ in 0..(depth - 1) {
        print!("  ");
    }
    print!(" -");
    print!(" Node {}", node.index());
    // #[cfg(feature = "names")]
    print!(" ({})", node.name().unwrap_or("<Unnamed>"));

	let transform = node.transform().matrix();
	let matrix = Matrix4::from_column_row_array(transform);

    print!(" ({:?})", transform);
    println!();

    for child in node.children() {
        print_tree(&child, depth + 1);
    }
}


fn load_node(world: &mut World, node: &gltf::Node, depth: i32) {
    print!(" Node {}", node.index());
    print!(" ({})", node.name().unwrap_or("<dimensions {:?}>"));


	// Transform
	let matrix = Matrix4::from_column_row_array( node.transform().matrix() );
	let transform = Transform::from_matrix(matrix);
	// / Transform

	// Mesh
	let mesh = node.mesh().map(|mesh| {
		print!(" -> Mesh: {} {}", mesh.index(), mesh.name().unwrap_or("<Unnamed>"));

		let primitives: Vec<_> = mesh.primitives()
			.map(|primitive| {
				let indices = primitive.indices()
					.map(|accessor|{
						println!("name: {}, dimensions: {:?}, data_type: {:?}, count: {}, size: {}, offset:{}",
							accessor.name().unwrap_or("<Unnamed>"),
							accessor.dimensions(),
							accessor.data_type(),
							accessor.count(),
							accessor.size(),
							accessor.offset(),
						);
						let view = accessor.view();
						let buffer = view.buffer();

						println!("VIEW length: {} buffer length: {}", view.length(), buffer.length() );
					});
			})
			.collect();
		// for primitive in  {
		// 	println!("{:?}", primitive);
		// }
	});


	// /Mesh


    println!();

    for child in node.children() {
        load_node(world, &child, depth + 1);
    }
}

// fn run(path: &str) -> Result<(), Box<StdError>> {
//     let file = fs::File::open(&path)?;
//     let reader = io::BufReader::new(file);
//     let gltf = gltf::Gltf::from_reader(reader)?;
//     for scene in gltf.scenes() {
//         print!("Scene {}", scene.index());
//         #[cfg(feature = "names")]
//         print!(" ({})", scene.name().unwrap_or("<Unnamed>"));
//         println!();
//         for node in scene.nodes() {
//             print_tree(&node, 1);
//         }
//     }
//     Ok(())
// }

// fn main() {
//     if let Some(path) = std::env::args().nth(1) {
//         run(&path).expect("runtime error");
//     } else {
//         println!("usage: gltf-tree <FILE>");
//     }
// }