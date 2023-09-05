//! # Complex Number Library
//!
//! This complex number library provides a simple generic implementation of
//! complex numbers and a more comprehensive implementation of 
//! complex numbers for `f32` and `f64` types.  The generic implementation includes 
//! overloaded operators (`+`, `-`, `*`, and `/`).  The float specific implementations 
//! have overloaded operators for operations between floats and `Complex`.
//! 
//! # Examples
//! ```
//! use imaginary::Complex;
//! 
//! let x: f64 = 3.0;
//! let z = Complex::<f64>::new(0.0, -4.0);
//! assert_eq!(x / (x + z), Complex::<f64>::new(0.36, 0.48));
//! ```
//! 
//! Chosing a convenient type definition can make working with complex numbers as 
//! easy as floats.
//! 
//! # Examples
//! ```
//! use imaginary::Complex;
//! type C64 = Complex<f64>;
//! 
//! let z = 1.4 - 1.4 * C64::I;
//! assert_eq!(z, C64::new(1.4, -1.4));
//! 
//! let z = 2.0 * C64::cis(3.1);
//! assert_eq!(z, 2.0 * f64::cos(3.1) + 2.0 * f64::sin(3.1) * C64::I);
//! ```

use core::ops::*;

#[macro_use]
mod float;
mod c32;
mod c64;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Complex<T: Copy>{
    /// real
    pub r: T,
    /// imaginary
    pub i: T
}

impl<T: Copy> Complex<T>{
    pub fn new(real: T, imag:  T) -> Complex<T> {
        Complex { r: real, i: imag }
    }
}

impl<T: Copy> From<(T, T)> for Complex<T>{
    fn from(value: (T, T)) -> Complex<T>{
        let (real, imag) = value;
        Complex { r: (real), i: (imag) }
    }
}

impl<T: Copy> Into<(T, T)> for Complex<T> {
    fn into(self) -> (T, T) {
        (self.r, self.i)
    }
}

impl<T> Complex<T>
where T: Neg<Output=T> + Copy {
    pub fn conj(self) -> Complex<T> {
        Complex { r: (self.r), i: (-self.i) }
    }
}

impl<T> Neg for Complex<T> 
where T: Neg<Output=T> + Copy {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T>{
        Complex { r: (-self.r), i: (-self.i) }
    }
}

impl<T> Add for Complex<T>
where T: Add<Output=T> + Copy {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        Complex { r: (self.r + rhs.r), i: (self.i + rhs.i) }
    }
}

impl<T> Sub for Complex<T>
where T: Sub<Output=T> + Copy {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Complex { r: (self.r - rhs.r), i: (self.i - rhs.i) }
    }
}

impl<T> Mul for Complex<T>
where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        Complex { 
            r: (self.r * rhs.r - self.i * rhs.i), 
            i: (self.r * rhs.i + self.i * rhs.r) 
        }
    }
}

impl<T> Div for Complex<T>
where T: Add<Output=T> + Sub<Output=T> + 
         Mul<Output=T> + Div<Output=T> + Copy {
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let denom = rhs.r * rhs.r + rhs.i * rhs.i;
        Complex { 
            r: (self.r * rhs.r + self.i * rhs.i)/denom, 
            i: (self.i * rhs.r - self.r * rhs.i)/denom
        }
    }
}

// Assign operators (+=, -=, *=, /=)

impl<T> AddAssign for Complex<T>
where T: AddAssign + Copy {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl<T> SubAssign for Complex<T>
where T: SubAssign + Copy {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.i -= rhs.i;
    }
}

impl<T> MulAssign for Complex<T>
where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T> DivAssign for Complex<T>
where T: Add<Output=T> + Sub<Output=T> + 
         Mul<Output=T> + Div<Output=T> + Copy {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_const_path(){
        assert_eq!(Complex::<f32>::I, Complex{r: 0.0_f32, i: 1.0_f32});
        assert_eq!(Complex::<f64>::I, Complex{r: 0.0_f64, i: 1.0_f64});
    }

    // Tests -a a+b a-b a*b a/b
    #[test]
    fn basic_ops() {
        let a = Complex::from((1.0, 2.0));
        assert_eq!(a, Complex{r: 1.0, i: 2.0});
        let (b, c) = a.into();
        assert_eq!((b, c), (1.0, 2.0));
        assert_eq!(-a, Complex{r: -1.0, i: -2.0});
        assert_eq!(a+a, Complex{r: 2.0, i: 4.0});
        assert_eq!(a-a, Complex{r: 0.0, i: 0.0});
        assert_eq!(a*a, Complex{r: -3.0, i: 4.0});
        assert_eq!(a/a, Complex{r: 1.0, i: 0.0});

        assert_eq!(a.conj(), Complex{r: 1.0, i: -2.0});
    }

    #[test]
    fn assign_ops() {
        let a_0 = Complex::from((1.0, 2.0));
        let mut a = a_0;
        a += a_0;
        assert_eq!(a, Complex{r: 2.0, i: 4.0});
        a -= a_0;
        assert_eq!(a, Complex{r: 1.0, i: 2.0});
        a *= a_0;
        assert_eq!(a, Complex{r: -3.0, i: 4.0});
        a /= a_0;
        assert_eq!(a, Complex{r: 1.0, i: 2.0});
    }
}
