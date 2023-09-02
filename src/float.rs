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
    }
}

macro_rules! impl_sqrt {
  ($t: ty) => {
    fn sqrt(self) -> Complex<$t> {
      todo!();
    }
  }
}