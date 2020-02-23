use super::{CursorDirection, Position, Selection};
use crate::LineLengh;
#[cfg(test)]
mod tests;

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Deref;

/// As selections within the buffer are not independent
/// (can be merged, for instance) this structure is aimed
/// to take special care of it
#[derive(Debug)]
pub(crate) struct SelectionStorage {
    // TODO: while the Tree is ok to store selections,
    // Rust std implementation's API is restrictive;
    // for instance, while doing insert it may be enough
    // to increment cursor positions which will not require
    // any rebuilds of the tree
    pub(crate) selections_tree: BTreeSet<SelectionIntersect>,
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

    pub(crate) fn add_selection(&mut self, ns: Selection) {
        if let Some(mut s) = self.find_hit_take(ns.head) {
            s.tail = ns.tail;
            // Here is a recursive call to verify that the new selection
            // has no overlaps
            self.add_selection(s);
        } else if let Some(mut s) = self.find_hit_take(ns.tail) {
            s.head = ns.head;
            self.add_selection(s);
        } else {
            self.selections_tree.insert(ns.into());
        }
    }

    fn find_hit_take(&mut self, s: Position) -> Option<Selection> {
        self.selections_tree
            .take(&Selection::from(s).into())
            .map(|si| si.0)
    }

    fn apply_to_selections<F>(&mut self, mut f: F)
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
        self.apply_to_selections(move |s| {
            s.move_left(n, extend, line_length.deref());
        });
    }

    pub(crate) fn move_right<L: LineLengh, D: Deref<Target = L>>(
        &mut self,
        n: usize,
        extend: bool,
        line_length: D,
    ) {
        self.apply_to_selections(move |s| {
            s.move_right(n, extend, line_length.deref());
        });
    }

    pub(crate) fn move_up<L: LineLengh, D: Deref<Target = L>>(
        &mut self,
        n: usize,
        extend: bool,
        line_length: D,
    ) {
        self.apply_to_selections(move |s| {
            s.move_up(n, extend, line_length.deref());
        });
    }

    pub(crate) fn move_down<L: LineLengh, D: Deref<Target = L>>(
        &mut self,
        n: usize,
        extend: bool,
        line_length: D,
    ) {
        self.apply_to_selections(move |s| {
            s.move_down(n, extend, line_length.deref());
        });
    }

    #[cfg(test)]
    fn find_hit(&self, s: Position) -> Option<&Selection> {
        self.selections_tree
            .get(&Selection::from(s).into())
            .map(|si| &si.0)
    }

    #[cfg(test)]
    pub(crate) fn gen_from_tuples(selections: &[SelectionQuick]) -> Self {
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

    pub(crate) fn iter(&self) -> impl DoubleEndedIterator<Item = Selection> + '_ {
        self.selections_tree.iter().map(|x| x.0.clone())
    }

    pub(crate) fn move_left_on_line(&mut self, line: usize, after: usize, n: usize) {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());

        let (on_the_line, others): (Vec<Selection>, Vec<Selection>) = selections_old
            .into_iter()
            .map(|x| x.0)
            .partition(|x| x.head.line == line.into() && x.head.col > after.into());
        for mut s in on_the_line {
            s.nudge_left(n);
            self.add_selection(s);
        }
        for s in others {
            self.add_selection(s);
        }
    }

    pub(crate) fn get_first_before(&self, after: &Selection) -> Option<Selection> {
        self.iter().rev().find(|s| s.head < after.head)
    }

    pub(crate) fn apply_delete<L: LineLengh, D: Deref<Target = L>>(
        &mut self,
        mut to_delete: Selection,
        line_length: D,
    ) {
        // Selections on the same line will be moved left on delta;
        // if delta includes deleted newlines, then the line after deleted newlines
        // will be appended to current and its selections will be moved left on chars delta
        // and all subsequent selection will be moved up

        let (from, to) = to_delete.get_bounds();
        to_delete.drop_selection_to_head();
        let to_line: usize = to.line.into();
        let to_col: usize = to.col.into();
        let from_col: usize = from.col.into();
        let from_line: usize = from.line.into();
        let chars_delta: usize = if to_line == from_line {
            to_col - from_col + 1
        } else {
            to_col
        };

        to_delete.drop_selection_to_head();
        self.replace_selection(to_delete.clone());

        let lines_delta = to_line - from_line
            + if to_col == line_length.length(to_line).unwrap() {
                1
            } else {
                0
            };

        self.move_left_on_line(to_line, to_col, chars_delta);

        if lines_delta > 0 {
            let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());

            let (selections_after, others): (Vec<Selection>, Vec<Selection>) = selections_old
                .into_iter()
                .map(|x| x.0)
                .partition(|x| x.head > to_delete.tail);

            let mut selections_after_iter = selections_after.into_iter();
            if let Some(mut first_after) = selections_after_iter.next() {
                first_after.nudge_up(lines_delta);
                first_after.nudge_right(from_col - 1);
                self.add_selection(first_after);
            }

            for mut s in selections_after_iter {
                s.nudge_up(lines_delta);
                self.add_selection(s);
            }

            for s in others {
                self.add_selection(s);
            }
        }
    }

    pub(crate) fn replace_selection(&mut self, to: Selection) {
        // TODO: perhaps we need some check here to verify that a replaced
        // selection won't overlap the next one
        self.selections_tree.replace(SelectionIntersect(to));
    }

    pub(crate) fn move_right_incremental(&mut self, n: usize) {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());

        let line_grouped = selections_old
            .into_iter()
            .map(|x| x.0)
            .group_by(|x| x.head.line);
        for (_, group) in &line_grouped {
            let mut offset = n;
            for mut s in group {
                if (s.cursor_direction == CursorDirection::Backward) || s.is_point() {
                    s.head.col += offset.into();
                }
                s.tail.col += offset.into();
                offset += n;
                self.add_selection(s);
            }
        }
    }

    pub(crate) fn move_down_incremental(&mut self, n: usize) {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());
        let mut offset = n;
        for mut s in selections_old.into_iter().map(|x| x.0) {
            if s.is_point() {
                s.head.line += offset.into();
                s.head.col = 1.into();
                s.tail.line += offset.into();
                s.tail.col = 1.into();
            } else if s.cursor_direction == CursorDirection::Backward {
                let col_diff = s.tail.col - s.head.col + 1.into();
                s.head.line += offset.into();
                s.head.col = 1.into();
                s.tail.line += offset.into();
                s.tail.col = col_diff;
            } else {
                s.head.line.add_assign(offset - n);
                s.tail.line += offset.into();
                s.tail.col = 1.into();
            }
            offset += n;
            self.add_selection(s);
        }
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
pub(crate) struct SelectionIntersect(pub(crate) Selection);

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
