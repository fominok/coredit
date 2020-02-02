use super::{Position, Selection};
use crate::LineLengh;
#[cfg(test)]
mod tests;

use std::cmp::Ordering;
use std::collections::BTreeSet;

/// As selections within the buffer are not independent
/// (can be merged, for instance) this structure is aimed
/// to take special care of it
#[derive(Debug)]
pub(crate) struct SelectionStorage<'a, L: LineLengh> {
    selections_tree: BTreeSet<SelectionIntersect>,
    line_length: &'a L,
}

pub struct MoveModifier {
    extend: bool,
    n: isize,
}

impl<'a, L: LineLengh> SelectionStorage<'a, L> {
    /// For a fresh buffer there is only one selection in the beginning of it
    pub fn new<'b: 'a>(line_length: &'b L) -> Self {
        let selection: Selection = Default::default();
        let mut tree = BTreeSet::new();
        tree.insert(SelectionIntersect(selection));

        SelectionStorage {
            selections_tree: tree,
            line_length: line_length,
        }
    }
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

    //pub fn move_selections_char(&mut self, m: MoveModifier) {
    //    todo!();
    //    let mut selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());
    //    let selections_new: BTreeSet<SelectionIntersect> = selections_old
    //        .into_iter()
    //        .map(|si| {})
    //        .collect();
    //    self.selections_tree = selections_new;
    //}
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
