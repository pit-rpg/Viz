use helpers::Nums;
use std::ops::{Sub};


pub trait Vector<T>

where T: Nums {
    fn new_zero() -> Self;
    fn new_one() -> Self;
    fn random() -> Self;
    fn randomize(&mut self) -> &mut Self;
    fn copy (&mut self, v: &Self) -> &mut Self;
    fn multiply_scalar(&mut self, s: T) -> &mut Self;
    fn length(&self) -> T;
    fn length_sq(&self) -> T;
    fn manhattan_length(&mut self) -> T;
    fn set_scalar(&mut self, s: T) -> &mut Self;
    fn add_scalar(&mut self, s: T) -> &mut Self;
    fn sub_scalar(&mut self, s: T) -> &mut Self;
    fn add(&mut self, v: &Self) -> &mut Self;
    fn sub(&mut self, v: &Self) -> &mut Self;
    fn multiply(&mut self, v: &Self) -> &mut Self;
    fn divide(&mut self, v: &Self) -> &mut Self;
    fn add_vectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn sub_vectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn multiply_vectors(&mut self, a: &Self, b: &Self) -> &mut Self;
    fn negate(&mut self) -> &mut Self;
    fn min(&mut self, v: &Self) -> &mut Self;
    fn max(&mut self, v: &Self) -> &mut Self;
    fn dot(&mut self, v: &Self) -> T;
    fn round(&mut self) -> &mut Self;
    fn floor(&mut self) -> &mut Self;
    fn ceil(&mut self) -> &mut Self;
    fn clamp (&mut self, min: &Self, max: &Self )-> &mut Self;
    fn lerp (&mut self, v: &Self,  alpha:T )-> &mut Self;
    fn zero () -> Self;
	fn equals(&self, v: &Self ) -> bool;
    // fn set_from_matrix_column (&mut self, m: &Matrix4<T>, index: usize ) -> &mut Self;
    fn from_array (&mut self, array: &[T] ) -> &mut Self;
    // fn apply_matrix_4 (&mut self, m: &Matrix4<T> ) -> &mut Self;
    // fn apply_matrix_3 (&mut self, m: &Matrix3<T> ) -> &mut Self;

    fn divide_scalar(&mut self, s: T) -> &mut Self {
        return self.multiply_scalar(T::one() / s);
    }

    fn normalize(&mut self) -> &mut Self {
        let mut l = self.length();
        if l == T::zero() {
            l = T::one()
        };
        self.divide_scalar(l);
        self
    }

    fn set_length(&mut self, length: T) -> &mut Self {
        self.normalize().multiply_scalar(length)
    }

    fn clamp_length (&mut self, min:T, max:T )-> &mut Self {
        let mut l = self.length();
        if l == T::zero() {l = T::one()};
        self.divide_scalar( l ).multiply_scalar(min.min( max.max(l)))
        // self.divide_scalar( l ).multiply_scalar(min(min1, max(max1, l)))
    }

    fn lerp_vectors (&mut self, v1: &Self, v2: &Self, alpha:T )-> &mut Self {
        self.sub_vectors( v2, v1 ).multiply_scalar( alpha ).add( v1 )
    }
}
