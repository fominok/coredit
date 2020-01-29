//! Utility structures not strongly connected to text editing
use derive_more::{Add, AddAssign, Display, From, Into};
use std::ops::{Sub, SubAssign};

#[derive(
    Add, Display, Into, Clone, Copy, Debug, PartialEq, Default, Eq, Ord, PartialOrd, AddAssign,
)]
pub struct PositiveUsize(usize);

impl Sub for PositiveUsize {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.0 <= other.0 {
            1.into()
        } else {
            (self.0 - other.0).into()
        }
    }
}

impl From<usize> for PositiveUsize {
    fn from(value: usize) -> Self {
        assert!(value > 0);
        PositiveUsize(value)
    }
}

impl PositiveUsize {
    pub fn new(value: usize) -> Self {
        assert!(value > 0);
        PositiveUsize(value)
    }

    pub fn sub_assign(&mut self, value: usize) {
        *self = *self - PositiveUsize(value as usize)
    }

    pub fn add_assign(&mut self, value: usize) {
        *self = *self + PositiveUsize(value as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let a: PositiveUsize = 228.into();
        let b = PositiveUsize::new(322);

        assert_eq!(a - b, PositiveUsize(1));
    }

    #[test]
    fn test_addition() {
        let a: PositiveUsize = 228.into();
        let b = PositiveUsize::new(322);

        assert_eq!(a + b, PositiveUsize(550));
    }
}
