//! # Complex Number Library
//!
//! This complex number library provides a simple generic implementation of
//! complex numbers and a more comprehensive implementation of complex numbers 
//! for [`f32`] and [`f64`] floating-point types.  Complex number functionality
//! is implemented by the [`Complex`] struct.  The generic implementation 
//! includes overloaded operators (`+`, `-`, `*`, and `/`).  The float specific 
//! implementations have overloaded operators for operations between floats and
//! [`Complex`] and additional functionality; such as, [`abs`](Complex::abs), 
//! [`sign`](Complex::sign), [`angle`](Complex::angle), [`cis`](Complex::cis), 
//! [`exp`](Complex::cis), [`powf`](Complex::powf), [`powc`](Complex::powc), 
//! and [`sqrt`](Complex::sqrt).
//! 
//! # Examples
//! ```
//! use imaginary::Complex;
//! 
//! let x = 3.0;
//! let z = Complex::new(0.0, -4.0);
//! assert_eq!(x / (x + z), Complex::new(0.36, 0.48));
//! ```
//! 
//! Choosing a convenient type definition can make working with complex numbers
//! as easy as floats.
//! 
//! # Examples
//! ```
//! use imaginary::{Complex, c32};
//! type C32 = Complex<f32>;
//! 
//! let z = 1.4 - 1.4 * c32::I;
//! assert_eq!(z, C32::new(1.4, -1.4));
//! 
//! let z = 2.0 * c32::cis(3.1);
//! assert_eq!(z, 2.0 * f32::cos(3.1) + 2.0 * f32::sin(3.1) * c32::I);
//! ```

use core::ops::*;

/// A struct for representing complex numbers
#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Complex<T: Copy>{
    /// real
    pub r: T,
    /// imaginary
    pub i: T
}

impl<T: Copy> Complex<T>{
    pub const fn new(real: T, imag:  T) -> Complex<T> {
        Complex { r: real, i: imag }
    }
}

impl<T: Copy> From<(T, T)> for Complex<T>{
    fn from(value: (T, T)) -> Complex<T>{
        let (real, imag) = value;
        Complex { r: (real), i: (imag) }
    }
}

impl<T: Copy> From<Complex<T>> for (T, T) {
    fn from(value: Complex<T>) -> (T, T) {
        (value.r, value.i)
    }
}

impl<T> Complex<T>
where T: Neg<Output=T> + Copy {
    /// Complex conjugate
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

macro_rules! impl_display_for_complex {
    ($t: ty) => {
        impl std::fmt::Display for Complex<$t> {
            /// Provides simple display representation for the `Complex` struct
            ///
            /// # Examples
            /// ```
            /// use imaginary::Complex;
            /// let num = format!("{}", Complex::new(3.0, -4.0));
            /// assert_eq!("3 - 4*i", num);
            /// ```
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if self.i == 0.0 {
                    write!(f, "{}", self.r)
                } else if self.r == 0.0 {
                    if self.i == 1.0 {
                        write!(f, "i")
                    } else if self.i == -1.0 {
                        write!(f, "-i")
                    } else {
                        write!(f, "{}*i", self.i)
                    }
                } else if self.i < 0.0 {
                    if self.i == -1.0 {
                        write!(f, "{} - i", self.r)
                    } else {
                        write!(f, "{} - {}*i", self.r, -self.i)
                    }
                } else {
                    if self.i == 1.0 {
                        write!(f, "{} + i", self.r)
                    } else {
                        write!(f, "{} + {}*i", self.r, self.i)
                    }
                }
            }
        }
    }
}
impl_display_for_complex!(f32);
impl_display_for_complex!(f64);

macro_rules! impl_from_float_for_complex {
    ($t: ty) => {
        impl From<$t> for Complex<$t> {
            fn from(value: $t) -> Self {
                Self{r: value, i: 0.0}
            }
        }
    }
}
impl_from_float_for_complex!(f32);
impl_from_float_for_complex!(f64);

macro_rules! impl_ops_for_complex {
    ($t: ty) => {
        /// # Examples
        /// ```
        /// use imaginary::Complex;
        /// let x = 1.0;
        /// let z = Complex::new(3.0, -2.0);
        /// assert_eq!(x + z, Complex::new(4.0, -2.0));
        /// ```
        impl Add<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn add(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex{
                    r: self + rhs.r,
                    i: rhs.i
                }
            }
        }
        /// # Examples
        /// ```
        /// use imaginary::Complex;
        /// let x = 1.0;
        /// let z = Complex::new(3.0, -2.0);
        /// assert_eq!(z + x, Complex::new(4.0, -2.0));
        /// ```
        impl Add<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn add(self, rhs: $t) -> Complex<$t> {
                Complex{
                    r: self.r + rhs,
                    i: self.i
                }
            }
        }
        /// # Examples
        /// ```
        /// use imaginary::Complex;
        /// let x = 1.0;
        /// let z = Complex::new(3.0, -2.0);
        /// assert_eq!(x - z, Complex::new(-2.0, 2.0));
        /// ```
        impl Sub<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn sub(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex{
                    r: self - rhs.r,
                    i: -rhs.i
                }
            }
        }
        /// # Examples
        /// ```
        /// use imaginary::Complex;
        /// let x = 1.0;
        /// let z = Complex::new(3.0, -2.0);
        /// assert_eq!(z - x, Complex::new(2.0, -2.0));
        /// ```
        impl Sub<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn sub(self, rhs: $t) -> Complex<$t> {
                Complex{
                    r: self.r - rhs,
                    i: self.i
                }
            }
        }
        impl Mul<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn mul(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex{
                    r: self * rhs.r,
                    i: self * rhs.i
                }
            }
        }
        impl Mul<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn mul(self, rhs: $t) -> Complex<$t> {
                Complex{
                    r: self.r * rhs,
                    i: self.i * rhs
                }
            }
        }
        impl Div<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn div(self, rhs: Complex<$t>) -> Complex<$t> {
                let denom = rhs.r * rhs.r + rhs.i * rhs.i;
                Complex{
                    r:  self * rhs.r / denom,
                    i: -self * rhs.i / denom
                }
            }
        }
        impl Div<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn div(self, rhs: $t) -> Complex<$t> {
                Complex{
                    r: self.r / rhs,
                    i: self.i / rhs
                }
            }
        }
    }
}
impl_ops_for_complex!(f32);
impl_ops_for_complex!(f64);

macro_rules! impl_complex {
    ($t: ident) => {
        impl Complex<$t>{
            /// The absolute value or magnitude
            ///
            /// # Examples
            ///
            /// ```
            /// use imaginary::Complex;
            /// let z = Complex::<f64>::new(-3.0, 4.0);
            /// assert_eq!(z.abs(), 5.0);
            /// ```
            pub fn abs(self) -> $t {
                self.r.hypot(self.i)
            }

            /// Returns a `Complex` in the same direction,
            /// but with a magnitude of 1
            ///
            /// # Examples
            ///
            /// ```
            /// use imaginary::Complex;
            /// let z = Complex::<f64>::new(-3.0, 4.0);
            /// assert_eq!(z.sign(), Complex::new(-0.6, 0.8));
            /// ```
            pub fn sign(self) -> Complex<$t> {
                self / self.abs()
            }

            /// Phase angle
            ///
            /// # Examples
            ///
            /// ```
            /// use imaginary::Complex;
            ///
            /// let z = Complex::<f32>::new(-5.0, 5.0);
            /// let theta = 3.0 * std::f32::consts::FRAC_PI_4; // 3π/4
            /// assert_eq!(z.angle(), theta);
            /// ```
            pub fn angle(self) -> $t {
                self.i.atan2(self.r)
            }

            /// Returns cos θ + i sin θ.  Equivalent to e^(θ i)
            /// # Examples
            /// ```
            /// use imaginary::{Complex, c32};
            ///
            /// let i = c32::I;
            /// let z = Complex::<f32>::cis(3.1);
            /// assert_eq!(z, f32::cos(3.1) + f32::sin(3.1) * i);
            /// ```
            pub fn cis(theta: $t) -> Complex<$t> {
                Complex::new(theta.cos(), theta.sin())
            }

            /// The exponential function, e^z
            ///
            /// # Examples
            /// ```
            /// use imaginary::{Complex, c32};
            ///
            /// // Euler's identity
            /// // e^(π*i) + 1 = 0
            ///
            /// let pi_i = std::f32::consts::PI * c32::I;
            /// assert!((pi_i.exp() + 1.0).abs() <= f32::EPSILON);
            /// ```
            pub fn exp(self) -> Complex<$t> {
                let r = self.r.exp();
                r * Complex::<$t>::cis(self.i)
            }

            /// The natural logarithm
            ///
            /// # Examples
            /// ```
            /// use imaginary::{Complex, c32};
            ///
            /// let pi_i = std::f32::consts::PI * c32::I;
            /// assert_eq!(Complex::<f32>::new(-1.0, 0.0).ln(), pi_i);
            /// ```
            pub fn ln(self) -> Complex<$t> {
                Complex::new(self.abs().ln(), self.angle())
            }

            /// Power, self^n where n is a float
            pub fn powf(self, n: $t) -> Complex<$t> {
                let r = self.abs().powf(n);
                let theta = n * self.angle();
                r * Complex::<$t>::cis(theta)
            }
            /// Power, self^n where n is complex
            pub fn powc(self, n: Complex<$t>) -> Complex<$t> {
                (n * self.ln()).exp()
            }

            /// Square root, √z
            pub fn sqrt(self) -> Complex<$t> {
                let (x, y) = self.into();
                if y == 0.0 {
                    if x >= 0.0 {
                        Complex::new(x.sqrt(), y)
                    } else {
                        Complex::new(0.0, (-x).sqrt().copysign(y))
                    }
                } else {
                    let r = self.abs();
                    let x_num = r + x;
                    if x_num != 0.0 {
                        use std::$t::consts::FRAC_1_SQRT_2;
                        let x_rt = x_num.sqrt();
                        Complex::new(x_rt, y / x_rt) * FRAC_1_SQRT_2
                    } else {
                        let x_rt = (-x).sqrt();
                        Complex::new(0.5 * y.abs() / x_rt, x_rt.copysign(y))
                    }
                }
            }
        }
    }
}
impl_complex!(f32);
impl_complex!(f64);


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_const_path(){
        assert_eq!(c32::I, Complex{r: 0.0_f32, i: 1.0_f32});
//        assert_eq!(Complex::<f64>::I, Complex{r: 0.0_f64, i: 1.0_f64});
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

macro_rules! complex_mod {
    ($m: ident for $t: ident) => {
        /// Functions in this module take floats and return [`Complex`] values
        pub mod $m {
            use std::assert_ne;
use super::*;

            /// The imaginary number, `I` = √-1
            ///
            /// `I` = i
            ///
            /// `I`² = -1
            pub const I: Complex<$t> = Complex::new(0.0, 1.0);
            // Square root of 3, `SQRT_3` = √3
            const SQRT_3: $t = 1.73205_08075_68877_29352_74463_41505_8724;
            /// The complex cube root of one, `CBRT_1` = ∛1
            ///
            /// `CBRT_1` = -1/2 + √3/2 i
            ///
            /// `CBRT_1`³ = 1
            pub const CBRT_1: Complex<$t> = Complex::new(-0.5, 0.5 * SQRT_3);

            /// Euler's formula
            ///
            /// `cis(θ)` = cos(θ) + i * sin(θ) = e^(θ*i)
            pub fn cis(theta: $t) -> Complex<$t> {
                Complex::<$t>::cis(theta)
            }

            /// Natural logarithm
            ///
            /// Returns a complex natural logarithm of any valid float.
            /// For the natural logarithm of a complex number use
            /// [`Complex::ln`]
            pub fn ln(exp: $t) -> Complex<$t> {
                const PI: $t = std::$t::consts::PI;
                if exp >= 0.0 {
                    Complex::new(exp.ln(), 0.0)
                } else {
                    Complex::new((-exp).ln(), PI)
                }
            }

            /// Square root
            ///
            /// Returns a complex square root of any valid float.
            /// For the square root of a complex number use
            /// [`Complex::sqrt`]
            pub fn sqrt(sq: $t) -> Complex<$t> {
                if sq >= 0.0 {
                    Complex::<$t>::new(sq.sqrt(), 0.0)
                } else {
                    Complex::<$t>::new(0.0, (-sq).sqrt())
                }
            }
            /// Quadratic roots
            ///
            /// Returns the complex roots of: a * x² + b * x + c = 0
            ///
            /// # Panics
            ///
            /// Panics if coefficients do not form a valid quadratic equation,
            /// that is if `a == 0.0` or any coefficients are `NAN`
            pub fn quad(a: $t, b: $t, c: $t) -> [Complex<$t>; 2] {
                assert!(!(a.is_nan() || b.is_nan() || c.is_nan()),
                    "NAN term, a = {a}, b = {b}, c = {c}"
                );
                assert_ne!(a, 0.0,
                    "Not a valid quadratic equation, 1st term = {a}"
                );
                let sqrt = sqrt(b * b - 4.0 * a * c);
                let dom = 0.5 / a;
                [(-b + sqrt) * dom, (-b - sqrt) * dom]
            }

            #[cfg(test)]
            mod test {
                use super::*;

                #[test]
                fn check_cis() {
                    let i: Complex<$t> = $m::I;
                    let z: Complex<$t> = $m::cis(3.1);
                    assert_eq!(z, $t::cos(3.1) + $t::sin(3.1) * i);
                }

                #[test]
                fn check_complex_float_ops(){
                    let a: $t = 2.0;
                    let b = Complex::<$t>::new(1.0, 3.0);
                    assert_eq!(a + b, Complex{r: 3.0, i: 3.0});
                    assert_eq!(b + a, Complex{r: 3.0, i: 3.0});
                    assert_eq!(a - b, Complex{r: 1.0, i: -3.0});
                    assert_eq!(b - a, Complex{r: -1.0, i: 3.0});
                    assert_eq!(a * b, Complex{r: 2.0, i: 6.0});
                    assert_eq!(b * a, Complex{r: 2.0, i: 6.0});
                    assert_eq!(a / b, Complex{r: 0.2, i: -0.6});
                    assert_eq!(b / a, Complex{r: 0.5, i: 1.5});
                }

                #[test]
                fn check_sqrt(){
                    let squares = [
                        Complex::new(-2.5, -2.5),
                        Complex::new( 0.5, -0.5),
                        Complex::new(-2.5,  2.5),
                        Complex::new( 0.5,  0.5),
                        Complex::new(-5.0,  0.0),
                        Complex::new( 5.0,  0.0),
                        Complex::new(-1.0, 0.5 *  $t::EPSILON.sqrt()),
                        Complex::new(-1.0, 0.5 * -$t::EPSILON.sqrt()),
                        Complex::new(-1.0, 2.0 *  $t::EPSILON.sqrt()),
                        Complex::new(-1.0, 2.0 * -$t::EPSILON.sqrt()),
                        Complex::new( 1.0, 0.5 *  $t::EPSILON.sqrt()),
                        Complex::new( 1.0, 0.5 * -$t::EPSILON.sqrt()),
                        Complex::new( 1.0, 2.0 *  $t::EPSILON.sqrt()),
                        Complex::new( 1.0, 2.0 * -$t::EPSILON.sqrt())
                    ];
                    println!("{}", $t::EPSILON);
                    for &sq in squares.iter() {
                        let sqrt = sq.sqrt();
                        let ep   = sq.abs() * $t::EPSILON;
                        let diff = ((sqrt * sqrt) - sq).abs();
                        println!("sq      = {sq}");
                        println!("sqrt^2  = {}", sqrt * sqrt);
                        println!("diff/ep = {}", diff / ep);
                        assert!(diff <= 2.0 * ep);
                    }
                }

                #[test]
                fn check_quad(){
                    let coeffs = [
                        [1.0, -4.0, 13.0],
                        [1.0, -2.0, 2.0],
                        [2.0, -2.0, 0.0],
                        [4.0, 0.0, -36.0]
                    ];
                    for coeff in coeffs {
                        let [a, b, c]= coeff;
                        println!("epsilon = {}", $t::EPSILON);
                        for &root in $m::quad(a, b, c).iter() {
                            let sum = a * root * root + b * root + c;
                            println!("root = {root}");
                            println!("sum = {sum}");
                            assert!(sum.abs() <= $t::EPSILON);
                        }
                    }
                }

                #[test]
                fn panic_quad() {
                    use std::panic;
                    let panics_coeffs = [
                        (true, [0.0, -4.0, 13.0]),
                        (true, [$t::NAN, -2.0, 2.0]),
                        (true, [1.0, $t::NAN, 2.0]),
                        (true, [1.0, -2.0, $t::NAN]),
                        (true, [$t::NAN, $t::NAN, $t::NAN]),
                        (false, [1.0, -2.0, 2.0])
                    ];
                    for panics_coeff in panics_coeffs {
                        let (should_panic, [a, b, c]) = panics_coeff;
                        let result = panic::catch_unwind(|| {
                            $m::quad(a, b, c)
                        });
                        assert!(should_panic == result.is_err());
                    }
                }
            }
        }
    }
}
complex_mod!(c32 for f32); // see macro above
complex_mod!(c64 for f64); // see macro above
