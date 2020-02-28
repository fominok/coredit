//! Crate providing `Buffer`: core part of your text editor
#![deny(missing_docs)]
mod buffer;
mod selections;
mod util;
pub use buffer::Buffer;
use snafu::Snafu;
use std::io;

/// Crate's error type
#[derive(Debug, Snafu)]
pub enum Error {
    /// Failure on buffer creation from `Reader`
    #[snafu(display("Unable to create buffer with reader: {}", source))]
    CreateFromReader {
        /// Source error raised by Ropey
        source: io::Error,
    },
}

/// Result with crate's error type applied
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Trait to provide required information for selections computation.
///
/// The main idea is to granularly provide necessary information to each
/// component, so underlying text may be abstracted from selection
/// when only lengths of some lines are required.
///
/// For selections thests usage of this trait makes a rope creation and
/// filling unnecessary.
pub(crate) trait LineLength {
    /// Return the length of the line specified by `line`. Note the first
    /// line has the index equal 1.
    fn length(&self, line: usize) -> Option<usize>;

    /// Return the count of lines.
    fn count(&self) -> usize;
}

#[cfg(test)]
mod tests {
    //! For selections tests a commonly used implementor of `LineLength`
    //! is a `HashMap` as it provides and interface of setting some "length"
    //! to some "line".
    use super::*;
    use std::collections::HashMap;

    impl LineLength for HashMap<usize, usize> {
        fn length(&self, line: usize) -> Option<usize> {
            self.get(&line).map(|x| *x)
        }

        fn count(&self) -> usize {
            *self.keys().max().unwrap_or(&0)
        }
    }

    impl LineLength for &HashMap<usize, usize> {
        fn length(&self, line: usize) -> Option<usize> {
            (*self).length(line)
        }

        fn count(&self) -> usize {
            (*self).count()
        }
    }
}
