#![macro_use]

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
                use render::open_gl::macros;
                macros::gl_check_error(stringify!($x));
            }
        }
    };
}


// Macro to get c strings from literals without runtime overhead
// Literal must not contain any interior nul bytes!
// macro_rules! c_str {
//     ($literal:expr) => {
//         CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
//     }
// }
