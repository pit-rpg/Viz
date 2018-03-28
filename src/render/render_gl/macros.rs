extern crate gl;

// use self::gl::GetString;
// use self::gl::types::*;
use std::fmt::Display;


pub fn gl_check_error<T: Display>(code: T) {
    let mut err;
    let mut count = 0;
    while { err = unsafe {gl::GetError()}; err != gl::NO_ERROR } {
        println!("[OpenGL Error] ({})", err);
        count += 1;
    }

    if count > 0 {
        println!("{}", code);
        panic!();
    }
}

#[macro_export]
macro_rules! gl_call {
    (  $x:block  ) => {
        unsafe {
            $x

            #[cfg(not(feature="prod"))]
            {
                use render::render_gl::macros;
                macros::gl_check_error(stringify!($x));
            }
        }
    };
}