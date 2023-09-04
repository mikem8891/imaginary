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

macro_rules! impl_abs {
    ($t: ty) => {
        pub fn abs(self) -> $t {
            self.r.hypot(self.i)
        }
    }
}

macro_rules! impl_angle {
    ($t: ty) => {
        pub fn angle(self) -> $t {
            self.i.atan2(self.r)
        }
    }
}

macro_rules! impl_cis {
    ($t: ty) => {
        pub fn cis(theta: $t) -> Complex<$t> {
            Complex::new(theta.cos(), theta.sin())
        }
    }
}

macro_rules! impl_exp {
    ($t: ty) => {
        pub fn exp(self) -> Complex<$t> {
            let r = self.r.exp();
            Complex::new(r * self.i.cos(), r * self.i.sin())
        }
    }
}

macro_rules! impl_sqrt {
    ($t: ty) => {
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