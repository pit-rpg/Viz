// #![macro_escape]

extern crate gl;
extern crate glutin;
extern crate rand;
extern crate uuid;


#[macro_use] pub mod macros;
mod gl_geometry;
mod gl_material;
pub mod gl_render;
mod gl_texture;
mod gl_shaderProgram;

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
use core::SharedGeometry;
use core::Material;
use core::SharedMaterial;
use core::{Texture2D, SharedTexture2D, Uniform, ShaderProgram};
use core::PerspectiveCamera;
use core::Transform;
use core::create_world;
use render::Renderer;
use self::gl_render::*;
use self::gl_texture::*;
use self::gl_material::GLMaterial;
use self::gl_material::GLMaterialIDs;
use helpers::geometry_generators;
use std::f32::consts::PI;



extern crate specs;
use self::specs::{World, RunNow, Builder};
