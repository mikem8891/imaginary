use super::*;

impl_complex_float_ops!(f64);

impl_complex_fn!(f64);

impl Complex<f64> {
    pub const I: Complex<f64> = Complex::<f64>{r: 0.0, i: 1.0};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_const_path(){
        assert_eq!(Complex::<f64>::I, Complex{r: 0.0_f64, i: 1.0_f64});
    }

    #[test]
    fn check_complex_float_ops(){
        let a: f64 = 2.0;
        let b: Complex<f64> = Complex::from((1.0, 3.0));
        assert_eq!(a + b, Complex{r: 3.0, i: 3.0});
        assert_eq!(b + a, Complex{r: 3.0, i: 3.0});
        assert_eq!(a - b, Complex{r: 1.0, i: -3.0});
        assert_eq!(b - a, Complex{r: -1.0, i: 3.0});
        assert_eq!(a * b, Complex{r: 2.0, i: 6.0});
        assert_eq!(b * a, Complex{r: 2.0, i: 6.0});
        assert_eq!(a / b, Complex{r: 0.2, i: -0.6});
        assert_eq!(b / a, Complex{r: 0.5, i: 1.5});
    }

}