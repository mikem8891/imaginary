use super::*;

pub const I: Complex<f32> = Complex{r: 0.0_f32, i: 1.0_f32};

impl_complex_float_ops!(f32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_const_path(){
        assert_eq!(crate::c32::I, Complex{r: 0.0_f32, i: 1.0_f32});
    }
    
    #[test]
    fn check_complex_float_ops(){
        let a = 1.0_f32;
        let b = Complex::from((2.0_f32, 3.0_f32));
        assert_eq!(a + b, Complex{r: 3.0_f32, i: 3.0_f32});
    }
}