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

pub(crate) trait LineLength {
    fn length(&self, line: usize) -> Option<usize>;
    fn count(&self) -> usize;
}

#[cfg(test)]
mod tests {
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
