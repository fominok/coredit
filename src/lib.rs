//! Crate providing `Buffer`: core part of your text editor
#![deny(missing_docs)]
mod buffer;
mod selections;
mod util;
pub use buffer::Buffer;
pub use ropey::Rope;
pub use selections::CursorDirection;
pub use selections::{Position, Selection};
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

// /// Selection with linked sources like `LineLength`
// /// which makes it context-aware and simplifies its usage
// pub struct BindedSelection<L: LineLength + Copy> {
//     wrapped_selection: Selection,
//     line_length: L,
// }

// impl<L: LineLength + Copy> BindedSelection<L> {
//     pub(crate) fn new(selection: Selection, line_length: L) -> Self {
//         BindedSelection {
//             wrapped_selection: selection,
//             line_length,
//         }
//     }

//     /// Check if the selection's length equals to 1.
//     pub fn is_point(&self) -> bool {
//         self.wrapped_selection.is_point()
//     }

//     /// Get `from` position
//     pub fn from(&self) -> BindedPosition<L> {
//         BindedPosition::new(self.wrapped_selection.from, self.line_length)
//     }

//     /// Get `to` position
//     pub fn to(&self) -> BindedPosition<L> {
//         BindedPosition::new(self.wrapped_selection.to, self.line_length)
//     }

//     /// Returns `from`, `to` pair consuming selection
//     pub fn coords(self) -> (BindedPosition<L>, BindedPosition<L>) {
//         (
//             BindedPosition::new(self.wrapped_selection.from, self.line_length),
//             BindedPosition::new(self.wrapped_selection.to, self.line_length),
//         )
//     }

//     /// Get cursor direction
//     pub fn cursor_direction(&self) -> CursorDirection {
//         self.wrapped_selection.cursor_direction
//     }
// }

// /// Position with linked sources like `LineLength`
// /// which makes it context-aware and simplifies its usage
// #[derive(Copy, Clone)]
// pub struct BindedPosition<L: LineLength + Copy> {
//     wrapped_position: Position,
//     line_length: L,
// }

// impl<L: LineLength + Copy> BindedPosition<L> {
//     pub(crate) fn new(position: Position, line_length: L) -> Self {
//         BindedPosition {
//             wrapped_position: position,
//             line_length,
//         }
//     }

//     /// Return a position which follows the callee.
//     /// Returns `None` if called for the last possible position in
//     /// buffer.
//     pub fn successor(&self) -> Option<Self> {
//         self.wrapped_position
//             .successor(self.line_length)
//             .map(|pos| Self::new(pos, self.line_length))
//     }

//     /// Return a position which is before the callee
//     /// Returns `None` if called for the beginning of buffer.
//     pub fn predecessor(&self) -> Option<Self> {
//         self.wrapped_position
//             .predecessor(self.line_length)
//             .map(|pos| Self::new(pos, self.line_length))
//     }

//     /// Get line coord
//     pub fn line(&self) -> usize {
//         self.wrapped_position.line.get()
//     }

//     /// Get col coord
//     pub fn col(&self) -> usize {
//         self.wrapped_position.col.get()
//     }

//     /// Check whether this position matches line end
//     pub fn is_line_end(&self) -> bool {
//         self.wrapped_position.is_line_end(self.line_length)
//     }
// }

// impl<L: LineLength + Copy> PartialEq for BindedPosition<L> {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.wrapped_position.eq(&rhs.wrapped_position)
//     }
// }

// impl<L: LineLength + Copy> PartialOrd for BindedPosition<L> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         self.wrapped_position.partial_cmp(&other.wrapped_position)
//     }
// }

/// Buffer's feedback for optimal redraws or any other case when full buffer
/// contents not needed
pub enum Delta<'a> {
    /// A selection identifiable by `old` moved into `new` state
    SelectionChanged {
        /// Old selection state
        old: Selection,
        /// New selection state
        new: Selection,
    },
    /// Line's contents changed
    LineChanged {
        /// Line index
        idx: usize,
        /// Line new content
        content: &'a str,
    },
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
