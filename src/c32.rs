use super::*;

pub const I: Complex<f32> = Complex{r: 0.0_f32, i: 1.0_f32};

impl_complex_float_ops!(f32);

impl Complex<f32> {
    pub const I: Complex<f32> = Complex{r: 0.0, i: 1.0};
    impl_abs!(f32);
    impl_angle!(f32);
    impl_cis!(f32);
    impl_exp!(f32);
    impl_ln!(f32);
    impl_pow!(f32);
    impl_sqrt!(f32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_const_path(){
        assert_eq!(crate::c32::I, Complex{r: 0.0_f32, i: 1.0_f32});
    }
    
    #[test]
    fn check_complex_float_ops(){
        let a: f32 = 2.0;
        let b: Complex<f32> = Complex::new(1.0, 3.0);
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
    fn float_fn() {
        const PI: f32 = std::f32::consts::PI;
        let a = Complex::new(3.0, -4.0);
        assert_eq!(a.abs(), 5.0);
        let e_pi_i = Complex::new(0.0, PI).exp();
        assert!((e_pi_i + Complex::new(1.0, 0.0)).abs() <= f32::EPSILON);
        let a_sqrt = a.sqrt();
        assert_eq!(a_sqrt * a_sqrt, a);
        assert_eq!(Complex::new( 4.0, 0.0).sqrt(), Complex::new(2.0, 0.0));
        assert_eq!(Complex::new(-4.0, 0.0).sqrt(), Complex::new(0.0, 2.0));
    }
}