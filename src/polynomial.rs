use std::ops::{AddAssign, Mul, MulAssign};

trait Zero {
    fn zero() -> Self;
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for usize {
    fn zero() -> Self {
        0
    }
}

impl Zero for i32 {
    fn zero() -> Self {
        0
    }
}

#[derive(Clone, Debug)]
pub struct Polynomial<T>(pub Vec<T>);

impl<T> Polynomial<T> {
    pub fn new_degree1(c0: T) -> Self {
        Self(vec![c0])
    }

    pub fn new_degree2(c0: T, c1: T) -> Self {
        Self(vec![c1, c0])
    }
}

impl<T> Polynomial<T>
where
    T: Zero + PartialEq,
{
    pub fn degree(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .rev()
            .find_map(|(degree, coeffs)| (*coeffs == T::zero()).then(|| degree))
            .unwrap_or(0)
    }
}

impl<T> Mul<Self> for Polynomial<T>
where
    T: Mul<T, Output = T> + Copy + AddAssign<T> + Zero,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let mut out = vec![T::zero(); (self.0.len() - 1) + (other.0.len() - 1) + 1];
        for (degree0, coeff0) in self.0.iter().enumerate() {
            for (degree1, coeff1) in other.0.iter().enumerate() {
                out[degree0 + degree1] += *coeff0 * *coeff1;
            }
        }
        Polynomial(out)
    }
}

impl<T> MulAssign<T> for Polynomial<T>
where
    T: MulAssign<T> + Copy,
{
    fn mul_assign(&mut self, other: T) {
        for coeff in self.0.iter_mut() {
            *coeff *= other;
        }
    }
}

impl<T> Mul<T> for Polynomial<T>
where
    T: MulAssign<T> + Copy,
{
    type Output = Self;
    fn mul(mut self, other: T) -> Self::Output {
        self *= other;
        self
    }
}
