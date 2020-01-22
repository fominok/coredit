//! Selections implementation
use super::Position;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// As selections within the buffer are not independent
/// (can be merged, for instance) this structure is aimed
/// to take special care of it
pub struct SelectionStorage {
    selections_tree: BTreeSet<SelectionIntersect>,
}

/// For a fresh buffer there is only one selection in the beginning of it
impl Default for SelectionStorage {
    fn default() -> Self {
        let selection: Selection = Default::default();
        let mut tree = BTreeSet::new();
        tree.insert(SelectionIntersect(selection));

        SelectionStorage {
            selections_tree: tree,
        }
    }
}

impl SelectionStorage {
    pub fn add_selection(&mut self, s: Selection) {
        todo!()
    }

    fn find_hit(&self, s: Position) -> Option<&Selection> {
        self.selections_tree
            .get(&Selection::from(s).into())
            .map(|si| &si.0)
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
}

/// Selection of length 1 is simply a cursor thus can be
/// created from `Position` of it
impl From<Position> for Selection {
    fn from(position: Position) -> Self {
        Selection {
            head: position,
            tail: position,
            cursor_direction: CursorDirection::Forward,
        }
    }
}

impl From<Selection> for SelectionIntersect {
    fn from(selection: Selection) -> Self {
        SelectionIntersect(selection)
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

    fn gen_storage() -> SelectionStorage {
        let mut storage: SelectionStorage = Default::default();
        let mut tree = BTreeSet::new();
        tree.insert(SelectionIntersect(Selection::new_quick(
            1,
            10,
            1,
            30,
            Default::default(),
        )));
        tree.insert(SelectionIntersect(Selection::new_quick(
            2,
            10,
            2,
            30,
            Default::default(),
        )));
        tree.insert(SelectionIntersect(Selection::new_quick(
            3,
            10,
            5,
            130,
            Default::default(),
        )));
        storage.selections_tree = tree;

        storage
    }

    #[test]
    fn test_selection_storage_search_some() {
        let storage = gen_storage();
        assert_eq!(
            *storage
                .find_hit(Position {
                    line: 3.into(),
                    col: 100.into()
                })
                .unwrap(),
            Selection {
                head: Position {
                    line: 3.into(),
                    col: 10.into()
                },
                tail: Position {
                    line: 5.into(),
                    col: 130.into()
                },
                cursor_direction: CursorDirection::Forward,
            }
        );
    }

    #[test]
    fn test_selection_storage_search_none() {
        let storage = gen_storage();
        assert!(storage
            .find_hit(Position {
                line: 2.into(),
                col: 50.into()
            })
            .is_none());
    }

    #[test]
    fn test_merge_forward() {}
}
