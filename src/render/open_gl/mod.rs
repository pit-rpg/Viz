// #![macro_escape]


extern crate gl;
extern crate glutin;
extern crate rand;
extern crate uuid;


#[macro_use]
pub mod macros;
mod gl_geometry;
mod gl_material;
mod gl_render;
mod gl_texture;

extern crate image;

use std::ffi::CStr;
use std::sync::{Arc, Mutex};


use self::glutin::GlContext;
use self::gl::GetString;
use math::Vector3;
use math::Vector2;
use math::Vector;

use self::gl_geometry::VertexArraysIDs;
use self::gl_geometry::GLGeometry;
use core::BufferType;
use core::BufferGeometry;
use core::Material;
use core::ProgramType;
use core::Texture;
use core::Transform;
use render::Renderer;
use self::gl_render::*;
use self::gl_texture::*;
use self::gl_material::GLMaterial;
use self::gl_material::GLMaterialIDs;
use helpers::sphere;


fn gl_clear_error() {
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
}

fn print_gl_version() {
	gl_call!({
		let version = GetString(gl::VERSION) as *const i8;
		println!("{:?}", CStr::from_ptr(version));
	});
}



extern crate specs;
use self::specs::{World, RunNow};



pub fn test()
// where T:Nums+'static
{
    let mut test_gl_render = GLRenderer::new();
    let mut f_count = 0.0;

    let mut color1 = Vector3::<f32>::random();
    let mut color2 = Vector3::<f32>::random();
    let mut color_tmp = Vector3::<f32>::new(color1.x, color1.y, color1.z);

    let mut running = true;

    let pos = vec![
            Vector3::<f32>::new(0.5,    0.5,    0.0),  // top right
            Vector3::<f32>::new(0.5,    -0.5,   0.0),  // bottom right
            Vector3::<f32>::new(-0.5,   -0.5,   0.0),  // bottom left
            Vector3::<f32>::new(-0.5,   0.5,    0.0)   // top left
    ];

    let uv = vec![
            Vector2::<f32>::new(1.0,    1.0),  // top right
            Vector2::<f32>::new(1.0,    0.0),  // bottom right
            Vector2::<f32>::new(0.0,    0.0),  // bottom left
            Vector2::<f32>::new(0.0,    1.0)   // top left
    ];

    let col = vec![
        Vector3::<f32>::new(1.0,    0.0,    0.0),  // top right
        Vector3::<f32>::new(0.0,    1.0,    0.0),  // bottom right
        Vector3::<f32>::new(1.0,    1.0,    1.0),  // bottom left
        Vector3::<f32>::new(0.0,    0.0,    1.0)  // top left
    ];

    let ind = vec![
            0, 1, 3,   // first triangle
            1, 2, 3    // second triangle
    ];

    let mut geom = BufferGeometry::new();
    geom.create_buffer_attribute("positions".to_string(), BufferType::Vector3(pos));
    geom.create_buffer_attribute("color".to_string(), BufferType::Vector3(col));
    geom.create_buffer_attribute("uv".to_string(), BufferType::Vector2(uv));
    geom.set_indices(ind);

    let mut geom2 = geom.duplicate();

    let mut geom3 = sphere(0.5, 12, 16);

    let mut transform1 = Transform::default();
    let transform2 = Transform::default();
    let transform3 = Transform::default();
    transform1.position.y -=0.2;
    transform1.position.x -=0.2;


    let texture1 = Texture::new("tile", "images/tile.jpg");
    let texture2 = Texture::new("AWESOME_FACE", "images/awesomeface.png");

    // let texture2 = Texture::new("AWESOME_FACE", "images/tile.jpg");
    // load_textures(&texture).expect("lolo");


    let mut material1 = Material::new_basic(&Vector3::new(1.0,0.0,0.0));
    let mut material3 = Material::new_normal();
    // material1.map_color = Some(Arc::new(Mutex::new(texture1)));
    // material1.map_color2 = Some(Arc::new(Mutex::new(texture2)));

    // let material1 = Materials::Basic( material1 );

    // let mut material2 = MeshNormalMaterial::new(Color::new(1.0, 0.0, 0.0));
    let mut material2 = Material::new_basic_texture(&Vector3::new(1.0,0.0,0.0));
    material2.set_texture("texture_color", Some(Arc::new(Mutex::new(texture2))), ProgramType::Fragment);
    // material2.map_color = Some(Arc::new(Mutex::new(texture2)));

    // let material2 = Materials::Normal( material2 );


    // let mut node = Node::<f32>::new();


    let mut world = World::new();
    world.register::<BufferGeometry>();
    world.register::<Material>();
    world.register::<Transform>();
    world.add_resource(VertexArraysIDs::new());
    world.add_resource(GLMaterialIDs::new());
    world.add_resource(GLTextureIDs::new());

    println!("{}", geom.uuid);
    println!("{}", geom2.uuid);

    // println!("{}", material.uuid);
    // println!("{}", material2.uuid);

    let e1 = world
        .create_entity()
        .with(geom)
        .with(material1)
        .with(transform1)
        .build();

    let e2 = world
        .create_entity()
        .with(geom2)
        .with(material2)
        .with(transform2)
        .build();

    // let e3 = world
    //     .create_entity()
    //     .with(geom3)
    //     .with(material3)
    //     .with(transform3)
    //     .build();


    let mut render_system = self::RenderSystem;

    // mesh.material.bind(&mut test_gl_render.gl_material_ids);
    // mesh.geometry.bind(&mut test_gl_render.vertex_arrays_ids);
    // let mesh = Mesh::new(geom, Box::from(material));

    // node.add_component(mesh);

    println!("{:?}", test_gl_render.gl_material_ids);
    println!("{:?}", test_gl_render.vertex_arrays_ids);

    while running {

        {
            let window = &test_gl_render.window;
            // let mut events_loop = &test_gl_render.events_loop;

            test_gl_render.events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent{ event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = window.get_hidpi_factor();
                            window.resize(logical_size.to_physical(dpi_factor));
                        },
                        // glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
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
            color2 = Vector3::random();
            f_count = 0.0;
        }


        color_tmp.copy(&color1);
        color_tmp.lerp(&color2, f_count);

        gl_call!({
            gl::ClearColor(color_tmp.x, color_tmp.y, color_tmp.z, 1.0);
        });

        // test_gl_render.render(&mut node);
        {
            let mut transform_store = world.write_storage::<Transform>();
            let transform = transform_store.get_mut(e2).unwrap();
            // transform.rotation.x += 0.1;
            // transform.rotation.y += 0.01;
            transform.rotation.z += 0.01;
            transform.position.x += 0.001;
            transform.position.y += 0.001;
            transform.scale.x -= 0.001;
            transform.scale.y -= 0.001;
            transform.scale.z -= 0.001;
            transform.update();
            // println!("{:?}", transform.matrix_local);
            // println!("{:?}", transform.scale);
        }



        test_gl_render.clear();

        render_system.run_now(&world.res);

        // mesh.material.bind(&mut test_gl_render.gl_material_ids);
        // mesh.geometry.bind(&mut test_gl_render.vertex_arrays_ids);
        // material.bind(&mut test_gl_render.gl_material_ids);
        // geom.bind(&mut test_gl_render.vertex_arrays_ids);

        gl_call!({

            // geom.bind(&mut test_gl_render.vertex_arrays_ids);
            // gl::BindVertexArray(VAO);
            // gl::UseProgram(shader_id);
            // gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

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