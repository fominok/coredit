//! Selections implementation
mod storage;
use crate::util::PositiveUsize;

// TODO:
// think of what to do if moved to the beginning of the line,
// perhaps it would be good to return remaining chars to go

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

    pub(crate) fn edit(&mut self, dline: isize, dcol: isize, extend: bool) {
        match self.cursor_direction {
            CursorDirection::Forward => {
                self.tail.line = self.tail.line.delta(dline);
                self.tail.col = self.tail.col.delta(dcol);
                if !extend {
                    self.head = self.tail;
                    self.cursor_direction = CursorDirection::Forward;
                }
            }
            CursorDirection::Backward => {
                self.head.line = self.head.line.delta(dline);
                self.head.col = self.head.col.delta(dcol);
                if !extend {
                    self.tail = self.head;
                    self.cursor_direction = CursorDirection::Forward;
                }
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

    #[test]
    fn test_movements_straight_ahead() {
        let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        forward.edit(0, 25, true);
        assert_eq!(
            forward,
            Selection::new_quick(4, 10, 6, 45, CursorDirection::Forward)
        );

        let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        backward.edit(0, -5, true);
        assert_eq!(
            backward,
            Selection::new_quick(4, 5, 6, 20, CursorDirection::Backward)
        );
    }

    #[test]
    fn test_movements_shrink() {
        let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        forward.edit(-1, 0, true);
        assert_eq!(
            forward,
            Selection::new_quick(4, 10, 5, 20, CursorDirection::Forward)
        );

        let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        backward.edit(1, 5, true);
        assert_eq!(
            backward,
            Selection::new_quick(5, 15, 6, 20, CursorDirection::Backward)
        );
    }

    #[test]
    fn test_movements_reverse() {
        let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        forward.edit(-3, 10, true);
        assert_eq!(
            forward,
            Selection::new_quick(3, 30, 4, 10, CursorDirection::Backward)
        );

        let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        backward.edit(2, 25, true);
        assert_eq!(
            backward,
            Selection::new_quick(6, 20, 6, 35, CursorDirection::Forward)
        );
    }

    #[test]
    fn test_non_expand() {
        let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
        forward.edit(0, 1, false);
        assert_eq!(
            forward,
            Selection::new_quick(6, 21, 6, 21, CursorDirection::Forward)
        );

        let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
        backward.edit(-2, -5, false);
        assert_eq!(
            backward,
            Selection::new_quick(2, 5, 2, 5, CursorDirection::Forward)
        );
    }
}
