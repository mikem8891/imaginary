use super::*;

pub const I: Complex<f32> = Complex{r: 0.0_f32, i: 1.0_f32};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_const_path(){
        assert_eq!(crate::f32::I, Complex{r: 0.0_f32, i: 1.0_f32});
    }
}