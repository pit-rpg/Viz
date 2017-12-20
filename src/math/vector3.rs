// mod vector;
//
use math::Vector;

#[derive(Clone, Debug, Copy)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


#[allow(dead_code)]
impl Vector3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn cross_vectors ( &mut self, a: &Self, b: &Self ) -> &mut Self {
        let ax = a.x;
        let ay = a.y;
        let az = a.z;
        let bx = b.x;
        let by = b.y;
        let bz = b.z;
        self.x = ay * bz - az * by;
        self.y = az * bx - ax * bz;
        self.z = ax * by - ay * bx;
        self
    }

    pub fn cross (&mut self, v: &Self )-> &mut Self {
        let c = Self::clone(self);
        self.cross_vectors(&c , v )
    }

    pub fn set(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }
}

impl Vector for Vector3 {

    fn multiplyScalar(&mut self, s: f64) -> &mut Self {
        self.x *= s;
        self.y *= s;
        self.z *= s;
        self
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        // return Math.sqrt( this.x * this.x + this.y * this.y + this.z * this.z );
    }

    fn lengthSq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn manhattanLength(&mut self) -> f64 {
        (self.x).abs() + (self.y).abs() + (self.z).abs()
    }

    fn setScalar(&mut self, s: f64) -> &mut Self {
        self.x = s;
        self.y = s;
        self.z = s;
        self
    }

    fn addScalar(&mut self, s: f64) -> &mut Self {
        self.x += s;
        self.y += s;
        self.z += s;
        self
    }

    fn subScalar(&mut self, s: f64) -> &mut Self {
        self.x -= s;
        self.y -= s;
        self.z -= s;
        self
    }

    fn add(&mut self, v: &Self) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
        self
    }

    fn sub(&mut self, v: &Self) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
        self
    }

    fn multiply(&mut self, v: &Self) -> &mut Self {
        self.x *= v.x;
        self.y *= v.y;
        self.z *= v.z;
        self
    }

    fn divide(&mut self, v: &Self) -> &mut Self {
        self.x /= v.x;
        self.y /= v.y;
        self.z /= v.z;
        self
    }

    fn addVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        self.z = a.z + b.z;
        self
    }

    fn subVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x - b.x;
        self.y = a.y - b.y;
        self.z = a.z - b.z;
        self
    }

    fn multiplyVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x * b.x;
        self.y = a.y * b.y;
        self.z = a.z * b.z;
        self
    }

    fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    fn min(&mut self, v: &Self) -> &mut Self {
        self.x = (self.x).min(v.x);
        self.y = (self.y).min(v.y);
        self.z = (self.z).min(v.z);
        self
    }

    fn max(&mut self, v: &Self) -> &mut Self {
        self.x = (self.x).max(v.x);
        self.y = (self.y).max(v.y);
        self.z = (self.z).max(v.z);
        self
    }

    fn dot(&mut self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    fn round(&mut self) -> &mut Self {
        self.x = self.x.round();
        self.y = self.y.round();
        self.z = self.z.round();
        self
    }

    fn floor(&mut self) -> &mut Self {
        self.x = self.x.floor();
        self.y = self.y.floor();
        self.z = self.z.floor();
        self
    }

    fn ceil(&mut self) -> &mut Self {
        self.x = self.x.ceil();
        self.y = self.y.ceil();
        self.z = self.z.ceil();
        self
    }

    fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self {
		self.x = min.x.max(max.x.min(self.x));
		self.y = min.y.max(max.y.min(self.y));
		self.z = min.z.max(max.z.min(self.z));
		self
    }

    fn lerp (&mut self, v: &Self,  alpha:f64 )-> &mut Self {
        self.x += ( v.x - self.x ) * alpha;
        self.y += ( v.y - self.y ) * alpha;
        self.z += ( v.z - self.z ) * alpha;
		self
    }

}

// 	clampScalar: function () {
// 		var min = new Vector3();
// 		var max = new Vector3();
// 		return function clampScalar( minVal, maxVal ) {
// 			min.set( minVal, minVal, minVal );
// 			max.set( maxVal, maxVal, maxVal );
// 			return this.clamp( min, max );
// 		};
// 	}(),

// 	roundToZero: function () {
// 		this.x = ( this.x < 0 ) ? Math.ceil( this.x ) : Math.floor( this.x );
// 		this.y = ( this.y < 0 ) ? Math.ceil( this.y ) : Math.floor( this.y );
// 		this.z = ( this.z < 0 ) ? Math.ceil( this.z ) : Math.floor( this.z );
// 		return this;
// 	},






// 	projectOnVector: function ( vector ) {
// 		var scalar = vector.dot( this ) / vector.lengthSq();
// 		return this.copy( vector ).multiplyScalar( scalar );
// 	},
// 	projectOnPlane: function () {
// 		var v1 = new Vector3();
// 		return function projectOnPlane( planeNormal ) {
// 			v1.copy( this ).projectOnVector( planeNormal );
// 			return this.sub( v1 );
// 		};
// 	}(),
// 	reflect: function () {
// 		// reflect incident vector off plane orthogonal to normal
// 		// normal is assumed to have unit length
// 		var v1 = new Vector3();
// 		return function reflect( normal ) {
// 			return this.sub( v1.copy( normal ).multiplyScalar( 2 * this.dot( normal ) ) );
// 		};
// 	}(),
// 	angleTo: function ( v ) {
// 		var theta = this.dot( v ) / ( Math.sqrt( this.lengthSq() * v.lengthSq() ) );
// 		// clamp, to handle numerical problems
// 		return Math.acos( _Math.clamp( theta, - 1, 1 ) );
// 	},
// 	distanceTo: function ( v ) {
// 		return Math.sqrt( this.distanceToSquared( v ) );
// 	},
// 	distanceToSquared: function ( v ) {
// 		var dx = this.x - v.x, dy = this.y - v.y, dz = this.z - v.z;
// 		return dx * dx + dy * dy + dz * dz;
// 	},
// 	manhattanDistanceTo: function ( v ) {
// 		return Math.abs( this.x - v.x ) + Math.abs( this.y - v.y ) + Math.abs( this.z - v.z );
// 	},
// 	setFromSpherical: function ( s ) {
// 		var sinPhiRadius = Math.sin( s.phi ) * s.radius;
// 		this.x = sinPhiRadius * Math.sin( s.theta );
// 		this.y = Math.cos( s.phi ) * s.radius;
// 		this.z = sinPhiRadius * Math.cos( s.theta );
// 		return this;
// 	},
// 	setFromCylindrical: function ( c ) {
// 		this.x = c.radius * Math.sin( c.theta );
// 		this.y = c.y;
// 		this.z = c.radius * Math.cos( c.theta );
// 		return this;
// 	},
// 	setFromMatrixPosition: function ( m ) {
// 		var e = m.elements;
// 		this.x = e[ 12 ];
// 		this.y = e[ 13 ];
// 		this.z = e[ 14 ];
// 		return this;
// 	},
// 	setFromMatrixScale: function ( m ) {
// 		var sx = this.setFromMatrixColumn( m, 0 ).length();
// 		var sy = this.setFromMatrixColumn( m, 1 ).length();
// 		var sz = this.setFromMatrixColumn( m, 2 ).length();
// 		this.x = sx;
// 		this.y = sy;
// 		this.z = sz;
// 		return this;
// 	},
// 	setFromMatrixColumn: function ( m, index ) {
// 		return this.fromArray( m.elements, index * 4 );
// 	},
// 	equals: function ( v ) {
// 		return ( ( v.x === this.x ) && ( v.y === this.y ) && ( v.z === this.z ) );
// 	},
// 	fromArray: function ( array, offset ) {
// 		if ( offset === undefined ) offset = 0;
// 		this.x = array[ offset ];
// 		this.y = array[ offset + 1 ];
// 		this.z = array[ offset + 2 ];
// 		return this;
// 	},
// 	toArray: function ( array, offset ) {
// 		if ( array === undefined ) array = [];
// 		if ( offset === undefined ) offset = 0;
// 		array[ offset ] = this.x;
// 		array[ offset + 1 ] = this.y;
// 		array[ offset + 2 ] = this.z;
// 		return array;
// 	},
// 	fromBufferAttribute: function ( attribute, index, offset ) {
// 		if ( offset !== undefined ) {
// 			console.warn( 'THREE.Vector3: offset has been removed from .fromBufferAttribute().' );
// 		}
// 		this.x = attribute.getX( index );
// 		this.y = attribute.getY( index );
// 		this.z = attribute.getZ( index );
// 		return this;
// 	}
// } );
// // 	setComponent: function ( index, value ) {
// 		switch ( index ) {
// 			case 0: this.x = value; break;
// 			case 1: this.y = value; break;
// 			case 2: this.z = value; break;
// 			default: throw new Error( 'index is out of range: ' + index );
// 		}
// 		return this;
// 	},
// 	getComponent: function ( index ) {
// 		switch ( index ) {
// 			case 0: return this.x;
// 			case 1: return this.y;
// 			case 2: return this.z;
// 			default: throw new Error( 'index is out of range: ' + index );
// 		}
// 	},

// 	// 	addScaledVector: function ( v, s ) {
// 		this.x += v.x * s;
// 		this.y += v.y * s;
// 		this.z += v.z * s;
// 		return this;
// 	},
// 	// 	applyEuler: function () {
// 		var quaternion = new Quaternion();
// 		return function applyEuler( euler ) {
// 			if ( ! ( euler && euler.isEuler ) ) {
// 				console.error( 'THREE.Vector3: .applyEuler() now expects an Euler rotation rather than a Vector3 and order.' );
// 			}
// 			return this.applyQuaternion( quaternion.setFromEuler( euler ) );
// 		};
// 	}(),
// 	applyAxisAngle: function () {
// 		var quaternion = new Quaternion();
// 		return function applyAxisAngle( axis, angle ) {
// 			return this.applyQuaternion( quaternion.setFromAxisAngle( axis, angle ) );
// 		};
// 	}(),
// 	// 	applyMatrix3: function ( m ) {
// 		var x = this.x, y = this.y, z = this.z;
// 		var e = m.elements;
// 		this.x = e[ 0 ] * x + e[ 3 ] * y + e[ 6 ] * z;
// 		this.y = e[ 1 ] * x + e[ 4 ] * y + e[ 7 ] * z;
// 		this.z = e[ 2 ] * x + e[ 5 ] * y + e[ 8 ] * z;
// 		return this;
// 	},
// 	applyMatrix4: function ( m ) {
// 		var x = this.x, y = this.y, z = this.z;
// 		var e = m.elements;
// 		var w = 1 / ( e[ 3 ] * x + e[ 7 ] * y + e[ 11 ] * z + e[ 15 ] );
// 		this.x = ( e[ 0 ] * x + e[ 4 ] * y + e[ 8 ] * z + e[ 12 ] ) * w;
// 		this.y = ( e[ 1 ] * x + e[ 5 ] * y + e[ 9 ] * z + e[ 13 ] ) * w;
// 		this.z = ( e[ 2 ] * x + e[ 6 ] * y + e[ 10 ] * z + e[ 14 ] ) * w;
// 		return this;
// 	},
// 	applyQuaternion: function ( q ) {
// 		var x = this.x, y = this.y, z = this.z;
// 		var qx = q.x, qy = q.y, qz = q.z, qw = q.w;
// 		// calculate quat * vector
// 		var ix = qw * x + qy * z - qz * y;
// 		var iy = qw * y + qz * x - qx * z;
// 		var iz = qw * z + qx * y - qy * x;
// 		var iw = - qx * x - qy * y - qz * z;
// 		// calculate result * inverse quat
// 		this.x = ix * qw + iw * - qx + iy * - qz - iz * - qy;
// 		this.y = iy * qw + iw * - qy + iz * - qx - ix * - qz;
// 		this.z = iz * qw + iw * - qz + ix * - qy - iy * - qx;
// 		return this;
// 	},
// 	project: function () {
// 		var matrix = new Matrix4();
// 		return function project( camera ) {
// 			matrix.multiplyMatrices( camera.projectionMatrix, matrix.getInverse( camera.matrixWorld ) );
// 			return this.applyMatrix4( matrix );
// 		};
// 	}(),
// 	unproject: function () {
// 		var matrix = new Matrix4();
// 		return function unproject( camera ) {
// 			matrix.multiplyMatrices( camera.matrixWorld, matrix.getInverse( camera.projectionMatrix ) );
// 			return this.applyMatrix4( matrix );
// 		};
// 	}(),
// 	transformDirection: function ( m ) {
// 		// input: THREE.Matrix4 affine matrix
// 		// vector interpreted as a direction
// 		var x = this.x, y = this.y, z = this.z;
// 		var e = m.elements;
// 		this.x = e[ 0 ] * x + e[ 4 ] * y + e[ 8 ] * z;
// 		this.y = e[ 1 ] * x + e[ 5 ] * y + e[ 9 ] * z;
// 		this.z = e[ 2 ] * x + e[ 6 ] * y + e[ 10 ] * z;
// 		return this.normalize();
// 	},
