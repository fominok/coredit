//! Selections implementation
use super::Position;
use std::cmp::Ordering;
use std::collections::HashMap;

// implementation notes:
//
// selections are necessarily looks in one direction,
// thus there are no any two selection for on of which
// head is before tail and another with head after tail
//
// TOOD: maybe use BTreeMap with range to find a selection
// which will be intersected with a new one on `add_selection`

/// As selections within the buffer are not independent
/// (can be merged, for instance) this structure is aimed
/// to take special care of it
pub struct SelectionStorage {
    indexed_selections: HashMap<usize, HashMap<usize, Selection>>,
}

impl Default for SelectionStorage {
    fn default() -> Self {
        let mut lines_hm = HashMap::new();
        let mut cols_hm = HashMap::new();
        cols_hm.insert(0, Default::default());
        lines_hm.insert(0, cols_hm);
        SelectionStorage {
            indexed_selections: lines_hm,
        }
    }
}

impl SelectionStorage {
    pub fn add_selection(&mut self, s: Selection) {
        todo!()
    }
}

#[derive(Debug)]
pub enum CursorDirection {
    Forward,
    Backward,
}

impl Default for CursorDirection {
    fn default() -> Self {
        CursorDirection::Forward
    }
}

/// Selection simply is as pair of positions, which are
/// pairs of line/column values. Note that there is no
/// information about underlying text, words and even movements.
#[derive(Default, Debug)]
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

    pub fn new_quick(
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
}

struct SelectionIntersect(Selection);

impl Eq for SelectionIntersect {}

impl PartialEq for SelectionIntersect {
    fn eq(&self, rhs: &Self) -> bool {
        let x = &self.0;
        let y = &rhs.0;
        x.head <= y.tail && y.head <= x.tail
    }
}

impl PartialOrd for SelectionIntersect {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for SelectionIntersect {
    fn cmp(&self, rhs: &Self) -> Ordering {
        if self == rhs {
            // If Selections have an intersection -- they are equal
            // in terms of SelectionIntersect newtype
            Ordering::Equal
        } else {
            // Else they should be compared by heads
            self.0.head.cmp(&rhs.0.head)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_intersect_partial_eq_forward() {
        let a = Selection::new_quick(87, 7, 88, 8, Default::default());
        let b = Selection::new_quick(87, 97, 105, 35, Default::default());
        assert!(SelectionIntersect(a) == SelectionIntersect(b))
    }

    #[test]
    fn test_selection_intersect_partial_eq_backward() {
        let b = Selection::new_quick(87, 7, 88, 8, Default::default());
        let a = Selection::new_quick(87, 97, 105, 35, Default::default());
        assert!(SelectionIntersect(a) == SelectionIntersect(b))
    }

    #[test]
    fn test_selection_intersect_partial_eq_corner() {
        let a = Selection::new_quick(87, 7, 88, 8, Default::default());
        let b = Selection::new_quick(88, 8, 105, 35, Default::default());
        assert!(SelectionIntersect(a) == SelectionIntersect(b))
    }

    #[test]
    fn test_selection_intersect_ord_lt() {
        let a = Selection::new_quick(87, 7, 88, 8, Default::default());
        let b = Selection::new_quick(88, 9, 105, 35, Default::default());
        assert!(SelectionIntersect(a) < SelectionIntersect(b))
    }

    #[test]
    fn test_selection_intersect_ord_gt() {
        let b = Selection::new_quick(87, 7, 88, 8, Default::default());
        let a = Selection::new_quick(88, 9, 105, 35, Default::default());
        assert!(SelectionIntersect(a) > SelectionIntersect(b))
    }

    #[test]
    fn test_merge_forward() {}
}
