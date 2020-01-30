//! Selections implementation
mod storage;
use crate::util::PositiveUsize;
use crate::LineLengh;

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
}

impl Selection {
    pub fn new(head: Position, tail: Position, cursor_direction: CursorDirection) -> Self {
        Selection {
            head,
            tail,
            cursor_direction,
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
        }
    }

    /// If something was moved too much and became reversed
    /// let's fix head/tail and change direction
    fn fix_direction(&mut self) {
        if self.head > self.tail {
            std::mem::swap(&mut self.head, &mut self.tail);
            self.cursor_direction.inverse();
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
    pub(crate) fn move_left<T: LineLengh>(&mut self, mut n: usize, line_length: &T) {
        let cursor = self.get_cursor_mut();
        while n > 0 {
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
    }

    /// Move cursor right by n characters, handling line lengthes and buffer bounds
    pub(crate) fn move_right<T: LineLengh>(&mut self, mut n: usize, line_length: &T) {
        let cursor = self.get_cursor_mut();
        let mut fallback = *cursor;
        while n >= 0 {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::collections::HashMap;

    #[test]
    fn test_set_straight_ahead() {
        let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        forward.set(6, 45, true);
        assert_eq!(
            forward,
            Selection::new_quick(4, 10, 6, 45, CursorDirection::Forward)
        );

        let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        backward.set(4, 5, true);
        assert_eq!(
            backward,
            Selection::new_quick(4, 5, 6, 20, CursorDirection::Backward)
        );
    }

    #[test]
    fn test_set_shrink() {
        let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        forward.set(5, 20, true);
        assert_eq!(
            forward,
            Selection::new_quick(4, 10, 5, 20, CursorDirection::Forward)
        );

        let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        backward.set(5, 15, true);
        assert_eq!(
            backward,
            Selection::new_quick(5, 15, 6, 20, CursorDirection::Backward)
        );
    }

    #[test]
    fn test_set_reverse() {
        let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        forward.set(3, 30, true);
        assert_eq!(
            forward,
            Selection::new_quick(3, 30, 4, 10, CursorDirection::Backward)
        );

        let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        backward.set(6, 35, true);
        assert_eq!(
            backward,
            Selection::new_quick(6, 20, 6, 35, CursorDirection::Forward)
        );
    }

    #[test]
    fn test_non_expand() {
        let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        forward.set(6, 21, false);
        assert_eq!(
            forward,
            Selection::new_quick(6, 21, 6, 21, CursorDirection::Forward)
        );

        let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        backward.set(2, 5, false);
        assert_eq!(
            backward,
            Selection::new_quick(2, 5, 2, 5, CursorDirection::Forward)
        );
    }

    #[test]
    fn test_move_left_one_line() {
        let line_length = HashMap::new();
        let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        selection.move_left(5, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(4, 10, 6, 15, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_left_multiple_lines() {
        let mut line_length = HashMap::new();
        line_length.insert(6, 322);
        line_length.insert(5, 40);
        line_length.insert(4, 30);
        let mut selection = Selection::new_quick(2, 20, 6, 20, CursorDirection::Forward);
        selection.move_left(80, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(2, 20, 4, 10, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_left_multiple_lines_until_beginning() {
        let mut line_length = HashMap::new();
        line_length.insert(6, 322);
        line_length.insert(5, 40);
        line_length.insert(4, 30);
        line_length.insert(3, 30);
        line_length.insert(2, 30);
        line_length.insert(1, 30);
        let mut selection = Selection::new_quick(2, 20, 6, 20, CursorDirection::Backward);
        selection.move_left(1337, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(1, 1, 6, 20, CursorDirection::Backward),
        );
    }

    #[test]
    fn test_move_left_one_line_until_beginning() {
        let mut line_length = HashMap::new();
        line_length.insert(1, 322);
        let mut selection = Selection::new_quick(1, 20, 1, 70, CursorDirection::Backward);
        selection.move_left(1337, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(1, 1, 1, 70, CursorDirection::Backward),
        );
    }

    #[test]
    fn test_move_left_one_empty_line() {
        let mut line_length = HashMap::new();
        line_length.insert(1, 1);
        let mut selection = Selection::new_quick(1, 1, 1, 1, CursorDirection::Forward);
        selection.move_left(1337, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(1, 1, 1, 1, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_left_multiple_lines_reversed() {
        let mut line_length = HashMap::new();
        line_length.insert(6, 322);
        line_length.insert(5, 40);
        line_length.insert(4, 30);
        let mut selection = Selection::new_quick(5, 20, 6, 20, CursorDirection::Forward);
        selection.move_left(80, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(4, 10, 5, 20, CursorDirection::Backward),
        );
    }

    #[test]
    fn test_move_right_one_line() {
        let mut line_length = HashMap::new();
        line_length.insert(6, 50);
        let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        selection.move_right(5, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(4, 10, 6, 25, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_right_multiple_lines() {
        let mut line_length = HashMap::new();
        line_length.insert(6, 30);
        line_length.insert(7, 35);
        line_length.insert(8, 335);
        let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        selection.move_right(70, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(4, 10, 8, 25, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_right_multiple_lines_until_end() {
        let mut line_length = HashMap::new();
        line_length.insert(6, 30);
        line_length.insert(7, 35);
        line_length.insert(8, 335);
        let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        selection.move_right(700, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(4, 10, 8, 335, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_right_one_line_until_end() {
        let mut line_length = HashMap::new();
        line_length.insert(1, 50);
        let mut selection = Selection::new_quick(1, 10, 1, 20, CursorDirection::Forward);
        selection.move_right(500, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(1, 10, 1, 50, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_right_one_empty_line() {
        let mut line_length = HashMap::new();
        line_length.insert(1, 1);
        let mut selection = Selection::new_quick(1, 1, 1, 1, CursorDirection::Forward);
        selection.move_right(420, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(1, 1, 1, 1, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_right_multiple_lines_reversed() {
        let mut line_length = HashMap::new();
        line_length.insert(4, 30);
        line_length.insert(5, 80);
        line_length.insert(6, 30);
        line_length.insert(7, 35);
        line_length.insert(8, 335);
        let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        selection.move_right(140, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(6, 20, 7, 10, CursorDirection::Forward),
        );
    }

    #[test]
    fn test_move_right_one_in_the_end() {
        let mut line_length = HashMap::new();
        line_length.insert(1, 30);
        let mut selection = Selection::new_quick(1, 10, 1, 30, CursorDirection::Forward);
        selection.move_right(1, &line_length);
        assert_eq!(
            selection,
            Selection::new_quick(1, 10, 1, 30, CursorDirection::Forward),
        );
    }
}
