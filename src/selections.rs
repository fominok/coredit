//! Selections implementation
mod storage;
use crate::util::PositiveUsize;
use crate::LineLengh;
#[cfg(test)]
mod tests;

/// A position in a text buffer represented by 1-based numbered
/// line and column
#[derive(PartialOrd, PartialEq, Ord, Eq, Default, Debug, Clone, Copy)]
pub struct Position {
    line: PositiveUsize,
    col: PositiveUsize,
}

impl Position {
    fn new(line: PositiveUsize, col: PositiveUsize) -> Self {
        Position { line, col }
    }
}

/// For selection the head must be less than the tail, but
/// cursor position can be specified with CursorDirection.
#[derive(Debug, PartialEq)]
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

/// Selection simply is as pair of positions, which are
/// pairs of line/column values.
#[derive(Default, Debug, PartialEq)]
pub struct Selection {
    head: Position,
    tail: Position,
    cursor_direction: CursorDirection,
    sticky_column: Option<PositiveUsize>,
}

impl Selection {
    pub fn new(head: Position, tail: Position, cursor_direction: CursorDirection) -> Self {
        Selection {
            head: head,
            tail: tail,
            cursor_direction: cursor_direction,
            sticky_column: None,
        }
    }

    /// A shortcut to create Position instances in place.
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
            cursor_direction: cursor_direction,
            sticky_column: None,
        }
    }

    pub(crate) fn with_sticky(mut self, sticky: usize) -> Self {
        self.sticky_column = Some(sticky.into());
        self
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

    /// As movements can be complicated, setting, on the contrary,
    /// is an assignment of a cursor to an existing position
    pub(crate) fn set(&mut self, line: usize, col: usize, extend: bool) {
        match self.cursor_direction {
            CursorDirection::Forward => {
                self.tail.line = PositiveUsize::new(line);
                self.tail.col = PositiveUsize::new(col);
                if !extend {
                    self.head = self.tail;
                    self.cursor_direction = CursorDirection::Forward;
                }
            }
            CursorDirection::Backward => {
                self.head.line = PositiveUsize::new(line);
                self.head.col = PositiveUsize::new(col);
                if !extend {
                    self.tail = self.head;
                    self.cursor_direction = CursorDirection::Forward;
                }
            }
        }
        self.fix_direction();
        self.drop_sticky();
    }

    /// Get cursor reference
    pub(crate) fn get_cursor(&self) -> &Position {
        match self.cursor_direction {
            CursorDirection::Forward => &self.tail,
            CursorDirection::Backward => &self.head,
        }
    }

    /// Get cursor mutable reference for inplace operations
    pub(crate) fn get_cursor_mut(&mut self) -> &mut Position {
        match self.cursor_direction {
            CursorDirection::Forward => &mut self.tail,
            CursorDirection::Backward => &mut self.head,
        }
    }

    /// Move cursor left by n characters, handling line lengthes and buffer bounds
    pub(crate) fn move_left<T: LineLengh>(&mut self, mut n: usize, extend: bool, line_length: &T) {
        let cursor = self.get_cursor_mut();
        loop {
            if n > cursor.col.into() {
                if let Some(line_length) = line_length.lengh(Into::<usize>::into(cursor.line) - 1) {
                    n -= Into::<usize>::into(cursor.col);
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
    pub(crate) fn move_right<T: LineLengh>(&mut self, mut n: usize, extend: bool, line_length: &T) {
        let cursor = self.get_cursor_mut();
        let mut fallback = *cursor;
        loop {
            if let Some(line_length) = line_length.lengh(Into::<usize>::into(cursor.line)) {
                let remaining = line_length - Into::<usize>::into(cursor.col);
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
    pub(crate) fn move_up<T: LineLengh>(&mut self, n: usize, extend: bool, line_length: &T) {
        let current_sticky_column = self.sticky_column;
        let cursor = self.get_cursor_mut();
        cursor.line.sub_assign(n);
        if let Some(line_length) = line_length.lengh(Into::<usize>::into(cursor.line)) {
            if line_length < Into::<usize>::into(cursor.col) {
                let sticky_column = Some(cursor.col);
                cursor.col = line_length.into();
                self.sticky_column = sticky_column;
            } else {
                if let Some(sticky_column) = current_sticky_column {
                    cursor.col = sticky_column.into();
                    self.sticky_column = None;
                }
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
    pub(crate) fn move_down<T: LineLengh>(&mut self, n: usize, extend: bool, line_length: &T) {
        let current_sticky_column = self.sticky_column;
        let cursor = self.get_cursor_mut();
        let target: usize = Into::<usize>::into(cursor.line) + n;
        let lines_count = line_length.count();
        if target > lines_count {
            cursor.line = lines_count.into();
        } else {
            cursor.line.add_assign(n);
        }
        if let Some(line_length) = line_length.lengh(Into::<usize>::into(cursor.line)) {
            if line_length < Into::<usize>::into(cursor.col) {
                let sticky_column = Some(cursor.col);
                cursor.col = line_length.into();
                self.sticky_column = sticky_column;
            } else {
                if let Some(sticky_column) = current_sticky_column {
                    cursor.col = sticky_column.into();
                    self.sticky_column = None;
                }
            }
        } else {
            cursor.line.sub_assign(1);
        }
        self.fix_direction();
        if !extend {
            self.drop_selection();
        }
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
