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
            pub fn abs(self) -> $t {
                self.r.hypot(self.i)
            }

            /// Phase angle
            pub fn angle(self) -> $t {
                self.i.atan2(self.r)
            }

            /// Returns cos Θ + i sin Θ. Equivalent to e^(Θ i)
            pub fn cis(theta: $t) -> Complex<$t> {
                Complex::new(theta.cos(), theta.sin())
            }

            /// The exponential function, e^z
            pub fn exp(self) -> Complex<$t> {
                let r = self.r.exp();
                r * Complex::cis(self.i)
            }

            /// The natural logarithm
            pub fn ln(self) -> Complex<$t> {
                Complex::new(self.abs().ln(), self.angle())
            }

            /// Power, z^n where n is a float
            pub fn powf(self, n: $t) -> Complex<$t> {
                let r = self.abs().powf(n);
                let theta = n * self.angle();
                r * Complex::cis(theta)
            }
            /// Power, z^n where n is complex
            pub fn powc(self, n: Complex<$t>) -> Complex<$t> {
                (n * self.ln()).exp()
            }

            /// Square root, √z
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