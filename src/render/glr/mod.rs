extern crate gl;
extern crate glutin;
extern crate rand;

use std::mem;
use std::fmt::Display;
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


const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) out vec4 FragColor;
    void main() {
        FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn gl_clear_error() {
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
}

fn gl_check_error<T: Display>(code: T) {
    let mut err;
    let mut count = 0;
    while { err = unsafe{ gl::GetError()}; err != gl::NO_ERROR } {
        println!("[OpenGL Error] ({})", err);
        count += 1;
    }

    if count > 0 {
        println!("{}", code);
        panic!();
    }
}

#[allow(unused_macros)]
macro_rules! gl_call {
    (  $x:block  ) => {
        unsafe {
            $x

            #[cfg(not(feature="prod"))]
            { gl_check_error(stringify!($x)); }
        }
    };
}


pub fn create_window() {

    let (mut events_loop, gl_window) = init_window();

    let shader_id = compile_shader_program(FRAGMENT_SHADER_SOURCE, VERTEX_SHADER_SOURCE);

    create_triangle();

    // let rng = rand::thread_rng();
    // let mut rng = rand::thread_rng();

    let positions: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
         0.5, -0.5, 0.0, // right
         0.0,  0.5, 0.0  // top
    ];

    let mut buf_id = 0;
    let mut va_buf_id = 0;
    let mut f_count = 0.0;


    let mut color1 = Color::<f32>::random();
    let mut color2 = Color::<f32>::random();
    let mut color_tmp = Color::new(color1.r,color1.g, color1.b);

    gl_call!({
        gl::GenBuffers(1, &mut buf_id);
        gl::GenVertexArrays(1, &mut va_buf_id);

        gl::BindVertexArray(va_buf_id);
        // gl::BindBuffer(gl::ARRAY_BUFFER, va_buf_id);

        gl::BindBuffer(gl::ARRAY_BUFFER, buf_id);

        gl::BufferData(
            gl::ARRAY_BUFFER,
            (mem::size_of::<GLfloat>() * positions.len()) as GLsizeiptr,
            &positions[0] as *const f32 as *const c_void,
            gl::DYNAMIC_DRAW
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);
    });

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

        if f_count > 1.0 {
            color1.copy(&color2);
            color2 = Color::random();
            f_count = 0.0;
        }

        f_count += 0.04;

        color_tmp.copy(&color1);
        color_tmp.lerp(&color2, f_count);

        gl_call!({
            gl::ClearColor(color_tmp.r, color_tmp.g, color_tmp.b, 1.0);
        });

        clear();

        gl_call!({
            gl::UseProgram(shader_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, buf_id);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
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
