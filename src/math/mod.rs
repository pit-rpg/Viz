// pub use self::vector3;
// mod vector_3;
#[allow(non_snake_case)]
// pub mod vector;
mod vector;
mod vector2;
mod vector3;
mod vector4;
mod matrix4;
mod quaternion;
mod euler;
// mod matrix3;
mod color;

// pub use self::vector::Vector;
// pub use self::vector2::Vector2;
// pub use self::vector3::Vector3;
pub use self::matrix4::*;
// pub use self::matrix3::Matrix3;
pub use self::color::Color;
pub use self::vector2::*;
pub use self::vector3::*;
pub use self::vector4::*;
pub use self::quaternion::*;
pub use self::vector::*;
pub use self::euler::*;

// pub use self::Vector3 as Vector32;

// pub use Vector::Vector as ;
// pub use self::vector_3::Vector3;
// use Vector3::Vector3 as vv;
