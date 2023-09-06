macro_rules! impl_complex_display {
    ($t: ty) => {
        impl std::fmt::Display for Complex<$t> {
            /// Provides simple display representation for the `Complex` struct
            /// 
            /// # Examples
            /// ```
            /// use imaginary::Complex;
            ///
            /// let num = format!("{}", Complex::<f32>{r: 3.0, i: 0.0});
            /// assert_eq!("3", num);
            /// let num = format!("{}", Complex::<f64>::I);
            /// assert_eq!("i", num);
            /// let num = format!("{}", -Complex::<f32>::I);
            /// assert_eq!("-i", num);
            /// let num = format!("{}", 4.0 * Complex::<f64>::I);
            /// assert_eq!("4*i", num);
            /// let num = format!("{}", -3.0 - Complex::<f32>::I);
            /// assert_eq!("-3 - i", num);
            /// let num = format!("{}", 3.0 - 4.0 * Complex::<f64>::I);
            /// assert_eq!("3 - 4*i", num);
            /// let num = format!("{}", -3.0 + Complex::<f32>::I);
            /// assert_eq!("-3 + i", num);
            /// let num = format!("{}", 3.0 + 4.0 * Complex::<f64>::I);
            /// assert_eq!("3 + 4*i", num);
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

macro_rules! impl_complex_float_ops {
    ($t: ty) => {
        impl Add<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn add(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex{
                    r: self + rhs.r,
                    i: rhs.i
                }
            }
        }
        impl Add<$t> for Complex<$t> {
            type Output = Complex<$t>;
            fn add(self, rhs: $t) -> Complex<$t> {
                Complex{
                    r: self.r + rhs,
                    i: self.i
                }
            }
        }
        impl Sub<Complex<$t>> for $t {
            type Output = Complex<$t>;
            fn sub(self, rhs: Complex<$t>) -> Complex<$t> {
                Complex{
                    r: self - rhs.r,
                    i: -rhs.i
                }
            }
        }
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

macro_rules! impl_complex_fn {
    ($t: ty) => {
        impl Complex<$t>{
            /// The absolute value or magnitude
            /// 
            /// # Examples
            /// 
            /// ```
            /// use imaginary::Complex;
            /// let z = Complex::<f32>{r: -3.0, i:  4.0};
            /// assert_eq!(z.abs(), 5.0);
            /// 
            /// let z = Complex::<f64>{r:  3.0, i: -4.0};
            /// assert_eq!(z.abs(), 5.0);
            /// ```
            pub fn abs(self) -> $t {
                self.r.hypot(self.i)
            }

            /// Returns a `Complex` in the same direction, but with a magnitude of 1 
            /// 
            /// # Examples
            /// 
            /// ```
            /// use imaginary::Complex;
            /// let z = Complex::<f32>{r: -3.0, i:  4.0};
            /// assert_eq!(z.sign(), Complex::<f32>::new(-0.6, 0.8));
            /// 
            /// let z = Complex::<f64>{r:  3.0, i: -4.0};
            /// assert_eq!(z.sign(), Complex::<f64>::new(0.6, -0.8));
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
            /// 
            /// let z = Complex::<f64>::new(5.0, -5.0);
            /// let theta = -(std::f64::consts::FRAC_PI_4); // -π/4
            /// assert_eq!(z.angle(), theta);
            /// ```
            pub fn angle(self) -> $t {
                self.i.atan2(self.r)
            }

            /// Returns cos φ + i sin φ.  Equivalent to e^(φ i)
            /// # Examples
            /// ```
            /// use imaginary::Complex;
            /// 
            /// let z = Complex::<f32>::cis(3.1);
            /// assert_eq!(z, f32::cos(3.1) + f32::sin(3.1) * Complex::<f32>::I);
            /// 
            /// let z = Complex::<f64>::cis(3.1);
            /// assert_eq!(z, f64::cos(3.1) + f64::sin(3.1) * Complex::<f64>::I);
            /// ```
            pub fn cis(theta: $t) -> Complex<$t> {
                Complex::new(theta.cos(), theta.sin())
            }

            /// The exponential function, e^z
            /// 
            /// # Examples
            /// ```
            /// use imaginary::Complex;
            /// 
            /// // Euler's identity
            /// // e^(π*i) + 1 = 0
            /// 
            /// let pi_i = std::f32::consts::PI * Complex::<f32>::I;
            /// assert!((pi_i.exp() + 1.0).abs() <= f32::EPSILON);
            /// 
            /// let pi_i = std::f64::consts::PI * Complex::<f64>::I;
            /// assert!((pi_i.exp() + 1.0).abs() <= f64::EPSILON);
            /// ```
            pub fn exp(self) -> Complex<$t> {
                let r = self.r.exp();
                r * Complex::<$t>::cis(self.i)
            }

            /// The natural logarithm
            /// 
            /// # Examples
            /// ```
            /// use imaginary::Complex;
            /// 
            /// let pi_i = std::f32::consts::PI * Complex::<f32>::I;
            /// assert_eq!(Complex::<f32>::new(-1.0, 0.0).ln(), pi_i);
            /// 
            /// let pi_i = std::f64::consts::PI * Complex::<f64>::I;
            /// assert_eq!(Complex::<f64>::new(-1.0, 0.0).ln(), pi_i);
            /// ```
            pub fn ln(self) -> Complex<$t> {
                Complex::new(self.abs().ln(), self.angle())
            }

            /// Power, z^n where n is a float
            pub fn powf(self, n: $t) -> Complex<$t> {
                let r = self.abs().powf(n);
                let theta = n * self.angle();
                r * Complex::<$t>::cis(theta)
            }
            /// Power, z^n where n is complex
            pub fn powc(self, n: Complex<$t>) -> Complex<$t> {
                (n * self.ln()).exp()
            }
            /// Square root, √x
            pub fn sqrt(x: f32) -> Complex<$t> {
                if x >= 0.0 {
                    Complex::new(x.sqrt(), 0.0)
                } else {
                    Complex::new(0.0, (-x).sqrt())
                }
            }
            /// Square root, √z
            /// 
            /// # Examples
            /// ```
            /// use imaginary::Complex;
            /// 
            /// let neg_1 = Complex::<f32>::new(-1.0, 0.0);
            /// assert_eq!(neg_1.sqrt(), Complex::<f32>::I);
            /// ```
            pub fn sqrt(self) -> Complex<$t> {
                let (x, y) = self.into();
                if y == 0.0 {
                    if x >= 0.0 {
                        Complex::new(x.sqrt(), 0.0)
                    } else {
                        Complex::new(0.0, (-x).sqrt())
                    }
                } else {
                    let r = self.abs();
                    let x_num = x + r;
                    let dom = 1.0 / (2.0 * x_num).sqrt();
                    Complex::new(x_num * dom, y * dom)
                }
            }
        }
    }
}