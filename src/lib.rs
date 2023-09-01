#![no_std]
use core::ops::*;

#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub struct Complex<T: Copy>{
    pub r: T,
    pub i: T
}

impl<T: Copy> From<(T, T)> for Complex<T>{
    fn from(value: (T, T)) -> Complex<T>{
        let (real, imag) = value;
        Complex { r: (real), i: (imag) }
    }
}

impl<T> Neg for Complex<T> 
where T: Neg<Output=T> + Copy {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T>{
        Complex { r: (-self.r), i: (-self.i) }
    }
}

impl<T> Add for Complex<T>
where T: Add<Output=T> + Copy {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        Complex { r: (self.r + rhs.r), i: (self.i + rhs.i) }
    }
}

impl<T> Sub for Complex<T>
where T: Sub<Output=T> + Copy {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Complex { r: (self.r - rhs.r), i: (self.i - rhs.i) }
    }
}

impl<T> Mul for Complex<T>
where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        Complex { 
            r: (self.r * rhs.r - self.i * rhs.i), 
            i: (self.r * rhs.i + self.i * rhs.r) 
        }
    }
}

impl<T> Div for Complex<T>
where T: Add<Output=T> + Sub<Output=T> + 
         Mul<Output=T> + Div<Output=T> + Copy {
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let denom = rhs.r * rhs.r + rhs.i * rhs.i;
        Complex { 
            r: (self.r * rhs.r + self.i * rhs.i)/denom, 
            i: (self.i * rhs.r - self.r * rhs.i)/denom
        }
    }
}

// Assign operators (+=, -=, *=, /=)

impl<T> AddAssign for Complex<T>
where T: AddAssign + Copy {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.i += rhs.i;
    }
}

impl<T> SubAssign for Complex<T>
where T: SubAssign + Copy {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.i -= rhs.i;
    }
}

impl<T> MulAssign for Complex<T>
where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T> DivAssign for Complex<T>
where T: Add<Output=T> + Sub<Output=T> + 
         Mul<Output=T> + Div<Output=T> + Copy {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use std::*;
    use super::*;

    // Tests -a a+b a-b a*b a/b
    #[test]
    fn basic_ops() {
        let a = Complex::from((1.0, 2.0));
        assert_eq!(a, Complex{r: 1.0, i: 2.0});
        assert_eq!(-a, Complex{r: -1.0, i: -2.0});
        assert_eq!(a+a, Complex{r: 2.0, i: 4.0});
        assert_eq!(a-a, Complex{r: 0.0, i: 0.0});
        assert_eq!(a*a, Complex{r: -3.0, i: 4.0});
        assert_eq!(a/a, Complex{r: 1.0, i: 0.0});
    }

    #[test]
    fn assign_ops() {
        let a_0 = Complex::from((1.0, 2.0));
        let mut a = a_0;
        a += a_0;
        assert_eq!(a, Complex{r: 2.0, i: 4.0});
        a -= a_0;
        assert_eq!(a, Complex{r: 1.0, i: 2.0});
        a *= a_0;
        assert_eq!(a, Complex{r: -3.0, i: 4.0});
        a /= a_0;
        assert_eq!(a, Complex{r: 1.0, i: 2.0});
    }
}
