use super::{CursorDirection, Position, Selection};

use std::cmp::Ordering;
use std::collections::BTreeSet;

/// As selections within the buffer are not independent
/// (can be merged, for instance) this structure is aimed
/// to take special care of it
#[derive(Debug)]
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
    pub fn add_selection(&mut self, mut ns: Selection) {
        if let Some(s) = self.find_hit_take(ns.head) {
            ns.head = s.head;
            // Here is a recursive call to verify that the new selection
            // has no overlaps
            self.add_selection(ns);
        } else if let Some(s) = self.find_hit_take(ns.tail) {
            ns.tail = s.tail;
            self.add_selection(ns);
        } else {
            self.selections_tree.insert(ns.into());
        }
    }

    fn find_hit(&self, s: Position) -> Option<&Selection> {
        self.selections_tree
            .get(&Selection::from(s).into())
            .map(|si| &si.0)
    }

    fn find_hit_take(&mut self, s: Position) -> Option<Selection> {
        self.selections_tree
            .take(&Selection::from(s).into())
            .map(|si| si.0)
    }
}

impl From<Selection> for SelectionIntersect {
    fn from(selection: Selection) -> Self {
        SelectionIntersect(selection)
    }
}

impl From<SelectionIntersect> for Selection {
    fn from(selection_intersect: SelectionIntersect) -> Self {
        selection_intersect.0
    }
}

#[derive(Debug)]
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
    fn test_merge_head() {
        let mut storage = gen_storage();
        let s = Selection::new_quick(2, 25, 2, 100, Default::default());
        storage.add_selection(s);

        // Unwrapped from newtype to provide intuitive comparison
        let selections_vec: Vec<Selection> = storage
            .selections_tree
            .into_iter()
            .map(|x| x.into())
            .collect();
        let selections_reference_vec = vec![
            Selection::new_quick(1, 10, 1, 30, Default::default()),
            Selection::new_quick(2, 10, 2, 100, Default::default()),
            Selection::new_quick(3, 10, 5, 130, Default::default()),
        ];

        assert_eq!(selections_vec, selections_reference_vec);
    }

    #[test]
    fn test_merge_tail() {
        let mut storage = gen_storage();
        let s = Selection::new_quick(2, 50, 4, 20, Default::default());
        storage.add_selection(s);

        // Unwrapped from newtype to provide intuitive comparison
        let selections_vec: Vec<Selection> = storage
            .selections_tree
            .into_iter()
            .map(|x| x.into())
            .collect();
        let selections_reference_vec = vec![
            Selection::new_quick(1, 10, 1, 30, Default::default()),
            Selection::new_quick(2, 10, 2, 30, Default::default()),
            Selection::new_quick(2, 50, 5, 130, Default::default()),
        ];

        assert_eq!(selections_vec, selections_reference_vec);
    }

    #[test]
    fn test_merge_miss() {
        let mut storage = gen_storage();
        let s = Selection::new_quick(2, 40, 3, 5, Default::default());
        storage.add_selection(s);

        // Unwrapped from newtype to provide intuitive comparison
        let selections_vec: Vec<Selection> = storage
            .selections_tree
            .into_iter()
            .map(|x| x.into())
            .collect();
        let selections_reference_vec = vec![
            Selection::new_quick(1, 10, 1, 30, Default::default()),
            Selection::new_quick(2, 10, 2, 30, Default::default()),
            Selection::new_quick(2, 40, 3, 5, Default::default()),
            Selection::new_quick(3, 10, 5, 130, Default::default()),
        ];

        assert_eq!(selections_vec, selections_reference_vec);
    }

    #[test]
    fn test_merge_both() {
        let mut storage = gen_storage();
        let s = Selection::new_quick(2, 20, 3, 20, Default::default());
        storage.add_selection(s);

        // Unwrapped from newtype to provide intuitive comparison
        let selections_vec: Vec<Selection> = storage
            .selections_tree
            .into_iter()
            .map(|x| x.into())
            .collect();
        let selections_reference_vec = vec![
            Selection::new_quick(1, 10, 1, 30, Default::default()),
            Selection::new_quick(2, 10, 5, 130, Default::default()),
        ];

        assert_eq!(selections_vec, selections_reference_vec);
    }
}
