use math::Vector3;
use math::Vector;

#[allow(dead_code)]
#[derive(Clone, Debug, Copy)]
pub struct Matrix4 {
    pub elements: [f64; 16],
}

static IDENTITY: [f64; 16] = [  1.0, 0.0, 0.0, 0.0,
                                0.0, 1.0, 0.0, 0.0,
                                0.0, 0.0, 1.0, 0.0,
                                0.0, 0.0, 0.0, 1.0];

#[allow(dead_code)]
impl Matrix4 {

    pub fn new () -> Self {
        Matrix4 {
            elements: IDENTITY.clone()
        }
    }

    pub fn set ( &mut self, n11:f64, n12:f64, n13:f64, n14:f64, n21:f64, n22:f64, n23:f64, n24:f64, n31:f64, n32:f64, n33:f64, n34:f64, n41:f64, n42:f64, n43:f64, n44:f64 ) -> &mut Self {
    	let mut te = self.elements;
    	te[ 0 ] = n11; te[ 4 ] = n12; te[ 8 ] = n13; te[ 12 ] = n14;
    	te[ 1 ] = n21; te[ 5 ] = n22; te[ 9 ] = n23; te[ 13 ] = n24;
    	te[ 2 ] = n31; te[ 6 ] = n32; te[ 10 ] = n33; te[ 14 ] = n34;
    	te[ 3 ] = n41; te[ 7 ] = n42; te[ 11 ] = n43; te[ 15 ] = n44;
        self
    }

	pub fn identity ( &mut self ) ->  &mut Self {
		self.set(
			1.0, 0.0, 0.0, 0.0,
			0.0, 1.0, 0.0, 0.0,
			0.0, 0.0, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
		);
		self
	}

	pub fn copy_position ( &mut self, m:Self  ) ->  &mut Self {
		let mut te = self.elements;
        let me = m.elements;
		te[ 12 ] = me[ 12 ];
		te[ 13 ] = me[ 13 ];
		te[ 14 ] = me[ 14 ];
		self
	}

    pub fn make_basis ( &mut self, x: Vector3, y: Vector3, z: Vector3 ) -> &mut Self {
		self.set(
			x.x, y.x, z.x, 0.0,
			x.y, y.y, z.y, 0.0,
			x.z, y.z, z.z, 0.0,
			0.0, 0.0, 0.0, 1.0
		);
		self
    }

	pub fn extract_basis (&self, x: &mut Vector3, y: &mut Vector3, z: &mut Vector3 ) -> &Self {
		x.set_from_matrix_column( self, 0 );
		y.set_from_matrix_column( self, 1 );
		z.set_from_matrix_column( self, 2 );
		self
	}

	pub fn makerotation_axis (&mut self, axis: &Vector3, angle: f64 ) -> &mut Self {
		// Based on http://www.gamedev.net/reference/articles/article1199.asp
		let c =  angle.cos();
		let s = angle.sin();
		let t = 1.0 - c;
		let x = axis.x;
        let y = axis.y;
        let z = axis.z;
		let tx = t * x;
        let ty = t * y;
		self.set(
			tx * x + c, tx * y - s * z, tx * z + s * y, 0.0,
			tx * y + s * z, ty * y + c, ty * z - s * x, 0.0,
			tx * z - s * y, ty * z + s * x, t * z * z + c, 0.0,
			0.0, 0.0, 0.0, 1.0
		);
		self
	}

	pub fn make_scale (&mut self, x:f64 , y:f64, z:f64 ) -> &mut Self {
		self.set(
			x, 0.0, 0.0, 0.0,
			0.0, y, 0.0, 0.0,
			0.0, 0.0, z, 0.0,
			0.0, 0.0, 0.0, 1.0
		);
		self
	}

	pub fn make_shear (&mut self, x:f64 , y:f64, z:f64 ) -> &mut Self {
		self.set(
			1.0, y, z, 0.0,
			x, 1.0, z, 0.0,
			x, y, 1.0, 0.0,
			0.0, 0.0, 0.0, 1.0
		);
		self
	}

	pub fn make_translation (&mut self, x:f64 , y:f64, z:f64 ) -> &mut Self {
		self.set(
			1.0, 0.0, 0.0, x,
			0.0, 1.0, 0.0, y,
			0.0, 0.0, 1.0, z,
			0.0, 0.0, 0.0, 1.0
		);
		self
	}

    pub fn make_rotation_x(&mut self, theta: f64 ) -> &mut Self {
    	let c =  theta.cos();
        let s =  theta.sin();
    	self.set(
    		1.0, 0.0, 0.0, 0.0,
    		0.0, c, - s, 0.0,
    		0.0, s, c, 0.0,
    		0.0, 0.0, 0.0, 1.0
    	);
    	self
    }

    pub fn make_rotation_y(&mut self, theta: f64 ) -> &mut Self {
    	let c =  theta.cos();
        let s =  theta.sin();
    	self.set(
    		 c, 0.0, s, 0.0,
    		 0.0, 1.0, 0.0, 0.0,
    		- s, 0.0, c, 0.0,
    		 0.0, 0.0, 0.0, 1.0
    	);
    	self
    }

    pub fn make_rotation_z(&mut self, theta: f64 ) -> &mut Self {
    	let c =  theta.cos();
        let s =  theta.sin();
    	self.set(
    		c, - s, 0.0, 0.0,
    		s, c, 0.0, 0.0,
    		0.0, 0.0, 1.0, 0.0,
    		0.0, 0.0, 0.0, 1.0
    	);
    	self
    }

	pub fn scale (&mut self, v: &Vector3 )-> &mut Self {
		let mut te = self.elements;
		let x = v.x;
        let y = v.y;
        let z = v.z;
		te[ 0 ] *= x; te[ 4 ] *= y; te[ 8 ] *= z;
		te[ 1 ] *= x; te[ 5 ] *= y; te[ 9 ] *= z;
		te[ 2 ] *= x; te[ 6 ] *= y; te[ 10 ] *= z;
		te[ 3 ] *= x; te[ 7 ] *= y; te[ 11 ] *= z;
		self
	}

	pub fn function (&mut self, matrix: &Matrix4 ) -> bool {
		let te = self.elements;
		let me = matrix.elements;

		for i in 0..17  {
			if te[ i ] != me[ i ] {return false};
		}
		true
	}

	pub fn make_perspective (&mut self, left: f64, right: f64, top: f64, bottom: f64, near: f64, far: f64 ) -> &mut Self {
		let mut te = self.elements;
		let x = 2.0 * near / ( right - left );
		let y = 2.0 * near / ( top - bottom );
		let a = ( right + left ) / ( right - left );
		let b = ( top + bottom ) / ( top - bottom );
		let c = - ( far + near ) / ( far - near );
		let d = - 2.0 * far * near / ( far - near );
		te[ 0 ] = x;	te[ 4 ] = 0.0;	te[ 8 ] = a;	te[ 12 ] = 0.0;
		te[ 1 ] = 0.0;	te[ 5 ] = y;	te[ 9 ] = b;	te[ 13 ] = 0.0;
		te[ 2 ] = 0.0;	te[ 6 ] = 0.0;	te[ 10 ] = c;	te[ 14 ] = d;
		te[ 3 ] = 0.0;	te[ 7 ] = 0.0;	te[ 11 ] = - 1.0;	te[ 15 ] = 0.0;
		self
	}

    pub fn make_orthographic (&mut self, left: f64, right: f64, top: f64, bottom: f64, near: f64, far: f64 ) -> &mut Self {
		let mut te = self.elements;
		let w = 1.0 / ( right - left );
		let h = 1.0 / ( top - bottom );
		let p = 1.0 / ( far - near );
		let x = ( right + left ) * w;
		let y = ( top + bottom ) * h;
		let z = ( far + near ) * p;
		te[ 0 ] = 2.0 * w ;	te[ 4 ] = 0.0;	te[ 8 ] = 0.0;	te[ 12 ] = - x;
		te[ 1 ] = 0.0;	te[ 5 ] = 2.0 * h;	te[ 9 ] = 0.0;	te[ 13 ] = - y;
		te[ 2 ] = 0.0;	te[ 6 ] = 0.0;	te[ 10 ] = - 2.0 * p;	te[ 14 ] = - z;
		te[ 3 ] = 0.0;	te[ 7 ] = 0.0;	te[ 11 ] = 0.0;	te[ 15 ] = 1.0;
		self
    }

	pub fn transpose (&mut self) -> &mut Self {
		let mut te = self.elements;
		let mut tmp;
		tmp = te[ 1 ]; te[ 1 ] = te[ 4 ]; te[ 4 ] = tmp;
		tmp = te[ 2 ]; te[ 2 ] = te[ 8 ]; te[ 8 ] = tmp;
		tmp = te[ 6 ]; te[ 6 ] = te[ 9 ]; te[ 9 ] = tmp;
		tmp = te[ 3 ]; te[ 3 ] = te[ 12 ]; te[ 12 ] = tmp;
		tmp = te[ 7 ]; te[ 7 ] = te[ 13 ]; te[ 13 ] = tmp;
		tmp = te[ 11 ]; te[ 11 ] = te[ 14 ]; te[ 14 ] = tmp;
		self
	}

	pub fn set_position (&mut self, v: &Vector3 ) -> &mut Self {
		let mut te = self.elements;
		te[ 12 ] = v.x;
		te[ 13 ] = v.y;
		te[ 14 ] = v.z;
		self
	}

    pub fn	extract_rotation (&mut self, m: &Self) -> &mut Self {
		let mut  v1 =  Vector3::zero();

		let mut te = self.elements;
		let me = m.elements;
		let scale_x = 1.0 / v1.set_from_matrix_column( m, 0 ).length();
		let scale_y = 1.0 / v1.set_from_matrix_column( m, 1 ).length();
		let scale_z = 1.0 / v1.set_from_matrix_column( m, 2 ).length();
		te[ 0 ] = me[ 0 ] * scale_x;
		te[ 1 ] = me[ 1 ] * scale_x;
		te[ 2 ] = me[ 2 ] * scale_x;
		te[ 4 ] = me[ 4 ] * scale_y;
		te[ 5 ] = me[ 5 ] * scale_y;
		te[ 6 ] = me[ 6 ] * scale_y;
		te[ 8 ] = me[ 8 ] * scale_z;
		te[ 9 ] = me[ 9 ] * scale_z;
		te[ 10 ] = me[ 10 ] * scale_z;
        self
    }

	pub fn multiply(&mut self,  m: &Self ) -> &mut Self {
        let clone = &self.clone();
		self.multiply_matrices( clone, m )
	}

    pub fn premultiply (&mut self, m: &Self ) -> &mut Self {
        let clone = &self.clone();
    	self.multiply_matrices( m, clone )
    }

	pub fn multiply_matrices (&mut self, a:&Self, b:&Self ) -> &mut Self {
		let ae = a.elements;
		let be = b.elements;
		let mut te = self.elements;

        let a11 = ae[ 0 ]; let a12 = ae[ 4 ]; let a13 = ae[ 8 ];  let a14 = ae[ 12 ];
		let a21 = ae[ 1 ]; let a22 = ae[ 5 ]; let a23 = ae[ 9 ];  let a24 = ae[ 13 ];
		let a31 = ae[ 2 ]; let a32 = ae[ 6 ]; let a33 = ae[ 10 ]; let a34 = ae[ 14 ];
		let a41 = ae[ 3 ]; let a42 = ae[ 7 ]; let a43 = ae[ 11 ]; let a44 = ae[ 15 ];
		let b11 = be[ 0 ]; let b12 = be[ 4 ]; let b13 = be[ 8 ];  let b14 = be[ 12 ];
		let b21 = be[ 1 ]; let b22 = be[ 5 ]; let b23 = be[ 9 ];  let b24 = be[ 13 ];
		let b31 = be[ 2 ]; let b32 = be[ 6 ]; let b33 = be[ 10 ]; let b34 = be[ 14 ];
		let b41 = be[ 3 ]; let b42 = be[ 7 ]; let b43 = be[ 11 ]; let b44 = be[ 15 ];

        te[ 0 ] = a11 * b11 + a12 * b21 + a13 * b31 + a14 * b41;
		te[ 4 ] = a11 * b12 + a12 * b22 + a13 * b32 + a14 * b42;
		te[ 8 ] = a11 * b13 + a12 * b23 + a13 * b33 + a14 * b43;
		te[ 12 ] = a11 * b14 + a12 * b24 + a13 * b34 + a14 * b44;
		te[ 1 ] = a21 * b11 + a22 * b21 + a23 * b31 + a24 * b41;
		te[ 5 ] = a21 * b12 + a22 * b22 + a23 * b32 + a24 * b42;
		te[ 9 ] = a21 * b13 + a22 * b23 + a23 * b33 + a24 * b43;
		te[ 13 ] = a21 * b14 + a22 * b24 + a23 * b34 + a24 * b44;
		te[ 2 ] = a31 * b11 + a32 * b21 + a33 * b31 + a34 * b41;
		te[ 6 ] = a31 * b12 + a32 * b22 + a33 * b32 + a34 * b42;
		te[ 10 ] = a31 * b13 + a32 * b23 + a33 * b33 + a34 * b43;
		te[ 14 ] = a31 * b14 + a32 * b24 + a33 * b34 + a34 * b44;
		te[ 3 ] = a41 * b11 + a42 * b21 + a43 * b31 + a44 * b41;
		te[ 7 ] = a41 * b12 + a42 * b22 + a43 * b32 + a44 * b42;
		te[ 11 ] = a41 * b13 + a42 * b23 + a43 * b33 + a44 * b43;
		te[ 15 ] = a41 * b14 + a42 * b24 + a43 * b34 + a44 * b44;

        self
	}

	pub fn look_at (&mut self, eye: &Vector3, target: &Vector3, up: &Vector3 ) -> &mut Self {
		let mut x = Vector3::zero();
		let mut y = Vector3::zero();
		let mut z = Vector3::zero();

		let mut te = self.elements;

        z.subVectors( eye, target );
		if  z.lengthSq() == 0.0  {
			// eye and target are in the same position
			z.z = 1.0;
		}
		z.normalize();

        x.cross_vectors( up, &z );

        if  x.lengthSq() == 0.0  {
			// up and z are parallel
			if  up.z.abs() == 1.0  {
				z.x += 0.0001;
			} else {
				z.z += 0.0001;
			}

			z.normalize();
			x.cross_vectors( up, &z );
		}

        x.normalize();
		y.cross_vectors( &z, &x );

        te[ 0 ] = x.x; te[ 4 ] = y.x; te[ 8 ] = z.x;
		te[ 1 ] = x.y; te[ 5 ] = y.y; te[ 9 ] = z.y;
		te[ 2 ] = x.z; te[ 6 ] = y.z; te[ 10 ] = z.z;

        self
	}


	pub fn multiply_scalar ( &mut self, s: f64 ) -> &mut Self {
		let mut te = self.elements;
		te[ 0 ] *= s; te[ 4 ] *= s; te[ 8 ] *= s; te[ 12 ] *= s;
		te[ 1 ] *= s; te[ 5 ] *= s; te[ 9 ] *= s; te[ 13 ] *= s;
		te[ 2 ] *= s; te[ 6 ] *= s; te[ 10 ] *= s; te[ 14 ] *= s;
		te[ 3 ] *= s; te[ 7 ] *= s; te[ 11 ] *= s; te[ 15 ] *= s;
		self
	}

    pub fn determinant (&mut self) -> f64 {
		let te = self.elements;
		let n11 = te[ 0 ]; let n12 = te[ 4 ]; let n13 = te[ 8 ];  let n14 = te[ 12 ];
		let n21 = te[ 1 ]; let n22 = te[ 5 ]; let n23 = te[ 9 ];  let n24 = te[ 13 ];
		let n31 = te[ 2 ]; let n32 = te[ 6 ]; let n33 = te[ 10 ]; let n34 = te[ 14 ];
		let n41 = te[ 3 ]; let n42 = te[ 7 ]; let n43 = te[ 11 ]; let n44 = te[ 15 ];
		//TODO: make this more efficient
		//( based on http://www.euclideanspace.com/maths/algebra/matrix/functions/inverse/fourD/index.htm )

		n41 * (
			  n14 * n23 * n32
			- n13 * n24 * n32
			- n14 * n22 * n33
			+ n12 * n24 * n33
			+ n13 * n22 * n34
			- n12 * n23 * n34
        ) +
		n42 * (
			  n11 * n23 * n34
			 - n11 * n24 * n33
			 + n14 * n21 * n33
			 - n13 * n21 * n34
			 + n13 * n24 * n31
			 - n14 * n23 * n31
		) +
		n43 * (
			  n11 * n24 * n32
			 - n11 * n22 * n34
			 - n14 * n21 * n32
			 + n12 * n21 * n34
			 + n14 * n22 * n31
			 - n12 * n24 * n31
		) +
		n44 * (
			- n13 * n22 * n31
			 - n11 * n23 * n32
			 + n11 * n22 * n33
			 + n13 * n21 * n32
			 - n12 * n21 * n33
			 + n12 * n23 * n31
		)
	}

    pub fn get_inverse (&mut self, m: &Self, throw_on_degenerate: bool ) -> Result<&mut Self, &str> {
		// based on http://www.euclideanspace.com/maths/algebra/matrix/functions/inverse/fourD/index.htm
		let mut te = self.elements;
		let	me = m.elements;
		let n11 = me[ 0 ]; let n21 = me[ 1 ]; let n31 = me[ 2 ]; let n41 = me[ 3 ];
		let n12 = me[ 4 ]; let n22 = me[ 5 ]; let n32 = me[ 6 ]; let n42 = me[ 7 ];
		let n13 = me[ 8 ]; let n23 = me[ 9 ]; let n33 = me[ 10 ]; let n43 = me[ 11 ];
		let n14 = me[ 12 ]; let n24 = me[ 13 ]; let n34 = me[ 14 ]; let n44 = me[ 15 ];
		let t11 = n23 * n34 * n42 - n24 * n33 * n42 + n24 * n32 * n43 - n22 * n34 * n43 - n23 * n32 * n44 + n22 * n33 * n44;
		let t12 = n14 * n33 * n42 - n13 * n34 * n42 - n14 * n32 * n43 + n12 * n34 * n43 + n13 * n32 * n44 - n12 * n33 * n44;
		let t13 = n13 * n24 * n42 - n14 * n23 * n42 + n14 * n22 * n43 - n12 * n24 * n43 - n13 * n22 * n44 + n12 * n23 * n44;
		let t14 = n14 * n23 * n32 - n13 * n24 * n32 - n14 * n22 * n33 + n12 * n24 * n33 + n13 * n22 * n34 - n12 * n23 * n34;
		let det = n11 * t11 + n21 * t12 + n31 * t13 + n41 * t14;

        if det == 0.0 {
			let msg = "THREE.Matrix4: .getInverse() can't invert matrix, determinant is 0";
            eprintln!("{}", msg);

            if throw_on_degenerate == true {
                return Err(msg);
			}

            return Ok(self.identity());
		}

        let det_inv = 1.0 / det;
		te[ 0 ] = t11 * det_inv;
		te[ 1 ] = ( n24 * n33 * n41 - n23 * n34 * n41 - n24 * n31 * n43 + n21 * n34 * n43 + n23 * n31 * n44 - n21 * n33 * n44 ) * det_inv;
		te[ 2 ] = ( n22 * n34 * n41 - n24 * n32 * n41 + n24 * n31 * n42 - n21 * n34 * n42 - n22 * n31 * n44 + n21 * n32 * n44 ) * det_inv;
		te[ 3 ] = ( n23 * n32 * n41 - n22 * n33 * n41 - n23 * n31 * n42 + n21 * n33 * n42 + n22 * n31 * n43 - n21 * n32 * n43 ) * det_inv;
		te[ 4 ] = t12 * det_inv;
		te[ 5 ] = ( n13 * n34 * n41 - n14 * n33 * n41 + n14 * n31 * n43 - n11 * n34 * n43 - n13 * n31 * n44 + n11 * n33 * n44 ) * det_inv;
		te[ 6 ] = ( n14 * n32 * n41 - n12 * n34 * n41 - n14 * n31 * n42 + n11 * n34 * n42 + n12 * n31 * n44 - n11 * n32 * n44 ) * det_inv;
		te[ 7 ] = ( n12 * n33 * n41 - n13 * n32 * n41 + n13 * n31 * n42 - n11 * n33 * n42 - n12 * n31 * n43 + n11 * n32 * n43 ) * det_inv;
		te[ 8 ] = t13 * det_inv;
		te[ 9 ] = ( n14 * n23 * n41 - n13 * n24 * n41 - n14 * n21 * n43 + n11 * n24 * n43 + n13 * n21 * n44 - n11 * n23 * n44 ) * det_inv;
		te[ 10 ] = ( n12 * n24 * n41 - n14 * n22 * n41 + n14 * n21 * n42 - n11 * n24 * n42 - n12 * n21 * n44 + n11 * n22 * n44 ) * det_inv;
		te[ 11 ] = ( n13 * n22 * n41 - n12 * n23 * n41 - n13 * n21 * n42 + n11 * n23 * n42 + n12 * n21 * n43 - n11 * n22 * n43 ) * det_inv;
		te[ 12 ] = t14 * det_inv;
		te[ 13 ] = ( n13 * n24 * n31 - n14 * n23 * n31 + n14 * n21 * n33 - n11 * n24 * n33 - n13 * n21 * n34 + n11 * n23 * n34 ) * det_inv;
		te[ 14 ] = ( n14 * n22 * n31 - n12 * n24 * n31 - n14 * n21 * n32 + n11 * n24 * n32 + n12 * n21 * n34 - n11 * n22 * n34 ) * det_inv;
		te[ 15 ] = ( n12 * n23 * n31 - n13 * n22 * n31 + n13 * n21 * n32 - n11 * n23 * n32 - n12 * n21 * n33 + n11 * n22 * n33 ) * det_inv;

        Ok(self)
	}

	pub fn get_max_scale_on_axis (&self) -> f64 {
		let te = self.elements;

        let scale_xs_q = te[ 0 ] * te[ 0 ] + te[ 1 ] * te[ 1 ] + te[ 2 ] * te[ 2 ];
		let scale_ys_q = te[ 4 ] * te[ 4 ] + te[ 5 ] * te[ 5 ] + te[ 6 ] * te[ 6 ];
		let scale_zs_q = te[ 8 ] * te[ 8 ] + te[ 9 ] * te[ 9 ] + te[ 10 ] * te[ 10 ];

        return ( scale_xs_q.max(scale_ys_q).max(scale_zs_q) ).sqrt();
	}
}



// import { Vector3 } from './Vector3.js';
// /**
//  * @author mrdoob / http://mrdoob.com/
//  * @author supereggbert / http://www.paulbrunt.co.uk/
//  * @author philogb / http://blog.thejit.org/
//  * @author jordi_ros / http://plattsoft.com
//  * @author D1plo1d / http://github.com/D1plo1d
//  * @author alteredq / http://alteredqualia.com/
//  * @author mikael emtinger / http://gomo.se/
//  * @author timknip / http://www.floorplanner.com/
//  * @author bhouston / http://clara.io
//  * @author WestLangley / http://github.com/WestLangley
//  */
// function Matrix4() {
// 	this.elements = [
// 		1, 0, 0, 0,
// 		0, 1, 0, 0,
// 		0, 0, 1, 0,
// 		0, 0, 0, 1
// 	];
// 	if ( arguments.length > 0 ) {
// 		console.error( 'THREE.Matrix4: the constructor no longer reads arguments. use .set() instead.' );
// 	}
// }
// Object.assign( Matrix4.prototype, {
// 	isMatrix4: true,



// 	makeRotationFromEuler: function ( euler ) {
// 		if ( ! ( euler && euler.isEuler ) ) {
// 			console.error( 'THREE.Matrix4: .makeRotationFromEuler() now expects a Euler rotation rather than a Vector3 and order.' );
// 		}
// 		var te = this.elements;
// 		var x = euler.x, y = euler.y, z = euler.z;
// 		var a = Math.cos( x ), b = Math.sin( x );
// 		var c = Math.cos( y ), d = Math.sin( y );
// 		var e = Math.cos( z ), f = Math.sin( z );
// 		if ( euler.order === 'XYZ' ) {
// 			var ae = a * e, af = a * f, be = b * e, bf = b * f;
// 			te[ 0 ] = c * e;
// 			te[ 4 ] = - c * f;
// 			te[ 8 ] = d;
// 			te[ 1 ] = af + be * d;
// 			te[ 5 ] = ae - bf * d;
// 			te[ 9 ] = - b * c;
// 			te[ 2 ] = bf - ae * d;
// 			te[ 6 ] = be + af * d;
// 			te[ 10 ] = a * c;
// 		} else if ( euler.order === 'YXZ' ) {
// 			var ce = c * e, cf = c * f, de = d * e, df = d * f;
// 			te[ 0 ] = ce + df * b;
// 			te[ 4 ] = de * b - cf;
// 			te[ 8 ] = a * d;
// 			te[ 1 ] = a * f;
// 			te[ 5 ] = a * e;
// 			te[ 9 ] = - b;
// 			te[ 2 ] = cf * b - de;
// 			te[ 6 ] = df + ce * b;
// 			te[ 10 ] = a * c;
// 		} else if ( euler.order === 'ZXY' ) {
// 			var ce = c * e, cf = c * f, de = d * e, df = d * f;
// 			te[ 0 ] = ce - df * b;
// 			te[ 4 ] = - a * f;
// 			te[ 8 ] = de + cf * b;
// 			te[ 1 ] = cf + de * b;
// 			te[ 5 ] = a * e;
// 			te[ 9 ] = df - ce * b;
// 			te[ 2 ] = - a * d;
// 			te[ 6 ] = b;
// 			te[ 10 ] = a * c;
// 		} else if ( euler.order === 'ZYX' ) {
// 			var ae = a * e, af = a * f, be = b * e, bf = b * f;
// 			te[ 0 ] = c * e;
// 			te[ 4 ] = be * d - af;
// 			te[ 8 ] = ae * d + bf;
// 			te[ 1 ] = c * f;
// 			te[ 5 ] = bf * d + ae;
// 			te[ 9 ] = af * d - be;
// 			te[ 2 ] = - d;
// 			te[ 6 ] = b * c;
// 			te[ 10 ] = a * c;
// 		} else if ( euler.order === 'YZX' ) {
// 			var ac = a * c, ad = a * d, bc = b * c, bd = b * d;
// 			te[ 0 ] = c * e;
// 			te[ 4 ] = bd - ac * f;
// 			te[ 8 ] = bc * f + ad;
// 			te[ 1 ] = f;
// 			te[ 5 ] = a * e;
// 			te[ 9 ] = - b * e;
// 			te[ 2 ] = - d * e;
// 			te[ 6 ] = ad * f + bc;
// 			te[ 10 ] = ac - bd * f;
// 		} else if ( euler.order === 'XZY' ) {
// 			var ac = a * c, ad = a * d, bc = b * c, bd = b * d;
// 			te[ 0 ] = c * e;
// 			te[ 4 ] = - f;
// 			te[ 8 ] = d * e;
// 			te[ 1 ] = ac * f + bd;
// 			te[ 5 ] = a * e;
// 			te[ 9 ] = ad * f - bc;
// 			te[ 2 ] = bc * f - ad;
// 			te[ 6 ] = b * e;
// 			te[ 10 ] = bd * f + ac;
// 		}
// 		// last column
// 		te[ 3 ] = 0;
// 		te[ 7 ] = 0;
// 		te[ 11 ] = 0;
// 		// bottom row
// 		te[ 12 ] = 0;
// 		te[ 13 ] = 0;
// 		te[ 14 ] = 0;
// 		te[ 15 ] = 1;
// 		return this;
// 	},
// 	makeRotationFromQuaternion: function ( q ) {
// 		var te = this.elements;
// 		var x = q._x, y = q._y, z = q._z, w = q._w;
// 		var x2 = x + x, y2 = y + y, z2 = z + z;
// 		var xx = x * x2, xy = x * y2, xz = x * z2;
// 		var yy = y * y2, yz = y * z2, zz = z * z2;
// 		var wx = w * x2, wy = w * y2, wz = w * z2;
// 		te[ 0 ] = 1 - ( yy + zz );
// 		te[ 4 ] = xy - wz;
// 		te[ 8 ] = xz + wy;
// 		te[ 1 ] = xy + wz;
// 		te[ 5 ] = 1 - ( xx + zz );
// 		te[ 9 ] = yz - wx;
// 		te[ 2 ] = xz - wy;
// 		te[ 6 ] = yz + wx;
// 		te[ 10 ] = 1 - ( xx + yy );
// 		// last column
// 		te[ 3 ] = 0;
// 		te[ 7 ] = 0;
// 		te[ 11 ] = 0;
// 		// bottom row
// 		te[ 12 ] = 0;
// 		te[ 13 ] = 0;
// 		te[ 14 ] = 0;
// 		te[ 15 ] = 1;
// 		return this;
// 	},

// 	applyToBufferAttribute: function () {
// 		var v1 = new Vector3();
// 		return function applyToBufferAttribute( attribute ) {
// 			for ( var i = 0, l = attribute.count; i < l; i ++ ) {
// 				v1.x = attribute.getX( i );
// 				v1.y = attribute.getY( i );
// 				v1.z = attribute.getZ( i );
// 				v1.applyMatrix4( this );
// 				attribute.setXYZ( i, v1.x, v1.y, v1.z );
// 			}
// 			return attribute;
// 		};
// 	}(),




// 	compose: function ( position, quaternion, scale ) {
// 		this.makeRotationFromQuaternion( quaternion );
// 		this.scale( scale );
// 		this.setPosition( position );
// 		return this;
// 	},
// 	decompose: function () {
// 		var vector = new Vector3();
// 		var matrix = new Matrix4();
// 		return function decompose( position, quaternion, scale ) {
// 			var te = this.elements;
// 			var sx = vector.set( te[ 0 ], te[ 1 ], te[ 2 ] ).length();
// 			var sy = vector.set( te[ 4 ], te[ 5 ], te[ 6 ] ).length();
// 			var sz = vector.set( te[ 8 ], te[ 9 ], te[ 10 ] ).length();
// 			// if determine is negative, we need to invert one scale
// 			var det = this.determinant();
// 			if ( det < 0 ) sx = - sx;
// 			position.x = te[ 12 ];
// 			position.y = te[ 13 ];
// 			position.z = te[ 14 ];
// 			// scale the rotation part
// 			matrix.copy( this );
// 			var invSX = 1 / sx;
// 			var invSY = 1 / sy;
// 			var invSZ = 1 / sz;
// 			matrix.elements[ 0 ] *= invSX;
// 			matrix.elements[ 1 ] *= invSX;
// 			matrix.elements[ 2 ] *= invSX;
// 			matrix.elements[ 4 ] *= invSY;
// 			matrix.elements[ 5 ] *= invSY;
// 			matrix.elements[ 6 ] *= invSY;
// 			matrix.elements[ 8 ] *= invSZ;
// 			matrix.elements[ 9 ] *= invSZ;
// 			matrix.elements[ 10 ] *= invSZ;
// 			quaternion.setFromRotationMatrix( matrix );
// 			scale.x = sx;
// 			scale.y = sy;
// 			scale.z = sz;
// 			return this;
// 		};
// 	}(),


// 	fromArray: function ( array, offset ) {
// 		if ( offset === undefined ) offset = 0;
// 		for ( var i = 0; i < 16; i ++ ) {
// 			this.elements[ i ] = array[ i + offset ];
// 		}
// 		return this;
// 	},
// 	toArray: function ( array, offset ) {
// 		if ( array === undefined ) array = [];
// 		if ( offset === undefined ) offset = 0;
// 		var te = this.elements;
// 		array[ offset ] = te[ 0 ];
// 		array[ offset + 1 ] = te[ 1 ];
// 		array[ offset + 2 ] = te[ 2 ];
// 		array[ offset + 3 ] = te[ 3 ];
// 		array[ offset + 4 ] = te[ 4 ];
// 		array[ offset + 5 ] = te[ 5 ];
// 		array[ offset + 6 ] = te[ 6 ];
// 		array[ offset + 7 ] = te[ 7 ];
// 		array[ offset + 8 ] = te[ 8 ];
// 		array[ offset + 9 ] = te[ 9 ];
// 		array[ offset + 10 ] = te[ 10 ];
// 		array[ offset + 11 ] = te[ 11 ];
// 		array[ offset + 12 ] = te[ 12 ];
// 		array[ offset + 13 ] = te[ 13 ];
// 		array[ offset + 14 ] = te[ 14 ];
// 		array[ offset + 15 ] = te[ 15 ];
// 		return array;
// 	}
// } );
// export { Matrix4 };