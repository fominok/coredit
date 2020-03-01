//! Utility structures not strongly connected to text editing
use derive_more::{Add, Display, Into};
use std::ops::Sub;

/// A helper wrapper that guarantees underlying `usize` is greater than 0.
/// If by creation or subtraction opposite happens, it will be equal to 1.
#[derive(Add, Display, Into, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct PositiveUsize(usize);

impl Default for PositiveUsize {
    fn default() -> Self {
        PositiveUsize(1)
    }
}

impl Sub for PositiveUsize {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        if self.0 <= other.0 {
            PositiveUsize(1)
        } else {
            PositiveUsize(self.0 - other.0)
        }
    }
}

impl From<usize> for PositiveUsize {
    fn from(value: usize) -> Self {
        PositiveUsize(if value > 0 { value } else { 1 })
    }
}

impl PositiveUsize {
    pub(crate) fn sub_assign(&mut self, value: usize) {
        if self.0 > value {
            self.0 -= value;
        } else {
            self.0 = 1;
        }
    }

    pub(crate) fn add_assign(&mut self, value: usize) {
        self.0 += value
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lower_bound() {
        let a: PositiveUsize = 228.into();
        let b: PositiveUsize = 322.into();

        assert_eq!(a - b, PositiveUsize(1));
    }

    #[test]
    fn test_addition() {
        let a: PositiveUsize = 228.into();
        let b: PositiveUsize = 322.into();

        assert_eq!(a + b, PositiveUsize(550));
    }
}
