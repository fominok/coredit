//! Selections implementation
mod storage;
use crate::util::PositiveUsize;
use crate::LineLengh;

// TODO:
// think of what to do if moved to the beginning of the line,
// perhaps it would be good to return remaining chars to go
// or finally link it with text

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

#[derive(Debug, PartialEq)]
pub enum CursorDirection {
    Forward,
    Backward,
}

impl Default for CursorDirection {
    fn default() -> Self {
        CursorDirection::Forward
    }
}

impl CursorDirection {
    pub(crate) fn reverse(&mut self) {
        match self {
            CursorDirection::Forward => *self = CursorDirection::Backward,
            CursorDirection::Backward => *self = CursorDirection::Forward,
        }
    }
}

/// Selection simply is as pair of positions, which are
/// pairs of line/column values. Note that there is no
/// information about underlying text, words and even movements.
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

    // If something was moved too much and became reversed
    // let's fix head/tail and change direction
    fn fix_direction(&mut self) {
        if self.head > self.tail {
            std::mem::swap(&mut self.head, &mut self.tail);
            self.cursor_direction.reverse();
        }
    }

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

    pub(crate) fn get_cursor(&self) -> &Position {
        match self.cursor_direction {
            CursorDirection::Forward => &self.tail,
            CursorDirection::Backward => &self.head,
        }
    }

    pub(crate) fn get_cursor_mut(&mut self) -> &mut Position {
        match self.cursor_direction {
            CursorDirection::Forward => &mut self.tail,
            CursorDirection::Backward => &mut self.head,
        }
    }

    pub(crate) fn move_left<T: LineLengh>(&mut self, mut n: usize, line_length: &T) {
        let cursor = self.get_cursor_mut();
        while n > 0 {
            if n > cursor.col.into() {
                let line_length = line_length
                    .lengh(Into::<usize>::into(cursor.line) - 1)
                    .unwrap();
                n -= Into::<usize>::into(cursor.col);
                cursor.col = line_length.into();
                cursor.line.sub_assign(1);
            } else {
                cursor.col.sub_assign(n);
                break;
            }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_move_left_multiple_lines_until_beginning() {}

    #[test]
    fn test_move_left_one_line_until_beginning() {}

    #[test]
    fn test_move_left_one_empty_line() {}

    #[test]
    fn test_move_left_multiple_lines_reversed() {}
}
