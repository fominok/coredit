//! Code specific to individual selection
pub(crate) mod storage;
use crate::util::PositiveUsize;
use crate::LineLength;
#[cfg(test)]
mod tests;

/// A position in a text buffer represented by 1-based numbered
/// line and column
#[derive(PartialOrd, PartialEq, Ord, Eq, Default, Debug, Clone, Copy)]
pub struct Position {
    pub(crate) line: PositiveUsize,
    pub(crate) col: PositiveUsize,
}

/// For selection the head must be less than the tail, but
/// cursor position can be specified with CursorDirection.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CursorDirection {
    /// Tail is also a cursor
    Forward,
    /// Head is also a cursor
    Backward,
}

impl Default for CursorDirection {
    fn default() -> Self {
        CursorDirection::Forward
    }
}

impl CursorDirection {
    /// Get an opposite direction
    pub(crate) fn inverse(&mut self) {
        match self {
            CursorDirection::Forward => *self = CursorDirection::Backward,
            CursorDirection::Backward => *self = CursorDirection::Forward,
        }
    }
}

/// Selection is as pair of positions, which are pairs of line/column values.
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Selection {
    /// One of the selection's ends nearest to the buffer's beginning
    head: Position,
    /// One of the selection's ends nearest to the buffer's end
    tail: Position,
    /// One of the selection's ends is marked as a "cursor", if it's on the right,
    /// then selection's cursor direction is `Forward`.
    cursor_direction: CursorDirection,
    /// If after up/down movement the selection happened to get onto a line which is
    /// shorter than the previous one, then it will be placed in the line's and
    /// remembering its previous column as a "sticky column". If a subsequent
    /// up/down movement leads to a line longer than this value the sticky column
    /// will restore the selection's original column. Left/right movements will
    /// reset `sticky_column`.
    sticky_column: Option<PositiveUsize>,
}

impl Selection {
    /// Check if the selection's length equals to 1.
    pub(crate) fn is_point(&self) -> bool {
        self.head == self.tail
    }

    /// If something was moved too much and became reversed
    /// let's fix head/tail and change direction
    fn fix_direction(&mut self) {
        if self.head > self.tail {
            std::mem::swap(&mut self.head, &mut self.tail);
            self.cursor_direction.inverse();
        }
    }

    /// Left or right movement should drop sticky column
    fn drop_sticky(&mut self) {
        self.sticky_column = None;
    }

    /// If not extend then drop selection to 1-length
    fn drop_selection(&mut self) {
        match self.cursor_direction {
            CursorDirection::Forward => {
                self.head = self.tail;
                self.cursor_direction = CursorDirection::Forward;
            }
            CursorDirection::Backward => {
                self.tail = self.head;
                self.cursor_direction = CursorDirection::Forward;
            }
        }
    }

    /// Drop selection to 1-length but always to the beginning
    pub(crate) fn drop_selection_to_head(&mut self) {
        self.tail = self.head;
        self.cursor_direction = CursorDirection::Forward;
    }

    /// Get cursor reference
    pub(crate) fn get_cursor(&self) -> &Position {
        match self.cursor_direction {
            CursorDirection::Forward => &self.tail,
            CursorDirection::Backward => &self.head,
        }
    }

    /// Get positions pair references
    pub(crate) fn get_bounds(&self) -> (Position, Position) {
        (self.head, self.tail)
    }

    /// Get cursor mutable reference for inplace operations
    pub(crate) fn get_cursor_mut(&mut self) -> &mut Position {
        match self.cursor_direction {
            CursorDirection::Forward => &mut self.tail,
            CursorDirection::Backward => &mut self.head,
        }
    }

    /// Move selection right keeping its shape if possible.
    /// If it is a multiline selection then only head is affected.
    ///
    /// This is what happens on characters deletion before the selection.
    pub(crate) fn nudge_left(&mut self, n: usize) {
        self.head.col.sub_assign(n);
        if self.tail.line == self.head.line {
            self.tail.col.sub_assign(n);
        }
    }

    /// Move selection right keeping its shape if possible.
    /// If it is a multiline selection then only head is affected.
    ///
    /// This is what happens on characters insertion before the selection.
    pub(crate) fn nudge_right(&mut self, n: usize) {
        self.head.col.add_assign(n);
        if self.tail.line == self.head.line {
            self.tail.col.add_assign(n);
        }
    }

    /// Move selection up keeping its shape.
    /// No checks are made and lengths used as it is a helper method.
    ///
    /// This is what happens on newlines deletion above.
    pub(crate) fn nudge_up(&mut self, n: usize) {
        self.head.line.sub_assign(n);
        self.tail.line.sub_assign(n);
    }

    // Actions triggered by user directly (meaning "move_x" command, not a helper methods):

    /// Move cursor left by n characters, handling line lengthes and buffer bounds
    pub(crate) fn move_left<L: LineLength>(&mut self, mut n: usize, extend: bool, line_length: L) {
        let cursor = self.get_cursor_mut();
        loop {
            if n >= cursor.col.into() {
                if let Some(line_length) = line_length.length(cursor.line.get() - 1) {
                    n -= cursor.col.get();
                    cursor.col = line_length.into();
                    cursor.line.sub_assign(1);
                } else {
                    cursor.col = 1.into();
                    cursor.line = 1.into();
                    break;
                }
            } else {
                cursor.col.sub_assign(n);
                break;
            }
        }
        self.fix_direction();
        self.drop_sticky();
        if !extend {
            self.drop_selection();
        }
    }

    /// Move cursor right by n characters, handling line lengthes and buffer bounds
    pub(crate) fn move_right<L: LineLength>(&mut self, mut n: usize, extend: bool, line_length: L) {
        let cursor = self.get_cursor_mut();
        let mut fallback = *cursor;
        loop {
            if let Some(line_length) = line_length.length(cursor.line.get()) {
                let remaining = line_length - cursor.col.get();
                if n > remaining {
                    cursor.col.add_assign(remaining);
                    fallback = *cursor;
                    cursor.col = 1.into();
                    cursor.line.add_assign(1);
                    n -= remaining + 1;
                } else {
                    cursor.col.add_assign(n);
                    break;
                }
            } else {
                *cursor = fallback;
                break;
            }
        }
        self.fix_direction();
        self.drop_sticky();
        if !extend {
            self.drop_selection();
        }
    }

    /// Move cursor up by n lines, handling line lengthes and buffer bounds;
    /// If line is shorter, then previous column is preserved as sticky column
    /// and will be restored on enough lenth.
    pub(crate) fn move_up<L: LineLength>(&mut self, n: usize, extend: bool, line_length: L) {
        let current_sticky_column = self.sticky_column;
        let cursor = self.get_cursor_mut();
        cursor.line.sub_assign(n);
        if let Some(line_length) = line_length.length(cursor.line.get()) {
            if line_length < cursor.col.get() {
                let sticky_column = Some(cursor.col);
                cursor.col = line_length.into();
                self.sticky_column = sticky_column;
            } else if let Some(sticky_column) = current_sticky_column {
                cursor.col = sticky_column;
                self.sticky_column = None;
            }
        }
        self.fix_direction();
        if !extend {
            self.drop_selection();
        }
    }

    /// Move cursor down by n lines, handling line lengthes and buffer bounds;
    /// If line is shorter, then previous column is preserved as sticky column
    /// and will be restored on enough lenth.
    pub(crate) fn move_down<L: LineLength>(&mut self, n: usize, extend: bool, line_length: L) {
        let current_sticky_column = self.sticky_column;
        let cursor = self.get_cursor_mut();
        let target: usize = cursor.line.get() + n;
        let lines_count = line_length.count();
        if target > lines_count {
            cursor.line = lines_count.into();
        } else {
            cursor.line.add_assign(n);
        }
        if let Some(line_length) = line_length.length(cursor.line.get()) {
            if line_length < cursor.col.get() {
                let sticky_column = Some(cursor.col);
                cursor.col = line_length.into();
                self.sticky_column = sticky_column;
            } else if let Some(sticky_column) = current_sticky_column {
                cursor.col = sticky_column;
                self.sticky_column = None;
            }
        } else {
            cursor.line.sub_assign(1);
        }
        self.fix_direction();
        if !extend {
            self.drop_selection();
        }
    }

    // Helper methods related to testing:

    /// A shortcut to create Position instances in place.
    #[cfg(test)]
    pub(crate) fn new_quick(
        head_line: usize,
        head_col: usize,
        tail_line: usize,
        tail_col: usize,
        cursor_direction: CursorDirection,
    ) -> Self {
        Selection {
            head: Position {
                line: head_line.into(),
                col: head_col.into(),
            },
            tail: Position {
                line: tail_line.into(),
                col: tail_col.into(),
            },
            cursor_direction,
            sticky_column: None,
        }
    }

    /// Set sticky column to the selection.
    #[cfg(test)]
    pub(crate) fn with_sticky(mut self, sticky: usize) -> Self {
        self.sticky_column = Some(sticky.into());
        self
    }

    /// As movements can be complicated, setting, on the contrary,
    /// is an assignment of a cursor to an existing position
    #[cfg(test)]
    pub(crate) fn set(&mut self, line: usize, col: usize, extend: bool) {
        match self.cursor_direction {
            CursorDirection::Forward => {
                self.tail.line = line.into();
                self.tail.col = col.into();
                if !extend {
                    self.head = self.tail;
                    self.cursor_direction = CursorDirection::Forward;
                }
            }
            CursorDirection::Backward => {
                self.head.line = line.into();
                self.head.col = col.into();
                if !extend {
                    self.tail = self.head;
                    self.cursor_direction = CursorDirection::Forward;
                }
            }
        }
        self.fix_direction();
        self.drop_sticky();
    }
}

/// Selection of length 1 is simply a cursor thus can be
/// created from [Position](../struct.Position.html) of it
impl From<Position> for Selection {
    fn from(position: Position) -> Self {
        Selection {
            head: position,
            tail: position,
            cursor_direction: CursorDirection::Forward,
            sticky_column: None,
        }
    }
}
