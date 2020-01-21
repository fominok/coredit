//! Utility structures not strongly connected to text editing
use derive_more::{Add, Display, From, Into};
use std::ops::Sub;

#[derive(Add, Display, From, Into, Clone, Copy, Debug, PartialEq, Default)]
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

impl PositiveUsize {
    pub fn new(value: usize) -> Self {
        value.into()
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
