extern crate uuid;
use self::uuid::Uuid;
use math::vector3::Vector3;
use math::vector3::Vector;
// use math::Matrix4;
// use math::Matrix3;
use core::Face3;
use helpers::Nums;
// use std::cmp::{ Eq, Ord, Ordering};
use std::ops::{Div,AddAssign,SubAssign,MulAssign, Mul, Add, DivAssign, Sub, Neg};

extern crate specs;
use self::specs::{Component, VecStorage};

// 	this.name = '';
// 	this.type = 'Geometry';

// #[derive(Clone, Debug, Copy)]
#[derive(Clone, Debug)]
pub struct Geometry<T>
where T:Nums
{
    pub uuid: Uuid,
    pub name: String,
    pub vertices: Vec<Vector3<T>>,
    pub faces: Vec<Face3<T>>,

	pub elements_need_update: bool,
	pub vertices_need_update: bool,
	pub uvs_need_update: bool,
	pub normals_need_update: bool,
	pub colors_need_update: bool,
	pub line_distances_need_update: bool,
	pub groups_need_update: bool,
}

#[allow(dead_code)]
impl <T> Geometry<T>
where T:Nums
{
    pub fn new() -> Geometry<T> {
        Geometry {
            uuid: Uuid::new_v4(),
            name: "".to_string(),
            vertices: vec![],
            faces: vec![],
            elements_need_update:false,
            vertices_need_update:false,
            uvs_need_update:false,
            normals_need_update:false,
            colors_need_update:false,
            line_distances_need_update:false,
            groups_need_update:false,
        }
    }

    // 	translate: function () {
        // 		// translate geometry
        // 		var m1 = new Matrix4();
        // 		return function translate( x, y, z ) {
        // 			m1.makeTranslation( x, y, z );
        // 			this.apply_matrix( m1 );
        // 			return this;
        // 		};
        // 	}(),


	// fn translate(&mut self, x: T, y: T, z: T) -> &mut Self {
	// 	// translate geometry
	// 	let mut m1 = Matrix4::new();

    //     m1.make_translation( x, y, z );
	// 	self.apply_matrix( &m1 );

    //     self
	// }

	// fn apply_matrix (&mut self, matrix: &Matrix4<T> ) -> &mut Self {
	// 	let mut normal_matrix = Matrix3::new();
    //     normal_matrix.get_normal_matrix( matrix );

    //     for vertex in &mut self.vertices {
	// 		vertex.apply_matrix_4( &matrix );
	// 	}

    //     for face in &mut self.faces {

    //         face.normal.apply_matrix_3( &normal_matrix ).normalize();

    //         for normal in &mut face.vertex_normals {
	// 			normal.apply_matrix_3( &normal_matrix ).normalize();
	// 		}
	// 	}

    //     // if ( this.boundingBox !== null ) {
	// 	// 	this.computeBoundingBox();
	// 	// }

    //     // if ( this.boundingSphere !== null ) {
	// 	// 	this.computeBoundingSphere();
	// 	// }

    //     self.vertices_need_update = true;
	// 	self.normals_need_update = true;

    //     self
	// }

}


impl Component for Geometry {
    type Storage = VecStorage<Self>;
}




// import { EventDispatcher } from './EventDispatcher.js';
// import { Face3 } from './Face3.js';
// import { Matrix3 } from '../math/Matrix3.js';
// import { Sphere } from '../math/Sphere.js';
// import { Box3 } from '../math/Box3.js';
// import { Vector3 } from '../math/Vector3.js';
// import { Matrix4 } from '../math/Matrix4.js';
// import { Vector2 } from '../math/Vector2.js';
// import { Color } from '../math/Color.js';
// import { Object3D } from './Object3D.js';
// import { _Math } from '../math/Math.js';
// /**
//  * @author mrdoob / http://mrdoob.com/
//  * @author kile / http://kile.stravaganza.org/
//  * @author alteredq / http://alteredqualia.com/
//  * @author mikael emtinger / http://gomo.se/
//  * @author zz85 / http://www.lab4games.net/zz85/blog
//  * @author bhouston / http://clara.io
//  */
// var geometryId = 0; // Geometry uses even numbers as Id
// function Geometry() {
// 	Object.defineProperty( this, 'id', { value: geometryId += 2 } );
// 	this.uuid = _Math.generateUUID();
// 	this.name = '';
// 	this.type = 'Geometry';
// 	this.vertices = [];
// 	this.colors = [];
// 	this.faces = [];
// 	this.faceVertexUvs = [[]];
// 	this.morphTargets = [];
// 	this.morphNormals = [];
// 	this.skinWeights = [];
// 	this.skinIndices = [];
// 	this.lineDistances = [];
// 	this.boundingBox = null;
// 	this.boundingSphere = null;
// 	// update flags
// 	this.elementsNeedUpdate = false;
// 	this.vertices_need_update = false;
// 	this.uvsNeedUpdate = false;
// 	this.normals_need_update = false;
// 	this.colorsNeedUpdate = false;
// 	this.lineDistancesNeedUpdate = false;
// 	this.groupsNeedUpdate = false;
// }
// Geometry.prototype = Object.assign( Object.create( EventDispatcher.prototype ), {
// 	constructor: Geometry,
// 	isGeometry: true,

// 	rotateX: function () {
// 		// rotate geometry around world x-axis
// 		var m1 = new Matrix4();
// 		return function rotateX( angle ) {
// 			m1.makeRotationX( angle );
// 			this.apply_matrix( m1 );
// 			return this;
// 		};
// 	}(),
// 	rotateY: function () {
// 		// rotate geometry around world y-axis
// 		var m1 = new Matrix4();
// 		return function rotateY( angle ) {
// 			m1.makeRotationY( angle );
// 			this.apply_matrix( m1 );
// 			return this;
// 		};
// 	}(),
// 	rotateZ: function () {
// 		// rotate geometry around world z-axis
// 		var m1 = new Matrix4();
// 		return function rotateZ( angle ) {
// 			m1.makeRotationZ( angle );
// 			this.apply_matrix( m1 );
// 			return this;
// 		};
// 	}(),

// 	scale: function () {
// 		// scale geometry
// 		var m1 = new Matrix4();
// 		return function scale( x, y, z ) {
// 			m1.makeScale( x, y, z );
// 			this.apply_matrix( m1 );
// 			return this;
// 		};
// 	}(),
// 	lookAt: function () {
// 		var obj = new Object3D();
// 		return function lookAt( vector ) {
// 			obj.lookAt( vector );
// 			obj.updateMatrix();
// 			this.apply_matrix( obj.matrix );
// 		};
// 	}(),
// 	fromBufferGeometry: function ( geometry ) {
// 		var scope = this;
// 		var indices = geometry.index !== null ? geometry.index.array : undefined;
// 		var attributes = geometry.attributes;
// 		var positions = attributes.position.array;
// 		var normals = attributes.normal !== undefined ? attributes.normal.array : undefined;
// 		var colors = attributes.color !== undefined ? attributes.color.array : undefined;
// 		var uvs = attributes.uv !== undefined ? attributes.uv.array : undefined;
// 		var uvs2 = attributes.uv2 !== undefined ? attributes.uv2.array : undefined;
// 		if ( uvs2 !== undefined ) this.faceVertexUvs[ 1 ] = [];
// 		var tempNormals = [];
// 		var tempUVs = [];
// 		var tempUVs2 = [];
// 		for ( var i = 0, j = 0; i < positions.length; i += 3, j += 2 ) {
// 			scope.vertices.push( new Vector3( positions[ i ], positions[ i + 1 ], positions[ i + 2 ] ) );
// 			if ( normals !== undefined ) {
// 				tempNormals.push( new Vector3( normals[ i ], normals[ i + 1 ], normals[ i + 2 ] ) );
// 			}
// 			if ( colors !== undefined ) {
// 				scope.colors.push( new Color( colors[ i ], colors[ i + 1 ], colors[ i + 2 ] ) );
// 			}
// 			if ( uvs !== undefined ) {
// 				tempUVs.push( new Vector2( uvs[ j ], uvs[ j + 1 ] ) );
// 			}
// 			if ( uvs2 !== undefined ) {
// 				tempUVs2.push( new Vector2( uvs2[ j ], uvs2[ j + 1 ] ) );
// 			}
// 		}
// 		function addFace( a, b, c, materialIndex ) {
// 			var vertex_normals = normals !== undefined ? [ tempNormals[ a ].clone(), tempNormals[ b ].clone(), tempNormals[ c ].clone() ] : [];
// 			var vertexColors = colors !== undefined ? [ scope.colors[ a ].clone(), scope.colors[ b ].clone(), scope.colors[ c ].clone() ] : [];
// 			var face = new Face3( a, b, c, vertex_normals, vertexColors, materialIndex );
// 			scope.faces.push( face );
// 			if ( uvs !== undefined ) {
// 				scope.faceVertexUvs[ 0 ].push( [ tempUVs[ a ].clone(), tempUVs[ b ].clone(), tempUVs[ c ].clone() ] );
// 			}
// 			if ( uvs2 !== undefined ) {
// 				scope.faceVertexUvs[ 1 ].push( [ tempUVs2[ a ].clone(), tempUVs2[ b ].clone(), tempUVs2[ c ].clone() ] );
// 			}
// 		}
// 		var groups = geometry.groups;
// 		if ( groups.length > 0 ) {
// 			for ( var i = 0; i < groups.length; i ++ ) {
// 				var group = groups[ i ];
// 				var start = group.start;
// 				var count = group.count;
// 				for ( var j = start, jl = start + count; j < jl; j += 3 ) {
// 					if ( indices !== undefined ) {
// 						addFace( indices[ j ], indices[ j + 1 ], indices[ j + 2 ], group.materialIndex );
// 					} else {
// 						addFace( j, j + 1, j + 2, group.materialIndex );
// 					}
// 				}
// 			}
// 		} else {
// 			if ( indices !== undefined ) {
// 				for ( var i = 0; i < indices.length; i += 3 ) {
// 					addFace( indices[ i ], indices[ i + 1 ], indices[ i + 2 ] );
// 				}
// 			} else {
// 				for ( var i = 0; i < positions.length / 3; i += 3 ) {
// 					addFace( i, i + 1, i + 2 );
// 				}
// 			}
// 		}
// 		this.computeFaceNormals();
// 		if ( geometry.boundingBox !== null ) {
// 			this.boundingBox = geometry.boundingBox.clone();
// 		}
// 		if ( geometry.boundingSphere !== null ) {
// 			this.boundingSphere = geometry.boundingSphere.clone();
// 		}
// 		return this;
// 	},
// 	center: function () {
// 		this.computeBoundingBox();
// 		var offset = this.boundingBox.getCenter().negate();
// 		this.translate( offset.x, offset.y, offset.z );
// 		return offset;
// 	},
// 	normalize: function () {
// 		this.computeBoundingSphere();
// 		var center = this.boundingSphere.center;
// 		var radius = this.boundingSphere.radius;
// 		var s = radius === 0 ? 1 : 1.0 / radius;
// 		var matrix = new Matrix4();
// 		matrix.set(
// 			s, 0, 0, - s * center.x,
// 			0, s, 0, - s * center.y,
// 			0, 0, s, - s * center.z,
// 			0, 0, 0, 1
// 		);
// 		this.apply_matrix( matrix );
// 		return this;
// 	},
// 	computeFaceNormals: function () {
// 		var cb = new Vector3(), ab = new Vector3();
// 		for ( var f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 			var face = this.faces[ f ];
// 			var vA = this.vertices[ face.a ];
// 			var vB = this.vertices[ face.b ];
// 			var vC = this.vertices[ face.c ];
// 			cb.subVectors( vC, vB );
// 			ab.subVectors( vA, vB );
// 			cb.cross( ab );
// 			cb.normalize();
// 			face.normal.copy( cb );
// 		}
// 	},
// 	computeVertexNormals: function ( areaWeighted ) {
// 		if ( areaWeighted === undefined ) areaWeighted = true;
// 		var v, vl, f, fl, face, vertices;
// 		vertices = new Array( this.vertices.length );
// 		for ( v = 0, vl = this.vertices.length; v < vl; v ++ ) {
// 			vertices[ v ] = new Vector3();
// 		}
// 		if ( areaWeighted ) {
// 			// vertex normals weighted by triangle areas
// 			// http://www.iquilezles.org/www/articles/normals/normals.htm
// 			var vA, vB, vC;
// 			var cb = new Vector3(), ab = new Vector3();
// 			for ( f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 				face = this.faces[ f ];
// 				vA = this.vertices[ face.a ];
// 				vB = this.vertices[ face.b ];
// 				vC = this.vertices[ face.c ];
// 				cb.subVectors( vC, vB );
// 				ab.subVectors( vA, vB );
// 				cb.cross( ab );
// 				vertices[ face.a ].add( cb );
// 				vertices[ face.b ].add( cb );
// 				vertices[ face.c ].add( cb );
// 			}
// 		} else {
// 			this.computeFaceNormals();
// 			for ( f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 				face = this.faces[ f ];
// 				vertices[ face.a ].add( face.normal );
// 				vertices[ face.b ].add( face.normal );
// 				vertices[ face.c ].add( face.normal );
// 			}
// 		}
// 		for ( v = 0, vl = this.vertices.length; v < vl; v ++ ) {
// 			vertices[ v ].normalize();
// 		}
// 		for ( f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 			face = this.faces[ f ];
// 			var vertex_normals = face.vertex_normals;
// 			if ( vertex_normals.length === 3 ) {
// 				vertex_normals[ 0 ].copy( vertices[ face.a ] );
// 				vertex_normals[ 1 ].copy( vertices[ face.b ] );
// 				vertex_normals[ 2 ].copy( vertices[ face.c ] );
// 			} else {
// 				vertex_normals[ 0 ] = vertices[ face.a ].clone();
// 				vertex_normals[ 1 ] = vertices[ face.b ].clone();
// 				vertex_normals[ 2 ] = vertices[ face.c ].clone();
// 			}
// 		}
// 		if ( this.faces.length > 0 ) {
// 			this.normals_need_update = true;
// 		}
// 	},
// 	computeFlatVertexNormals: function () {
// 		var f, fl, face;
// 		this.computeFaceNormals();
// 		for ( f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 			face = this.faces[ f ];
// 			var vertex_normals = face.vertex_normals;
// 			if ( vertex_normals.length === 3 ) {
// 				vertex_normals[ 0 ].copy( face.normal );
// 				vertex_normals[ 1 ].copy( face.normal );
// 				vertex_normals[ 2 ].copy( face.normal );
// 			} else {
// 				vertex_normals[ 0 ] = face.normal.clone();
// 				vertex_normals[ 1 ] = face.normal.clone();
// 				vertex_normals[ 2 ] = face.normal.clone();
// 			}
// 		}
// 		if ( this.faces.length > 0 ) {
// 			this.normals_need_update = true;
// 		}
// 	},
// 	computeMorphNormals: function () {
// 		var i, il, f, fl, face;
// 		// save original normals
// 		// - create temp variables on first access
// 		//   otherwise just copy (for faster repeated calls)
// 		for ( f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 			face = this.faces[ f ];
// 			if ( ! face.__originalFaceNormal ) {
// 				face.__originalFaceNormal = face.normal.clone();
// 			} else {
// 				face.__originalFaceNormal.copy( face.normal );
// 			}
// 			if ( ! face.__originalVertexNormals ) face.__originalVertexNormals = [];
// 			for ( i = 0, il = face.vertex_normals.length; i < il; i ++ ) {
// 				if ( ! face.__originalVertexNormals[ i ] ) {
// 					face.__originalVertexNormals[ i ] = face.vertex_normals[ i ].clone();
// 				} else {
// 					face.__originalVertexNormals[ i ].copy( face.vertex_normals[ i ] );
// 				}
// 			}
// 		}
// 		// use temp geometry to compute face and vertex normals for each morph
// 		var tmpGeo = new Geometry();
// 		tmpGeo.faces = this.faces;
// 		for ( i = 0, il = this.morphTargets.length; i < il; i ++ ) {
// 			// create on first access
// 			if ( ! this.morphNormals[ i ] ) {
// 				this.morphNormals[ i ] = {};
// 				this.morphNormals[ i ].faceNormals = [];
// 				this.morphNormals[ i ].vertex_normals = [];
// 				var dstNormalsFace = this.morphNormals[ i ].faceNormals;
// 				var dstNormalsVertex = this.morphNormals[ i ].vertex_normals;
// 				var faceNormal, vertex_normals;
// 				for ( f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 					faceNormal = new Vector3();
// 					vertex_normals = { a: new Vector3(), b: new Vector3(), c: new Vector3() };
// 					dstNormalsFace.push( faceNormal );
// 					dstNormalsVertex.push( vertex_normals );
// 				}
// 			}
// 			var morphNormals = this.morphNormals[ i ];
// 			// set vertices to morph target
// 			tmpGeo.vertices = this.morphTargets[ i ].vertices;
// 			// compute morph normals
// 			tmpGeo.computeFaceNormals();
// 			tmpGeo.computeVertexNormals();
// 			// store morph normals
// 			var faceNormal, vertex_normals;
// 			for ( f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 				face = this.faces[ f ];
// 				faceNormal = morphNormals.faceNormals[ f ];
// 				vertex_normals = morphNormals.vertex_normals[ f ];
// 				faceNormal.copy( face.normal );
// 				vertex_normals.a.copy( face.vertex_normals[ 0 ] );
// 				vertex_normals.b.copy( face.vertex_normals[ 1 ] );
// 				vertex_normals.c.copy( face.vertex_normals[ 2 ] );
// 			}
// 		}
// 		// restore original normals
// 		for ( f = 0, fl = this.faces.length; f < fl; f ++ ) {
// 			face = this.faces[ f ];
// 			face.normal = face.__originalFaceNormal;
// 			face.vertex_normals = face.__originalVertexNormals;
// 		}
// 	},
// 	computeLineDistances: function () {
// 		var d = 0;
// 		var vertices = this.vertices;
// 		for ( var i = 0, il = vertices.length; i < il; i ++ ) {
// 			if ( i > 0 ) {
// 				d += vertices[ i ].distanceTo( vertices[ i - 1 ] );
// 			}
// 			this.lineDistances[ i ] = d;
// 		}
// 	},
// 	computeBoundingBox: function () {
// 		if ( this.boundingBox === null ) {
// 			this.boundingBox = new Box3();
// 		}
// 		this.boundingBox.setFromPoints( this.vertices );
// 	},
// 	computeBoundingSphere: function () {
// 		if ( this.boundingSphere === null ) {
// 			this.boundingSphere = new Sphere();
// 		}
// 		this.boundingSphere.setFromPoints( this.vertices );
// 	},
// 	merge: function ( geometry, matrix, materialIndexOffset ) {
// 		if ( ! ( geometry && geometry.isGeometry ) ) {
// 			console.error( 'THREE.Geometry.merge(): geometry not an instance of THREE.Geometry.', geometry );
// 			return;
// 		}
// 		var normal_matrix,
// 			vertexOffset = this.vertices.length,
// 			vertices1 = this.vertices,
// 			vertices2 = geometry.vertices,
// 			faces1 = this.faces,
// 			faces2 = geometry.faces,
// 			uvs1 = this.faceVertexUvs[ 0 ],
// 			uvs2 = geometry.faceVertexUvs[ 0 ],
// 			colors1 = this.colors,
// 			colors2 = geometry.colors;
// 		if ( materialIndexOffset === undefined ) materialIndexOffset = 0;
// 		if ( matrix !== undefined ) {
// 			normal_matrix = new Matrix3().get_normal_matrix( matrix );
// 		}
// 		// vertices
// 		for ( var i = 0, il = vertices2.length; i < il; i ++ ) {
// 			var vertex = vertices2[ i ];
// 			var vertexCopy = vertex.clone();
// 			if ( matrix !== undefined ) vertexCopy.apply_matrix4( matrix );
// 			vertices1.push( vertexCopy );
// 		}
// 		// colors
// 		for ( var i = 0, il = colors2.length; i < il; i ++ ) {
// 			colors1.push( colors2[ i ].clone() );
// 		}
// 		// faces
// 		for ( i = 0, il = faces2.length; i < il; i ++ ) {
// 			var face = faces2[ i ], faceCopy, normal, color,
// 				faceVertexNormals = face.vertex_normals,
// 				faceVertexColors = face.vertexColors;
// 			faceCopy = new Face3( face.a + vertexOffset, face.b + vertexOffset, face.c + vertexOffset );
// 			faceCopy.normal.copy( face.normal );
// 			if ( normal_matrix !== undefined ) {
// 				faceCopy.normal.apply_matrix_3( normal_matrix ).normalize();
// 			}
// 			for ( var j = 0, jl = faceVertexNormals.length; j < jl; j ++ ) {
// 				normal = faceVertexNormals[ j ].clone();
// 				if ( normal_matrix !== undefined ) {
// 					normal.apply_matrix_3( normal_matrix ).normalize();
// 				}
// 				faceCopy.vertex_normals.push( normal );
// 			}
// 			faceCopy.color.copy( face.color );
// 			for ( var j = 0, jl = faceVertexColors.length; j < jl; j ++ ) {
// 				color = faceVertexColors[ j ];
// 				faceCopy.vertexColors.push( color.clone() );
// 			}
// 			faceCopy.materialIndex = face.materialIndex + materialIndexOffset;
// 			faces1.push( faceCopy );
// 		}
// 		// uvs
// 		for ( i = 0, il = uvs2.length; i < il; i ++ ) {
// 			var uv = uvs2[ i ], uvCopy = [];
// 			if ( uv === undefined ) {
// 				continue;
// 			}
// 			for ( var j = 0, jl = uv.length; j < jl; j ++ ) {
// 				uvCopy.push( uv[ j ].clone() );
// 			}
// 			uvs1.push( uvCopy );
// 		}
// 	},
// 	mergeMesh: function ( mesh ) {
// 		if ( ! ( mesh && mesh.isMesh ) ) {
// 			console.error( 'THREE.Geometry.mergeMesh(): mesh not an instance of THREE.Mesh.', mesh );
// 			return;
// 		}
// 		mesh.matrixAutoUpdate && mesh.updateMatrix();
// 		this.merge( mesh.geometry, mesh.matrix );
// 	},
// 	/*
// 	 * Checks for duplicate vertices with hashmap.
// 	 * Duplicated vertices are removed
// 	 * and faces' vertices are updated.
// 	 */
// 	mergeVertices: function () {
// 		var verticesMap = {}; // Hashmap for looking up vertices by position coordinates (and making sure they are unique)
// 		var unique = [], changes = [];
// 		var v, key;
// 		var precisionPoints = 4; // number of decimal points, e.g. 4 for epsilon of 0.0001
// 		var precision = Math.pow( 10, precisionPoints );
// 		var i, il, face;
// 		var indices, j, jl;
// 		for ( i = 0, il = this.vertices.length; i < il; i ++ ) {
// 			v = this.vertices[ i ];
// 			key = Math.round( v.x * precision ) + '_' + Math.round( v.y * precision ) + '_' + Math.round( v.z * precision );
// 			if ( verticesMap[ key ] === undefined ) {
// 				verticesMap[ key ] = i;
// 				unique.push( this.vertices[ i ] );
// 				changes[ i ] = unique.length - 1;
// 			} else {
// 				//console.log('Duplicate vertex found. ', i, ' could be using ', verticesMap[key]);
// 				changes[ i ] = changes[ verticesMap[ key ] ];
// 			}
// 		}
// 		// if faces are completely degenerate after merging vertices, we
// 		// have to remove them from the geometry.
// 		var faceIndicesToRemove = [];
// 		for ( i = 0, il = this.faces.length; i < il; i ++ ) {
// 			face = this.faces[ i ];
// 			face.a = changes[ face.a ];
// 			face.b = changes[ face.b ];
// 			face.c = changes[ face.c ];
// 			indices = [ face.a, face.b, face.c ];
// 			// if any duplicate vertices are found in a Face3
// 			// we have to remove the face as nothing can be saved
// 			for ( var n = 0; n < 3; n ++ ) {
// 				if ( indices[ n ] === indices[ ( n + 1 ) % 3 ] ) {
// 					faceIndicesToRemove.push( i );
// 					break;
// 				}
// 			}
// 		}
// 		for ( i = faceIndicesToRemove.length - 1; i >= 0; i -- ) {
// 			var idx = faceIndicesToRemove[ i ];
// 			this.faces.splice( idx, 1 );
// 			for ( j = 0, jl = this.faceVertexUvs.length; j < jl; j ++ ) {
// 				this.faceVertexUvs[ j ].splice( idx, 1 );
// 			}
// 		}
// 		// Use unique set of vertices
// 		var diff = this.vertices.length - unique.length;
// 		this.vertices = unique;
// 		return diff;
// 	},
// 	setFromPoints: function ( points ) {
// 		this.vertices = [];
// 		for ( var i = 0, l = points.length; i < l; i ++ ) {
// 			var point = points[ i ];
// 			this.vertices.push( new Vector3( point.x, point.y, point.z || 0 ) );
// 		}
// 		return this;
// 	},
// 	sortFacesByMaterialIndex: function () {
// 		var faces = this.faces;
// 		var length = faces.length;
// 		// tag faces
// 		for ( var i = 0; i < length; i ++ ) {
// 			faces[ i ]._id = i;
// 		}
// 		// sort faces
// 		function materialIndexSort( a, b ) {
// 			return a.materialIndex - b.materialIndex;
// 		}
// 		faces.sort( materialIndexSort );
// 		// sort uvs
// 		var uvs1 = this.faceVertexUvs[ 0 ];
// 		var uvs2 = this.faceVertexUvs[ 1 ];
// 		var newUvs1, newUvs2;
// 		if ( uvs1 && uvs1.length === length ) newUvs1 = [];
// 		if ( uvs2 && uvs2.length === length ) newUvs2 = [];
// 		for ( var i = 0; i < length; i ++ ) {
// 			var id = faces[ i ]._id;
// 			if ( newUvs1 ) newUvs1.push( uvs1[ id ] );
// 			if ( newUvs2 ) newUvs2.push( uvs2[ id ] );
// 		}
// 		if ( newUvs1 ) this.faceVertexUvs[ 0 ] = newUvs1;
// 		if ( newUvs2 ) this.faceVertexUvs[ 1 ] = newUvs2;
// 	},
// 	toJSON: function () {
// 		var data = {
// 			metadata: {
// 				version: 4.5,
// 				type: 'Geometry',
// 				generator: 'Geometry.toJSON'
// 			}
// 		};
// 		// standard Geometry serialization
// 		data.uuid = this.uuid;
// 		data.type = this.type;
// 		if ( this.name !== '' ) data.name = this.name;
// 		if ( this.parameters !== undefined ) {
// 			var parameters = this.parameters;
// 			for ( var key in parameters ) {
// 				if ( parameters[ key ] !== undefined ) data[ key ] = parameters[ key ];
// 			}
// 			return data;
// 		}
// 		var vertices = [];
// 		for ( var i = 0; i < this.vertices.length; i ++ ) {
// 			var vertex = this.vertices[ i ];
// 			vertices.push( vertex.x, vertex.y, vertex.z );
// 		}
// 		var faces = [];
// 		var normals = [];
// 		var normalsHash = {};
// 		var colors = [];
// 		var colorsHash = {};
// 		var uvs = [];
// 		var uvsHash = {};
// 		for ( var i = 0; i < this.faces.length; i ++ ) {
// 			var face = this.faces[ i ];
// 			var hasMaterial = true;
// 			var hasFaceUv = false; // deprecated
// 			var hasFaceVertexUv = this.faceVertexUvs[ 0 ][ i ] !== undefined;
// 			var hasFaceNormal = face.normal.length() > 0;
// 			var hasFaceVertexNormal = face.vertex_normals.length > 0;
// 			var hasFaceColor = face.color.r !== 1 || face.color.g !== 1 || face.color.b !== 1;
// 			var hasFaceVertexColor = face.vertexColors.length > 0;
// 			var faceType = 0;
// 			faceType = setBit( faceType, 0, 0 ); // isQuad
// 			faceType = setBit( faceType, 1, hasMaterial );
// 			faceType = setBit( faceType, 2, hasFaceUv );
// 			faceType = setBit( faceType, 3, hasFaceVertexUv );
// 			faceType = setBit( faceType, 4, hasFaceNormal );
// 			faceType = setBit( faceType, 5, hasFaceVertexNormal );
// 			faceType = setBit( faceType, 6, hasFaceColor );
// 			faceType = setBit( faceType, 7, hasFaceVertexColor );
// 			faces.push( faceType );
// 			faces.push( face.a, face.b, face.c );
// 			faces.push( face.materialIndex );
// 			if ( hasFaceVertexUv ) {
// 				var faceVertexUvs = this.faceVertexUvs[ 0 ][ i ];
// 				faces.push(
// 					getUvIndex( faceVertexUvs[ 0 ] ),
// 					getUvIndex( faceVertexUvs[ 1 ] ),
// 					getUvIndex( faceVertexUvs[ 2 ] )
// 				);
// 			}
// 			if ( hasFaceNormal ) {
// 				faces.push( getNormalIndex( face.normal ) );
// 			}
// 			if ( hasFaceVertexNormal ) {
// 				var vertex_normals = face.vertex_normals;
// 				faces.push(
// 					getNormalIndex( vertex_normals[ 0 ] ),
// 					getNormalIndex( vertex_normals[ 1 ] ),
// 					getNormalIndex( vertex_normals[ 2 ] )
// 				);
// 			}
// 			if ( hasFaceColor ) {
// 				faces.push( getColorIndex( face.color ) );
// 			}
// 			if ( hasFaceVertexColor ) {
// 				var vertexColors = face.vertexColors;
// 				faces.push(
// 					getColorIndex( vertexColors[ 0 ] ),
// 					getColorIndex( vertexColors[ 1 ] ),
// 					getColorIndex( vertexColors[ 2 ] )
// 				);
// 			}
// 		}
// 		function setBit( value, position, enabled ) {
// 			return enabled ? value | ( 1 << position ) : value & ( ~ ( 1 << position ) );
// 		}
// 		function getNormalIndex( normal ) {
// 			var hash = normal.x.toString() + normal.y.toString() + normal.z.toString();
// 			if ( normalsHash[ hash ] !== undefined ) {
// 				return normalsHash[ hash ];
// 			}
// 			normalsHash[ hash ] = normals.length / 3;
// 			normals.push( normal.x, normal.y, normal.z );
// 			return normalsHash[ hash ];
// 		}
// 		function getColorIndex( color ) {
// 			var hash = color.r.toString() + color.g.toString() + color.b.toString();
// 			if ( colorsHash[ hash ] !== undefined ) {
// 				return colorsHash[ hash ];
// 			}
// 			colorsHash[ hash ] = colors.length;
// 			colors.push( color.getHex() );
// 			return colorsHash[ hash ];
// 		}
// 		function getUvIndex( uv ) {
// 			var hash = uv.x.toString() + uv.y.toString();
// 			if ( uvsHash[ hash ] !== undefined ) {
// 				return uvsHash[ hash ];
// 			}
// 			uvsHash[ hash ] = uvs.length / 2;
// 			uvs.push( uv.x, uv.y );
// 			return uvsHash[ hash ];
// 		}
// 		data.data = {};
// 		data.data.vertices = vertices;
// 		data.data.normals = normals;
// 		if ( colors.length > 0 ) data.data.colors = colors;
// 		if ( uvs.length > 0 ) data.data.uvs = [ uvs ]; // temporal backward compatibility
// 		data.data.faces = faces;
// 		return data;
// 	},
// 	clone: function () {
// 		/*
// 		 // Handle primitives
// 		 var parameters = this.parameters;
// 		 if ( parameters !== undefined ) {
// 		 var values = [];
// 		 for ( var key in parameters ) {
// 		 values.push( parameters[ key ] );
// 		 }
// 		 var geometry = Object.create( this.constructor.prototype );
// 		 this.constructor.apply( geometry, values );
// 		 return geometry;
// 		 }
// 		 return new this.constructor().copy( this );
// 		 */
// 		return new Geometry().copy( this );
// 	},
// 	copy: function ( source ) {
// 		var i, il, j, jl, k, kl;
// 		// reset
// 		this.vertices = [];
// 		this.colors = [];
// 		this.faces = [];
// 		this.faceVertexUvs = [[]];
// 		this.morphTargets = [];
// 		this.morphNormals = [];
// 		this.skinWeights = [];
// 		this.skinIndices = [];
// 		this.lineDistances = [];
// 		this.boundingBox = null;
// 		this.boundingSphere = null;
// 		// name
// 		this.name = source.name;
// 		// vertices
// 		var vertices = source.vertices;
// 		for ( i = 0, il = vertices.length; i < il; i ++ ) {
// 			this.vertices.push( vertices[ i ].clone() );
// 		}
// 		// colors
// 		var colors = source.colors;
// 		for ( i = 0, il = colors.length; i < il; i ++ ) {
// 			this.colors.push( colors[ i ].clone() );
// 		}
// 		// faces
// 		var faces = source.faces;
// 		for ( i = 0, il = faces.length; i < il; i ++ ) {
// 			this.faces.push( faces[ i ].clone() );
// 		}
// 		// face vertex uvs
// 		for ( i = 0, il = source.faceVertexUvs.length; i < il; i ++ ) {
// 			var faceVertexUvs = source.faceVertexUvs[ i ];
// 			if ( this.faceVertexUvs[ i ] === undefined ) {
// 				this.faceVertexUvs[ i ] = [];
// 			}
// 			for ( j = 0, jl = faceVertexUvs.length; j < jl; j ++ ) {
// 				var uvs = faceVertexUvs[ j ], uvsCopy = [];
// 				for ( k = 0, kl = uvs.length; k < kl; k ++ ) {
// 					var uv = uvs[ k ];
// 					uvsCopy.push( uv.clone() );
// 				}
// 				this.faceVertexUvs[ i ].push( uvsCopy );
// 			}
// 		}
// 		// morph targets
// 		var morphTargets = source.morphTargets;
// 		for ( i = 0, il = morphTargets.length; i < il; i ++ ) {
// 			var morphTarget = {};
// 			morphTarget.name = morphTargets[ i ].name;
// 			// vertices
// 			if ( morphTargets[ i ].vertices !== undefined ) {
// 				morphTarget.vertices = [];
// 				for ( j = 0, jl = morphTargets[ i ].vertices.length; j < jl; j ++ ) {
// 					morphTarget.vertices.push( morphTargets[ i ].vertices[ j ].clone() );
// 				}
// 			}
// 			// normals
// 			if ( morphTargets[ i ].normals !== undefined ) {
// 				morphTarget.normals = [];
// 				for ( j = 0, jl = morphTargets[ i ].normals.length; j < jl; j ++ ) {
// 					morphTarget.normals.push( morphTargets[ i ].normals[ j ].clone() );
// 				}
// 			}
// 			this.morphTargets.push( morphTarget );
// 		}
// 		// morph normals
// 		var morphNormals = source.morphNormals;
// 		for ( i = 0, il = morphNormals.length; i < il; i ++ ) {
// 			var morphNormal = {};
// 			// vertex normals
// 			if ( morphNormals[ i ].vertex_normals !== undefined ) {
// 				morphNormal.vertex_normals = [];
// 				for ( j = 0, jl = morphNormals[ i ].vertex_normals.length; j < jl; j ++ ) {
// 					var srcVertexNormal = morphNormals[ i ].vertex_normals[ j ];
// 					var destVertexNormal = {};
// 					destVertexNormal.a = srcVertexNormal.a.clone();
// 					destVertexNormal.b = srcVertexNormal.b.clone();
// 					destVertexNormal.c = srcVertexNormal.c.clone();
// 					morphNormal.vertex_normals.push( destVertexNormal );
// 				}
// 			}
// 			// face normals
// 			if ( morphNormals[ i ].faceNormals !== undefined ) {
// 				morphNormal.faceNormals = [];
// 				for ( j = 0, jl = morphNormals[ i ].faceNormals.length; j < jl; j ++ ) {
// 					morphNormal.faceNormals.push( morphNormals[ i ].faceNormals[ j ].clone() );
// 				}
// 			}
// 			this.morphNormals.push( morphNormal );
// 		}
// 		// skin weights
// 		var skinWeights = source.skinWeights;
// 		for ( i = 0, il = skinWeights.length; i < il; i ++ ) {
// 			this.skinWeights.push( skinWeights[ i ].clone() );
// 		}
// 		// skin indices
// 		var skinIndices = source.skinIndices;
// 		for ( i = 0, il = skinIndices.length; i < il; i ++ ) {
// 			this.skinIndices.push( skinIndices[ i ].clone() );
// 		}
// 		// line distances
// 		var lineDistances = source.lineDistances;
// 		for ( i = 0, il = lineDistances.length; i < il; i ++ ) {
// 			this.lineDistances.push( lineDistances[ i ] );
// 		}
// 		// bounding box
// 		var boundingBox = source.boundingBox;
// 		if ( boundingBox !== null ) {
// 			this.boundingBox = boundingBox.clone();
// 		}
// 		// bounding sphere
// 		var boundingSphere = source.boundingSphere;
// 		if ( boundingSphere !== null ) {
// 			this.boundingSphere = boundingSphere.clone();
// 		}
// 		// update flags
// 		this.elementsNeedUpdate = source.elementsNeedUpdate;
// 		this.vertices_need_update = source.vertices_need_update;
// 		this.uvsNeedUpdate = source.uvsNeedUpdate;
// 		this.normals_need_update = source.normals_need_update;
// 		this.colorsNeedUpdate = source.colorsNeedUpdate;
// 		this.lineDistancesNeedUpdate = source.lineDistancesNeedUpdate;
// 		this.groupsNeedUpdate = source.groupsNeedUpdate;
// 		return this;
// 	},
// 	dispose: function () {
// 		this.dispatchEvent( { type: 'dispose' } );
// 	}
// } );
// export { Geometry };
