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

use std::sync::{Arc, Mutex};
use std::f64::consts::PI as PI_f64;


use self::glutin::GlContext;
use math::Vector4;
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
use core::PerspectiveCamera;
use core::Transform;
use render::Renderer;
use self::gl_render::*;
use self::gl_texture::*;
use self::gl_material::GLMaterial;
use self::gl_material::GLMaterialIDs;
use helpers::{sphere, box_geometry};
use std::f32::consts::PI;


#[derive(Copy, Clone, PartialEq, Debug, Default)]
struct WindowState {
    pointer_pos: (f64, f64),
    pointer_pressed: (bool, bool, bool),
    pointer_wheel: f32,
    window_size: (f64, f64),
}





extern crate specs;
use self::specs::{World, RunNow};



pub fn test()
// where T:Nums+'static
{
    let mut render_system = self::RenderSystem::default();

    gl_call!({
        gl::Enable(gl::DEPTH_TEST);
    });

    let mut f_count = 0.0;
    let up = Vector3::new(0.0, 1.0, 0.0);
    let center = Vector3::new_zero();
    let radius = 8.0;

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
            1, 2, 3 // second triangle
    ];

    let mut geom = BufferGeometry::new();
    geom.create_buffer_attribute("positions".to_string(), BufferType::Vector3(pos));
    geom.create_buffer_attribute("color".to_string(), BufferType::Vector3(col));
    geom.create_buffer_attribute("uv".to_string(), BufferType::Vector2(uv));
    geom.set_indices(ind);

    let geom2 = box_geometry(1.0,1.0,1.0);
    let geom_container = box_geometry(1.0,1.0,1.0);
    let geom_sphere = sphere(0.5, 32, 32);
    let geom_light = sphere(0.5, 12, 12);

    let camera = PerspectiveCamera::new();

    let mut transform1 = Transform::default();
    transform1.position.y -=0.2;
    transform1.position.x -=0.2;
    let transform2 = Transform::default();
    let transform_spare = Transform::default();

    let mut transform_camera = Transform::default();
    transform_camera.position.z = 6.0;
    transform_camera.update();

    let mut transform_light = Transform::default();
    transform_light.position.set(1.2, 1.0, 2.0);
    transform_light.scale.set(0.2, 0.2, 0.2);
    transform_light.update();

    let texture1 = Texture::new("tile", "images/tile.jpg");
    let texture2 = Texture::new("AWESOME_FACE", "images/awesomeface.png");
    let texture3 = Texture::new("AWESOME_FACE", "images/earth.jpg");
    let texture_container = Texture::new("CONTAINER_2", "images/container2.png");
    let texture_container_specular = Texture::new("CONTAINER_2", "images/container2_specular.png");
    let m_texture2 = Arc::new(Mutex::new(texture2));
    let m_texture3 = Arc::new(Mutex::new(texture3));
    let m_texture_container = Arc::new(Mutex::new(texture_container));
    let m_texture_container_specular = Arc::new(Mutex::new(texture_container_specular));


    let material1 = Material::new_basic(&Vector4::random());
    let mut material2 = Material::new_basic_texture(&Vector4::random());
    material2.set_texture("texture_color", Some(m_texture2.clone()), ProgramType::Fragment);

    // let material_sphere = Material::new_normal();
    // let material_sphere = Material::new_light(&Vector4::new(1.0,0.5,0.31,1.0), &Vector3::new_one(), &transform_light.position);
    let mut material_sphere = Material::new_light_texture(&Vector4::new(1.0,0.5,0.31,1.0), &Vector3::new_one(), &transform_light.position);
    material_sphere.set_texture("texture_color", Some(m_texture_container), ProgramType::Fragment);
    material_sphere.set_texture("texture_specular", Some(m_texture_container_specular), ProgramType::Fragment);
    let material_light = Material::new_basic(&Vector4::new(1.0,1.0,1.0,1.0));


    let mut world = World::new();
    world.register::<BufferGeometry>();
    world.register::<Material>();
    world.register::<Transform>();
    world.register::<PerspectiveCamera>();
    world.add_resource(VertexArraysIDs::new());
    world.add_resource(GLMaterialIDs::new());
    world.add_resource(GLTextureIDs::new());

    println!("{}", geom.uuid);
    println!("{}", geom2.uuid);

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

    let e3 = world
        .create_entity()
        .with(geom_container)
        // .with(geom_sphere)
        .with(material_sphere)
        .with(transform_spare)
        .build();

    let e_cam = world
        .create_entity()
        .with(transform_camera)
        .with(camera)
        .build();

    let e_Light = world
        .create_entity()
        .with(geom_light)
        .with(material_light)
        .with(transform_light)
        .build();


    render_system.camera = Some(e_cam);
    let hidpi_factor = render_system.window.get_hidpi_factor().round();
    let mut window_state = WindowState::default();

    while running {

        {
            let window = &render_system.window;
            // let mut events_loop = &test_gl_render.events_loop;
            use self::glutin::WindowEvent::*;

            render_system.events_loop.poll_events(|event| {
                match event {
                    glutin::Event::WindowEvent{ event, .. } => match event {
                        glutin::WindowEvent::CloseRequested => running = false,
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = window.get_hidpi_factor();
                            window.resize(logical_size.to_physical(dpi_factor));
                            println!("{:?}", logical_size);
                            window_state.window_size.0 = logical_size.width;
                            window_state.window_size.1 = logical_size.height;
                        },
                        CursorMoved { position: pos, .. } =>{
                            window_state.pointer_pos = pos
                                .to_physical(window.get_hidpi_factor())
                                .to_logical(hidpi_factor)
                                .into();
                            // println!("{:?}", mouse_state.pos);
                        }
                        // WindowEvent::Resized(data) => {
                        //     println!("{:?}", data);
                        //     // window.resize(w, h),
                        // }
                        _ => ()
                    },
                    _ => ()
                }
            });
        }

        f_count += 0.01;

        if f_count > 1.0 {
            color1.copy(&color2);
            color2 = Vector3::random();
            f_count = 0.0;
        }

        color_tmp.copy(&color1);
        color_tmp.lerp(&color2, f_count);

        // render_system.clear_color.from_vector3(&color_tmp, 1.0);
        // render_system.clear_color_need_update = true;

        {
            let mut transform_store = world.write_storage::<Transform>();
            {
                let transform = transform_store.get_mut(e1).unwrap();
                transform.rotation.z = PI/4.0;
                transform.position.x = -0.3;
                transform.position.y = 0.6;
                transform.scale.x = 0.4;
                transform.scale.y = 0.4;
                transform.scale.z = 0.4;
                transform.update();
            }
            {
                let transform = transform_store.get_mut(e2).unwrap();
                transform.rotation.y += 0.01;
                transform.rotation.z += 0.01;
                transform.position.x += 0.001;
                transform.position.y += 0.001;
                transform.position.z -= 0.01;
                transform.update();
            }
            {
                let transform_spare = transform_store.get_mut(e3).unwrap();
                transform_spare.rotation.y += 0.01;
                transform_spare.scale.y = 2.0 * render_system.get_duration().sin().abs();
                transform_spare.update();
            }
            {
                let transform_camera = transform_store.get_mut(e_cam).unwrap();
                let x_prog = window_state.pointer_pos.0 / window_state.window_size.0;
                let y_prog = window_state.pointer_pos.1 / window_state.window_size.1;
                transform_camera.position.z = ( (x_prog * PI_f64).sin() * radius ) as f32;
                transform_camera.position.x = (( x_prog * radius - radius/2.0) * 2.0) as f32;
                transform_camera.position.y = (( y_prog * radius - radius/2.0) * 2.0) as f32;
                // println!("{:?}", transform_camera.rotation);
                transform_camera.look_at(&center, &up);
                transform_camera.update();
            }
        }

        render_system.run_now(&world.res);

    }

}