use super::{Position, Selection};
use crate::LineLengh;
#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Deref;

/// As selections within the buffer are not independent
/// (can be merged, for instance) this structure is aimed
/// to take special care of it
#[derive(Debug)]
pub(crate) struct SelectionStorage {
    selections_tree: BTreeSet<SelectionIntersect>,
}

#[cfg(test)]
impl PartialEq for SelectionStorage {
    fn eq(&self, rhs: &Self) -> bool {
        let self_vec: Vec<Selection> = self.selections_tree.iter().map(|x| x.0.clone()).collect();

        let rhs_vec: Vec<Selection> = rhs.selections_tree.iter().map(|x| x.0.clone()).collect();

        self_vec == rhs_vec
    }
}

#[cfg(test)]
type SelectionQuick = (usize, usize, usize, usize, bool);

impl SelectionStorage {
    /// For a fresh buffer there is only one selection in the beginning of it
    pub(crate) fn new() -> Self {
        let selection: Selection = Default::default();
        let mut tree = BTreeSet::new();
        tree.insert(SelectionIntersect(selection));

        SelectionStorage {
            selections_tree: tree,
        }
    }
    pub(crate) fn add_selection(&mut self, mut ns: Selection) {
        if let Some(s) = self.find_hit_take(ns.head) {
            ns.head = s.head;
            // Here is a recursive call to verify that the new selection
            // has no overlaps
            ns.drop_sticky();
            self.add_selection(ns);
        } else if let Some(s) = self.find_hit_take(ns.tail) {
            ns.tail = s.tail;
            ns.drop_sticky();
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

    fn move_selections_char<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Selection) -> (),
    {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());
        for s in selections_old {
            let mut selection = s.0;
            f(&mut selection);
            self.add_selection(selection);
        }
    }

    pub(crate) fn move_left<L: LineLengh, D: Deref<Target = L>>(
        &mut self,
        n: usize,
        extend: bool,
        line_length: D,
    ) {
        self.move_selections_char(move |s| {
            s.move_left(n, extend, line_length.deref());
        });
    }

    pub(crate) fn move_right<L: LineLengh, D: Deref<Target = L>>(
        &mut self,
        n: usize,
        extend: bool,
        line_length: D,
    ) {
        self.move_selections_char(move |s| {
            s.move_right(n, extend, line_length.deref());
        });
    }

    pub(crate) fn move_up<L: LineLengh, D: Deref<Target = L>>(
        &mut self,
        n: usize,
        extend: bool,
        line_length: D,
    ) {
        self.move_selections_char(move |s| {
            s.move_up(n, extend, line_length.deref());
        });
    }

    pub(crate) fn move_down<L: LineLengh, D: Deref<Target = L>>(
        &mut self,
        n: usize,
        extend: bool,
        line_length: D,
    ) {
        self.move_selections_char(move |s| {
            s.move_down(n, extend, line_length.deref());
        });
    }

    #[cfg(test)]
    pub(crate) fn gen_from_tuples(selections: &[SelectionQuick]) -> Self {
        use super::CursorDirection;

        let mut storage = SelectionStorage::new();
        let mut tree = BTreeSet::new();
        for s in selections {
            tree.insert(SelectionIntersect(Selection::new_quick(
                s.0,
                s.1,
                s.2,
                s.3,
                if s.4 {
                    CursorDirection::Forward
                } else {
                    CursorDirection::Backward
                },
            )));
        }
        storage.selections_tree = tree;

        storage
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
