//! Crate providing `Buffer`: core part of your text editor
// #![deny(missing_docs)]
mod buffer;
mod selections;
mod util;
pub use buffer::Buffer;
pub use ropey::Rope;
pub use selections::CursorDirection;
pub use selections::{Position, Selection};
use selections::{PositionUnbound, SelectionUnbound};
use std::io;

/// Crate's error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failure on buffer creation from `Reader`
    #[error("Unable to create buffer with reader: {0}")]
    CreateFromReader(#[from] io::Error),
}

/// Result with crate's error type applied
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Trait to provide required information for selections computation.
///
/// The main idea is to granularly provide necessary information to each
/// component, so underlying text may be abstracted from selection
/// when only lengths of some lines are required.
///
/// For selections tests usage of this trait makes a rope creation and
/// filling unnecessary.
pub trait LineLength {
    /// Return the length of the line specified by `line`. Note the first
    /// line has the index equal 1.
    fn line_length(&self, line: usize) -> Option<usize>;

    /// Return the count of lines.
    fn lines_count(&self) -> usize;
}

impl<T: LineLength + ?Sized> LineLength for &T {
    fn line_length(&self, line: usize) -> Option<usize> {
        (*self).line_length(line)
    }

    fn lines_count(&self) -> usize {
        (*self).lines_count()
    }
}

/// Buffer's feedback for optimal redraws or any other case when full buffer
/// contents not needed
#[derive(Debug, PartialEq)]
pub struct Delta<'a, 'b> {
    pub buffer: &'a Buffer,
    pub delta_type: DeltaType<'b>,
}

#[derive(Debug, PartialEq)]
pub enum DeltaType<'a> {
    /// A selection identifiable by `old` moved into `new` state
    SelectionChanged {
        identity: PositionUnbound,
        new_state: SelectionUnbound,
    },
    /// New selection added
    SelectionAdded {
        /// New selection
        selection: SelectionUnbound,
    },
    /// Selection was deleted
    SelectionDeleted {
        /// Deleted selection info
        identity: PositionUnbound,
    },
    /// Line's contents changed
    LineChanged {
        /// Line index
        idx: usize,
        /// Line new content
        content: &'a str,
    },
}

impl<'b> DeltaType<'b> {
    pub fn bind<'a>(self, buffer: &'a Buffer) -> Delta<'a, 'b> {
        Delta {
            buffer,
            delta_type: self,
        }
    }

    pub fn bind_vec<'a>(v: Vec<DeltaType<'b>>, buffer: &'a Buffer) -> Vec<Delta<'a, 'b>> {
        v.into_iter().map(|x| x.bind(buffer)).collect()
    }
}

#[cfg(test)]
mod tests {
    //! For selections tests a commonly used implementor of `LineLength`
    //! is a `HashMap` as it provides and interface of setting some "length"
    //! to some "line".
    use super::*;
    use std::collections::HashMap;

    impl LineLength for HashMap<usize, usize> {
        fn line_length(&self, line: usize) -> Option<usize> {
            self.get(&line).map(|x| *x)
        }

        fn lines_count(&self) -> usize {
            *self.keys().max().unwrap_or(&0)
        }
    }
}
