// #![macro_escape]

extern crate gl;
extern crate glutin;
extern crate rand;
extern crate uuid;


#[macro_use] pub mod macros;
mod gl_geometry;
mod gl_material;
pub mod systems;
mod gl_texture;
mod gl_shader_program;

extern crate image;

pub use self::systems::*;
use self::systems::system_render::*;
use self::gl_geometry::GLGeometry;
use self::gl_material::GLMaterial;



