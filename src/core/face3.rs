use math::vector3::*;
use std::cmp::{ Eq, Ord, Ordering};
use std::ops::{Div,AddAssign,SubAssign,MulAssign, Mul, Add, DivAssign, Sub, Neg};
// use std::ops::{Div};
// use self::num_traits::Float;
use helpers::Nums;


#[derive(Clone, Debug)]
pub struct Face3<T>
where T:Nums<T>+MulAssign+AddAssign+SubAssign+Mul<Output=T>+Add<Output=T>+DivAssign+Sub<Output=T>+Neg<Output=T>+Clone+Div<Output=T>
{
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub normal: Vector3<T>,
    pub vertex_normals: [Vector3<T>; 3],
}

#[allow(dead_code)]
impl <T> Face3<T>
where T:Nums<T>+MulAssign+AddAssign+SubAssign+Mul<Output=T>+Add<Output=T>+DivAssign+Sub<Output=T>+Neg<Output=T>+Clone+Div<Output=T>+PartialOrd
{
    pub fn new (a:usize, b:usize, c:usize) -> Self {
        Face3 {
            a, b, c,
            normal: Vector3::new(),
            vertex_normals: [Vector3::new(),Vector3::new(),Vector3::new()],
        }
    }
}

// import { Color } from '../math/Color.js';
// import { Vector3 } from '../math/Vector3.js';
//
// /**
//  * @author mrdoob / http://mrdoob.com/
//  * @author alteredq / http://alteredqualia.com/
//  */
//
// function Face3( a, b, c, normal, color, materialIndex ) {
//
// 	this.a = a;
// 	this.b = b;
// 	this.c = c;
//
// 	this.normal = ( normal && normal.isVector3 ) ? normal : new Vector3();
// 	this.vertex_normals = Array.isArray( normal ) ? normal : [];
//
// 	this.color = ( color && color.isColor ) ? color : new Color();
// 	this.vertexColors = Array.isArray( color ) ? color : [];
//
// 	this.materialIndex = materialIndex !== undefined ? materialIndex : 0;
//
// }
//
// Object.assign( Face3.prototype, {
//
// 	clone: function () {
//
// 		return new this.constructor().copy( this );
//
// 	},
//
// 	copy: function ( source ) {
//
// 		this.a = source.a;
// 		this.b = source.b;
// 		this.c = source.c;
//
// 		this.normal.copy( source.normal );
// 		this.color.copy( source.color );
//
// 		this.materialIndex = source.materialIndex;
//
// 		for ( var i = 0, il = source.vertex_normals.length; i < il; i ++ ) {
//
// 			this.vertex_normals[ i ] = source.vertex_normals[ i ].clone();
//
// 		}
//
// 		for ( var i = 0, il = source.vertexColors.length; i < il; i ++ ) {
//
// 			this.vertexColors[ i ] = source.vertexColors[ i ].clone();
//
// 		}
//
// 		return this;
//
// 	}
//
// } );
//
//
// export { Face3 };
