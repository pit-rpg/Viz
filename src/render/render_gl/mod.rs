// #![macro_escape]


extern crate gl;
extern crate glutin;
extern crate rand;

#[macro_use]
pub mod macros;
mod gl_geometry;
mod render;

use std::mem;

use std::ptr;
use std::str;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_void;

use self::glutin::GlContext;
use self::gl::GetString;
use self::gl::types::*;
// use self::rand::Rng;
use math::Color;
use math::ColorTrait;
use math::vector3::Vector3;
use math::vector3::Vector;

use core::BufferGeometry;
use self::gl_geometry::VartexArrays;
use self::gl_geometry::GLGeometry;
use core::BufferType;
// use core::BufferGroup;
// use core::BufferAttribute;


const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec3 aColor;
    out vec4 color;
    void main() {
        color = vec4(aColor.xyz, 1.0);
        gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) out vec4 FragColor;
    in vec4 color;
    // uniform vec4 u_Color;

    void main() {
        FragColor = color;
    }
"#;

pub fn init () {
    // self::gl_geometry::init();
}


fn gl_clear_error() {
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
}





pub fn create_window() {
    // self::gl_geometry::init();

    let (mut events_loop, gl_window) = init_window();

    let shader_id = compile_shader_program(FRAGMENT_SHADER_SOURCE, VERTEX_SHADER_SOURCE);

    // println!("{}", self::gl_geometry::a);

    create_triangle();

    // let rng = rand::thread_rng();
    // let mut rng = rand::thread_rng();

    let pos = vec![
            Vector3::<f32>::new_from(0.5,    0.5,    0.0),  // top right
            Vector3::<f32>::new_from(0.5,    -0.5,   0.0),  // bottom right
            Vector3::<f32>::new_from(-0.5,   -0.5,   0.0),  // bottom left
            Vector3::<f32>::new_from(-0.5,   0.5,    0.0)   // top left
    ];


    let col = vec![
        Color::new_from(1.0,    0.0,    0.0),  // top right
        Color::new_from(0.0,    1.0,    0.0),  // bottom right
        Color::new_from(1.0,    1.0,    1.0),  // bottom left
        Color::new_from(0.0,    0.0,    1.0)  // top left
    ];

    let ind = vec![
            0, 1, 3,   // first triangle
            1, 2, 3    // second triangle
    ];

    let mut geom = BufferGeometry::new();
    geom.create_buffer_attribute("positions".to_string(), BufferType::Vector3f32(pos), 3);
    geom.create_buffer_attribute("color".to_string(), BufferType::Colorf32(col), 3);
    geom.set_indices(ind);

    let mut hash_map = VartexArrays::new();

    let _positions: [f32; 24] = [
        0.5,    0.5,    0.0,        1.0,    0.0,    0.0,  // top right
        0.5,    -0.5,   0.0,        0.0,    1.0,    0.0,  // bottom right
        -0.5,   -0.5,   0.0,        1.0,    1.0,    1.0,  // bottom left
        -0.5,   0.5,    0.0,        0.0,    0.0,    1.0  // top left
    ];

    let _indices: [i32; 6] = [  // note that we start from 0!
        0, 1, 3,   // first triangle
        1, 2, 3    // second triangle
    ];

    // let mut VBO = 0;
    // let mut VAO = 0;
    // let mut EBO = 0;
    let mut f_count = 0.0;


    let mut color1 = Color::<f32>::random();
    let mut color2 = Color::<f32>::random();
    let mut color_tmp = Color::new_from(color1.r,color1.g, color1.b);

    // gl_call!({
    //     gl::GenVertexArrays(1, &mut VAO);
    //     gl::GenBuffers(1, &mut VBO);
    //     gl::GenBuffers(1, &mut EBO);

    //     gl::BindVertexArray(VAO);

    //     gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    //     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);

    //     gl::BufferData(
    //         gl::ARRAY_BUFFER,
    //         (mem::size_of::<GLfloat>() * positions.len()) as GLsizeiptr,
    //         &positions[0] as *const f32 as *const c_void,
    //         gl::DYNAMIC_DRAW
    //     );

    //     gl::BufferData(
    //         gl::ELEMENT_ARRAY_BUFFER,
    //         (mem::size_of::<GLint>() * indices.len()) as GLsizeiptr,
    //         &indices[0] as *const i32 as *const c_void,
    //         gl::STATIC_DRAW
    //     );

    println!("or_cap{}", mem::size_of::<GLfloat>() * _positions.len());
    println!("->VertexAttribPointer index:{}, vals:{}, val_type:{}, vertex_byte_len:{} byte_offset:{}", 0,3,gl::FLOAT, 6 * mem::size_of::<GLfloat>(), 0 );
    println!("->VertexAttribPointer index:{}, vals:{}, val_type:{}, vertex_byte_len:{} byte_offset:{}", 1,3,gl::FLOAT, 6 * mem::size_of::<GLfloat>(), 3 * mem::size_of::<GLfloat>() );


    //     gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<GLfloat>() as GLsizei, 0 as *const c_void);
    //     gl::EnableVertexAttribArray(0);
    //     gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 6 * mem::size_of::<GLfloat>() as GLsizei, (3 * mem::size_of::<GLfloat>()) as *const c_void );
    //     gl::EnableVertexAttribArray(1);
    //     // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    //     // gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    // });

    gl_call!({
        gl::UseProgram(shader_id);
    });

            // let u_color_uniform_id;
            // gl_call!({
            //     u_color_uniform_id = gl::GetUniformLocation(shader_id, CString::new("u_Color").unwrap().as_ptr());
            //     gl::Uniform4f(u_color_uniform_id, 0.2, 0.2, 0.2, 1.0);
            //     // u_color_uniform_id = gl::GetUniformLocation(shader_id, "u_Color" as *const &[gl::types::GLchar]);
            // });




    let mut running = true;
    while running {

        events_loop.poll_events(|event| {
            match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                    _ => ()
                },
                _ => ()
            }
        });

        gl_clear_error();

        f_count += 0.01;

        if f_count > 1.0 {
            color1.copy(&color2);
            color2 = Color::random();
            f_count = 0.0;
            // gl_call!({
            //     gl::Uniform4f(u_color_uniform_id, rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0);
            // });
        }


        color_tmp.copy(&color1);
        color_tmp.lerp(&color2, f_count);

        gl_call!({
            gl::ClearColor(color_tmp.r, color_tmp.g, color_tmp.b, 1.0);
        });

        clear();

        gl_call!({

            // gl::UseProgram(shader_id);
            // let u_color_uniform_id;
            // gl_call!({
            //     u_color_uniform_id = gl::GetUniformLocation(shader_id, CString::new("u_Color").unwrap().as_ptr());
            //     gl::Uniform4f(u_color_uniform_id, 0.2, 0.2, 0.2, 1.0);
            //     // u_color_uniform_id = gl::GetUniformLocation(shader_id, "u_Color" as *const &[gl::types::GLchar]);
            // });
            // gl::BindVertexArray(VBO);
            // // gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);

            geom.bind(&mut hash_map);
            // gl::BindVertexArray(VAO);
            gl::UseProgram(shader_id);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, 0 as *const c_void);

            // if f_count > 0.5 {
            //     gl::UseProgram(shader_id);
            //     gl::Uniform4f(u_color_uniform_id, rand::random::<f32>(), rand::random::<f32>(), rand::random::<f32>(), 1.0);
            //     gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, 3 as *const c_void);
            // }

            gl::BindVertexArray(0);
        });

        gl_window.swap_buffers().unwrap();
    }
}

fn create_triangle() {

}

// fn drow_triengle() {
//
// }

fn compile_shader(t: GLenum, src: &str) -> u32 {
    let id;

    gl_call!({
        id = gl::CreateShader(t);
        let c_str_frag = CString::new(src[..].as_bytes()).unwrap();

        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

        gl::ShaderSource(id, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(id);

        // check for shader compile errors
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            match t {
                gl::FRAGMENT_SHADER => println!("ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()),
                gl::VERTEX_SHADER => println!("ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()),
                _ => println!("ERROR::SHADER::?::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap()),
            };
            gl::DeleteShader(id);
            panic!();
        }
    });

    id
}

fn compile_shader_program(fs_source: &str, vs_source: &str) -> u32 {

    let id;

    gl_call!({
        id = gl::CreateProgram();

        let fs = compile_shader(gl::FRAGMENT_SHADER, fs_source);
        let vs = compile_shader(gl::VERTEX_SHADER, vs_source);

        gl::AttachShader(id, fs);
        gl::AttachShader(id, vs);

        gl::LinkProgram(id);
        gl::ValidateProgram(id);

        let mut info_log = Vec::with_capacity(512);
        info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
        // gl::GetShaderInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
        // println!("{}", str::from_utf8(&info_log).unwrap());
        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(id, 512, ptr::null_mut(), info_log.as_mut_ptr() as *mut GLchar);
            println!("ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}", str::from_utf8(&info_log).unwrap());
        }

        // TODO - releace remove shasers
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
    });

    id
}

fn print_gl_version() {
    gl_call!({
        let version = GetString(gl::VERSION) as *const i8;
        println!("{:?}", CStr::from_ptr(version));
    });
}

fn init_window() -> (glutin::EventsLoop, glutin::GlWindow) {
    let events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(1024, 768);

    let context = glutin::ContextBuilder::new()
        .with_vsync(true);

    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    unsafe {
        gl_window.make_current().unwrap();
    }

    gl_call!({
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 0.2, 0.2, 1.0);
    });

    print_gl_version();

    (events_loop, gl_window)
}

fn clear() {
    gl_call!({
        gl::Clear(gl::COLOR_BUFFER_BIT);
    });
}
