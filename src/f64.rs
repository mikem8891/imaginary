use super::*;

pub const I: Complex<f64> = Complex{r: 0.0_f64, i: 1.0_f64};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_const_path(){
        assert_eq!(crate::f64::I, Complex{r: 0.0_f64, i: 1.0_f64});
    }
}