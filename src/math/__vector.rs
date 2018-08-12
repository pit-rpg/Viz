pub trait Vector {
    // fn clone(v: &Self) -> Self;
    fn multiplyScalar(&mut self, s: f64) -> &mut Self;
    fn length(&self) -> f64;
    fn lengthSq(&self) -> f64;
    fn manhattanLength(&mut self) -> f64;
    fn setScalar(&mut self, s: f64) -> &mut Self;
    fn addScalar(&mut self, s: f64) -> &mut Self;
    fn subScalar(&mut self, s: f64) -> &mut Self;
    fn add(&mut self, v: &Self) -> &mut Self;
    fn sub(&mut self, v: &Self) -> &mut Self;
    fn multiply(&mut self, v: &Self) -> &mut Self;
    fn divide(&mut self, v: &Self) -> &mut Self;
    fn addVectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn subVectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn multiplyVectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn negate(&mut self) -> &mut Self;
    fn min(&mut self, v: &Self) -> &mut Self;
    fn max(&mut self, v: &Self) -> &mut Self;
    fn dot(&mut self, v: &Self) -> f64;
    fn round(&mut self) -> &mut Self;
    fn floor(&mut self) -> &mut Self;
    fn ceil(&mut self) -> &mut Self;
    fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self;
    fn lerp (&mut self, v: &Self,  alpha:f64 )-> &mut Self;
    fn zero () -> Self;

    fn divideScalar(&mut self, s: f64) -> &mut Self {
        return self.multiplyScalar(1.0 / s);
    }

    fn normalize(&mut self) -> &mut Self {
        let mut l = self.length();
        if l == 0.0 {
            l = 1.0
        };
        self.divideScalar(l);
        self
    }

    fn setLength(&mut self, length: f64) -> &mut Self {
        self.normalize().multiplyScalar(length)
    }

    fn clampLength (&mut self, min:f64, max:f64 )-> &mut Self {
        let mut l = self.length();
        if l == 0.0 {l = 1.0};
        self.divideScalar( l ).multiplyScalar( min.min(max.max(l)) )
    }

    fn lerpVectors (&mut self, v1: &Self, v2: &Self, alpha:f64 )-> &mut Self {
        self.subVectors( v2, v1 ).multiplyScalar( alpha ).add( v1 )
    }
}
