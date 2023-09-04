use super::*;

pub const I: Complex<f32> = Complex{r: 0.0_f32, i: 1.0_f32};

impl_complex_float_ops!(f32);
impl_float_fn(f32);

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
        let b: Complex<f32> = Complex::from((1.0, 3.0));
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
        let a = Complex::from((3.0_f32,4.0_f32));
        assert_eq!(a.abs(), 5.0)
    }
}