// #![macro_escape]


extern crate gl;
extern crate glutin;
extern crate rand;

#[macro_use]
pub mod macros;
mod gl_geometry;
mod gl_material;
mod gl_render;



use std::mem;

// use std::ptr;
use std::str;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;

use self::glutin::GlContext;
use self::gl::GetString;
// use self::gl::types::*;
// use self::rand::Rng;
use math::Color;
use math::ColorTrait;
use math::Vector3;
use math::Vector;

use self::gl_geometry::VartexArraysIDs;
use self::gl_geometry::GLGeometry;
use core::BufferType;
use core::BufferGeometry;
// use core::Mesh;
use core::Material;
use core::MeshBasicMaterial;
use render::Renderer;
use self::gl_render::GLRenderer;
use self::gl_material::GLMaterial;
// use core::BufferGroup;
// use core::BufferAttribute;

fn gl_clear_error() {
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
}

fn print_gl_version() {
	gl_call!({
		let version = GetString(gl::VERSION) as *const i8;
		println!("{:?}", CStr::from_ptr(version));
	});
}



pub fn test() {
    let mut test_gl_render = GLRenderer::new();
    let mut f_count = 0.0;

    let mut color1 = Color::<f32>::random();
    let mut color2 = Color::<f32>::random();
    let mut color_tmp = Color::new(color1.r, color1.g, color1.b);

    let mut running = true;


    let pos = vec![
            Vector3::<f32>::new(0.5,    0.5,    0.0),  // top right
            Vector3::<f32>::new(0.5,    -0.5,   0.0),  // bottom right
            Vector3::<f32>::new(-0.5,   -0.5,   0.0),  // bottom left
            Vector3::<f32>::new(-0.5,   0.5,    0.0)   // top left
    ];
    let col = vec![
        Color::new(1.0,    0.0,    0.0),  // top right
        Color::new(0.0,    1.0,    0.0),  // bottom right
        Color::new(1.0,    1.0,    1.0),  // bottom left
        Color::new(0.0,    0.0,    1.0)  // top left
    ];
    let ind = vec![
            0, 1, 3,   // first triangle
            1, 2, 3    // second triangle
    ];
    let mut geom = BufferGeometry::new();
    geom.create_buffer_attribute("positions".to_string(), BufferType::Vector3f32(pos), 3);
    geom.create_buffer_attribute("color".to_string(), BufferType::Colorf32(col), 3);
    geom.set_indices(ind);

    let material = MeshBasicMaterial::new(Color::new(1.0, 0.0, 0.0));

    // let mesh = Mesh::new(&geom, &material);

    // mesh.material.bind(&mut test_gl_render.gl_material_ids);
    // mesh.geometry.bind(&mut test_gl_render.vartex_arrays_ids);

    println!("{:?}", test_gl_render.gl_material_ids);
    println!("{:?}", test_gl_render.vartex_arrays_ids);

    while running {

        {
            let window = &test_gl_render.window;
            // let mut events_loop = &test_gl_render.events_loop;

            test_gl_render.events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent{ event, .. } => match event {
                        glutin::WindowEvent::Closed => running = false,
                        glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
                        _ => ()
                    },
                    _ => ()
                }
            });
        }

        gl_clear_error();

        f_count += 0.01;

        if f_count > 1.0 {
            color1.copy(&color2);
            color2 = Color::random();
            f_count = 0.0;
        }


        color_tmp.copy(&color1);
        color_tmp.lerp(&color2, f_count);

        gl_call!({
            gl::ClearColor(color_tmp.r, color_tmp.g, color_tmp.b, 1.0);
        });

        test_gl_render.clear();

        // mesh.material.bind(&mut test_gl_render.gl_material_ids);
        // mesh.geometry.bind(&mut test_gl_render.vartex_arrays_ids);
        material.bind(&mut test_gl_render.gl_material_ids);
        geom.bind(&mut test_gl_render.vartex_arrays_ids);

        gl_call!({

            // geom.bind(&mut test_gl_render.vartex_arrays_ids);
            // gl::BindVertexArray(VAO);
            // gl::UseProgram(shader_id);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

            // gl::BindVertexArray(0);
        });

        test_gl_render.window.swap_buffers().unwrap();
    }

}


pub fn create_window() {

    // let (mut events_loop, gl_window) = init_window();

    // let shader_id = compile_shader_program(FRAGMENT_SHADER_SOURCE, VERTEX_SHADER_SOURCE);

    // create_triangle();

    // let pos = vec![
    //         Vector3::<f32>::new_from(0.5,    0.5,    0.0),  // top right
    //         Vector3::<f32>::new_from(0.5,    -0.5,   0.0),  // bottom right
    //         Vector3::<f32>::new_from(-0.5,   -0.5,   0.0),  // bottom left
    //         Vector3::<f32>::new_from(-0.5,   0.5,    0.0)   // top left
    // ];


    // let col = vec![
    //     Color::new_from(1.0,    0.0,    0.0),  // top right
    //     Color::new_from(0.0,    1.0,    0.0),  // bottom right
    //     Color::new_from(1.0,    1.0,    1.0),  // bottom left
    //     Color::new_from(0.0,    0.0,    1.0)  // top left
    // ];

    // let ind = vec![
    //         0, 1, 3,   // first triangle
    //         1, 2, 3    // second triangle
    // ];

    // let mut geom = BufferGeometry::new();
    // geom.create_buffer_attribute("positions".to_string(), BufferType::Vector3f32(pos), 3);
    // geom.create_buffer_attribute("color".to_string(), BufferType::Colorf32(col), 3);
    // geom.set_indices(ind);

    // let mut hash_map = VartexArrays::new();

    // let _positions: [f32; 24] = [
    //     0.5,    0.5,    0.0,        1.0,    0.0,    0.0,  // top right
    //     0.5,    -0.5,   0.0,        0.0,    1.0,    0.0,  // bottom right
    //     -0.5,   -0.5,   0.0,        1.0,    1.0,    1.0,  // bottom left
    //     -0.5,   0.5,    0.0,        0.0,    0.0,    1.0  // top left
    // ];

    // let _indices: [i32; 6] = [  // note that we start from 0!
    //     0, 1, 3,   // first triangle
    //     1, 2, 3    // second triangle
    // ];

    // let mut f_count = 0.0;


    // let mut color1 = Color::<f32>::random();
    // let mut color2 = Color::<f32>::random();
    // let mut color_tmp = Color::new_from(color1.r,color1.g, color1.b);


    // println!("or_cap{}", mem::size_of::<GLfloat>() * _positions.len());
    // println!("->VertexAttribPointer index:{}, vals:{}, val_type:{}, vertex_byte_len:{} byte_offset:{}", 0,3,gl::FLOAT, 6 * mem::size_of::<GLfloat>(), 0 );
    // println!("->VertexAttribPointer index:{}, vals:{}, val_type:{}, vertex_byte_len:{} byte_offset:{}", 1,3,gl::FLOAT, 6 * mem::size_of::<GLfloat>(), 3 * mem::size_of::<GLfloat>() );



    // gl_call!({
    //     gl::UseProgram(shader_id);
    // });


    // let mut running = true;
    // while running {

    //     events_loop.poll_events(|event| {
    //         match event {
    //             glutin::Event::WindowEvent{ event, .. } => match event {
    //                 glutin::WindowEvent::Closed => running = false,
    //                 glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
    //                 _ => ()
    //             },
    //             _ => ()
    //         }
    //     });

    //     gl_clear_error();

    //     f_count += 0.01;

    //     if f_count > 1.0 {
    //         color1.copy(&color2);
    //         color2 = Color::random();
    //         f_count = 0.0;
    //     }


    //     color_tmp.copy(&color1);
    //     color_tmp.lerp(&color2, f_count);

    //     gl_call!({
    //         gl::ClearColor(color_tmp.r, color_tmp.g, color_tmp.b, 1.0);
    //     });

    //     clear();

    //     gl_call!({

    //         geom.bind(&mut hash_map);
    //         // gl::BindVertexArray(VAO);
    //         gl::UseProgram(shader_id);
    //         gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

    //         gl::BindVertexArray(0);
    //     });

    //     gl_window.swap_buffers().unwrap();
    // }
}