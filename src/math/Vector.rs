// use super::Vector::Vector;
// use std::io;
// mod Vector;

pub trait Vector {
    fn add_vectors(&mut self, a: &Self, b: &Self) -> &Self;
    fn add(&mut self, a: &Self) -> &Self;
    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &Self;
    fn sub(&mut self, v: &Self) -> &Self;
    fn multiplyScalarVector<'a, 'b>(v: &'a mut Self, s: &'b f64) -> &'a Self;
    fn negate(&mut self) -> &Self;
    fn dot(&self, v: &Self) -> f64;
    fn lengthSq(&self) -> f64;
    fn length(&self) -> f64;
    fn lengthManhattan(&self) -> f64;
    fn distanceToSquared(&self, v: &Self) -> f64;
    fn crossVectors(&mut self, a: &Self, b: &Self) -> &Self;
    fn cross(&mut self, v: &Self) -> &Self;
    fn equals(&mut self, v: &Self) -> bool;
    fn min(&mut self, v: &Self) -> &Self;
    fn max(&mut self, v: &Self) -> &Self;
    fn setLength(&mut self, length: f64) -> &Self {
        let thisLength = self.length();
        self.multiplyScalar(&(length / thisLength))
    }
    fn normalize(&mut self) -> &Self {
        let length = self.length();
        self.divideScalar(&length)
    }
    fn distanceTo(&self, v: &Self) -> f64 {
        (self.distanceToSquared(v)).sqrt()
    }
    fn multiplyScalar(&mut self, s: &f64) -> &Self {
        Self::multiplyScalarVector(self, s)
    }
    fn divideScalar(&mut self, s: &f64) -> &Self {
        Self::multiplyScalarVector(self, &(1.0 / s))
    }
    fn clamp(&mut self, min: &Self, max: &Self) -> &Self {
        self.min(min);
        self.max(max);
        self
    }
    // .floor ()
    // .ceil ()
    // .round ()
    // .clampScalar (min, max) this
    // .roundToZero ()
    // .setFromMatrixPosition ( m ) this
    // .setFromMatrixScale ( m ) this
    // .clone ()
    // .applyMatrix3 (m) this
    // .applyMatrix4 (m) this
    // .projectOnPlane (planeNormal) this
    // .projectOnVector (Vector3) this
    // .addScalar (Float) this
    // .divide (v) this
    // .setComponent (index, value) this
    // .getComponent (index)
    // .applyAxisAngle (axis, angle) this
    // .transformDirection (m) this
    // .multiplyVectors (a, b) this
    // .lerp (v, alpha) this
    // .lerpVectors (v1, v2, alpha) this
    // .angleTo (v)
    // .setFromMatrixColumn (index, matrix) this
    // .reflect (normal) this
    // .fromArray (array) this
    // .multiply (v) this
    // .applyProjection (m) this
    // .toArray ( array )
    // .applyEuler (euler) this
    // .applyQuaternion (quaternion) this
    // .project ( camera )
    // .unproject ( camera )
    //
    fn copy(&mut self, from: &Self) -> &Self;
}

#[allow(dead_code)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(dead_code)]
impl Vector3 {
    pub fn set(&mut self, x: f64, y: f64, z: f64) -> &Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }
}

impl Vector for Vector3 {
    fn add_vectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        self.z = a.z + b.z;
        self
    }

    fn add(&mut self, a: &Self) -> &Self {
        self.x = a.x + self.x;
        self.y = a.y + self.y;
        self.z = a.z + self.z;
        self
    }

    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x - b.x;
        self.y = a.y - b.y;
        self.z = a.z - b.z;
        self
    }

    fn sub(&mut self, a: &Self) -> &Self {
        self.x = a.x - self.x;
        self.y = a.y - self.y;
        self.z = a.z - self.z;
        self
    }

    fn copy(&mut self, from: &Self) -> &Self {
        self.x = from.x.clone();
        self.y = from.y.clone();
        self.z = from.z.clone();
        self
    }

    fn multiplyScalarVector<'a, 'b>(v: &'a mut Self, s: &'b f64) -> &'a Self {
        v.x = v.x * s;
        v.y = v.y * s;
        v.z = v.y * s;
        v
    }

    fn negate(&mut self) -> &Self {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self
    }

    fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    fn lengthSq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn lengthManhattan(&self) -> f64 {
        (self.x).abs() + (self.y).abs() + (self.z).abs()
    }

    fn distanceToSquared(&self, v: &Self) -> f64 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;

        dx * dx + dy * dy + dz * dz
    }

    fn crossVectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.y * b.z - a.z * b.y;
        self.y = a.z * b.x - a.x * b.z;
        self.z = a.x * b.y - a.y * b.x;

        self
    }

    fn cross(&mut self, v: &Self) -> &Self {
        self.x = self.y * v.z - self.z * v.y;
        self.y = self.z * v.x - self.x * v.z;
        self.z = self.x * v.y - self.y * v.x;

        self
    }

    fn equals(&mut self, v: &Self) -> bool {
        self.x == v.x && self.y == v.y && self.z == v.z
    }

    fn min(&mut self, v: &Self) -> &Self {
        self.x = self.x.min(v.x);
        self.y = self.y.min(v.y);
        self.z = self.z.min(v.z);

        self
    }

    fn max(&mut self, v: &Self) -> &Self {
        self.x = self.x.max(v.x);
        self.y = self.y.max(v.y);
        self.z = self.z.max(v.z);

        self
    }
}




#[allow(dead_code)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn set(&mut self, x: f64, y: f64) -> &Self {
        self.x = x;
        self.y = y;
        self
    }
}

impl Vector for Vector2 {
    fn add_vectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        return self;
    }
    fn add(&mut self, a: &Self) -> &Self {
        self.x = a.x + self.x;
        self.y = a.y + self.y;
        return self;
    }
    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &Self {
        self.x = a.x - b.x;
        self.y = a.y - b.y;
        self
    }
    fn sub(&mut self, a: &Self) -> &Self {
        self.x = a.x - self.x;
        self.y = a.y - self.y;
        self
    }
    fn copy(&mut self, from: &Self) -> &Self {
        self.x = from.x.clone();
        self.y = from.y.clone();
        self
    }
    fn multiplyScalarVector<'a, 'b>(v: &'a mut Self, s: &'b f64) -> &'a Self {
        v.x = v.x * s;
        v.y = v.y * s;
        v
    }
    fn negate(&mut self) -> &Self {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
    fn dot(&self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y
    }
    fn lengthSq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    fn lengthManhattan(&self) -> f64 {
        (self.x).abs() + (self.y).abs()
    }
    fn distanceToSquared(&self, v: &Self) -> f64 {
        let dx = self.x - v.x;
        let dy = self.y - v.y;

        dx * dx + dy * dy
    }
    fn crossVectors(&mut self, a: &Self, b: &Self) -> &Self {
        let zero = 0.0;

        self.x = a.y * zero - zero * b.y;
        self.y = zero * b.x - a.x * zero;

        self
    }
    fn cross(&mut self, v: &Self) -> &Self {
        let zero = 0.0;
        self.x = self.y * zero - zero * v.y;
        self.y = zero * v.x - self.x * zero;

        self
    }
    fn equals(&mut self, v: &Self) -> bool {
        self.x == v.x && self.y == v.y
    }
    fn min(&mut self, v: &Self) -> &Self {
        self.x = self.x.min(v.x);
        self.y = self.y.min(v.y);

        self
    }

    fn max(&mut self, v: &Self) -> &Self {
        self.x = self.x.max(v.x);
        self.y = self.y.max(v.y);

        self
    }
}
