// mod vector;
//
use math::Vector;

#[derive(Clone, Debug, Copy)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

#[allow(dead_code)]
impl Vector2 {
    pub fn new (x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn set(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }
}

impl Vector for Vector2 {

    fn multiplyScalar(&mut self, s: f64) -> &mut Self {
        self.x *= s;
        self.y *= s;
        self
    }

    fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
        // return Math.sqrt( this.x * this.x + this.y * this.y + this.z * this.z );
    }

    fn lengthSq(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    fn manhattanLength(&mut self) -> f64 {
        (self.x).abs() + (self.y).abs()
    }


    fn setScalar(&mut self, s: f64) -> &mut Self {
        self.x = s;
        self.y = s;
        self
    }

    fn addScalar(&mut self, s: f64) -> &mut Self {
        self.x += s;
        self.y += s;
        self
    }

    fn subScalar(&mut self, s: f64) -> &mut Self {
        self.x -= s;
        self.y -= s;
        self
    }

    fn add(&mut self, v: &Self) -> &mut Self {
        self.x += v.x;
        self.y += v.y;
        self
    }

    fn sub(&mut self, v: &Self) -> &mut Self {
        self.x -= v.x;
        self.y -= v.y;
        self
    }

    fn multiply(&mut self, v: &Self) -> &mut Self {
        self.x *= v.x;
        self.y *= v.y;
        self
    }

    fn divide(&mut self, v: &Self) -> &mut Self {
        self.x /= v.x;
        self.y /= v.y;
        self
    }

    fn addVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x + b.x;
        self.y = a.y + b.y;
        self
    }

    fn subVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x - b.x;
        self.y = a.y - b.y;
        self
    }

    fn multiplyVectors(&mut self, a: &Self, b: &Self) -> &mut Self {
        self.x = a.x * b.x;
        self.y = a.y * b.y;
        self
    }

    fn negate(&mut self) -> &mut Self {
        self.x = -self.x;
        self.y = -self.y;
        self
    }

    fn min(&mut self, v: &Self) -> &mut Self {
        self.x = (self.x).min(v.x);
        self.y = (self.y).min(v.y);
        self
    }

    fn max(&mut self, v: &Self) -> &mut Self {
        self.x = (self.x).max(v.x);
        self.y = (self.y).max(v.y);
        self
    }

    fn dot(&mut self, v: &Self) -> f64 {
        self.x * v.x + self.y * v.y
    }

    fn round(&mut self) -> &mut Self {
        self.x = self.x.round();
        self.y = self.y.round();
        self
    }

    fn floor(&mut self) -> &mut Self {
        self.x = self.x.floor();
        self.y = self.y.floor();
        self
    }

    fn ceil(&mut self) -> &mut Self {
        self.x = self.x.ceil();
        self.y = self.y.ceil();
        self
    }

    fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self {
		self.x = min.x.max(max.x.min(self.x));
		self.y = min.y.max(max.y.min(self.y));
		self
    }

    fn lerp (&mut self, v: &Self,  alpha:f64 )-> &mut Self {
        self.x += ( v.x - self.x ) * alpha;
        self.y += ( v.y - self.y ) * alpha;
		self
    }

    fn zero() -> Self {
        Vector2{x:0.0,y:0.0}
    }
}
