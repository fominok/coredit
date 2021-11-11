//! Selections storage API with an implementation respecting multiple selections
//! interaction.
use super::{CursorDirection, PositionRaw, SelectionRaw};
use crate::{Buffer, Delta, LineLength};
#[cfg(test)]
mod tests;

use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BTreeSet;

/// Unbinded selection deltas.
/// Public API's Delta requires binded selections so this structure
/// helps to postpone addition of dependency on the Buffer
pub(crate) struct PartialSelectionDeltas {
    selection_pairs: Vec<(SelectionRaw, SelectionRaw)>,
}

impl PartialSelectionDeltas {
    /// Wrap selection pairs
    pub(crate) fn new(selection_pairs: Vec<(SelectionRaw, SelectionRaw)>) -> Self {
        Self { selection_pairs }
    }

    /// Bind partial selection deltas to the buffer
    pub(crate) fn bind<'a>(self, buffer: &'a Buffer) -> Vec<Delta<'a>> {
        self.selection_pairs
            .into_iter()
            .map(|(old, new)| Delta::SelectionChanged {
                old: old.binded(buffer),
                new: new.binded(buffer),
            })
            .collect()
    }
}

/// As selections within the buffer are not independent
/// (can be merged, for instance) this structure is aimed
/// to take special care of it
#[derive(Debug)]
pub(crate) struct SelectionStorage {
    // TODO: while the Tree is ok to store selections,
    // Rust std implementation's API is restrictive;
    // for instance, while doing insert it may be enough
    // to increment cursor positions which will not require
    // any rebuilds of the tree.
    //
    // Also it relies on SelectionIntersect wrapper which Ord
    // implementation does not respect antisymmetry outside
    // current SelectionStorage implementation which inserts
    // twice to eleminate overlaps.
    pub(crate) selections_tree: BTreeSet<SelectionIntersect>,

    /// One of selections must be marked as `main selection`.
    /// As selection is atomic it should not carry such information
    /// as there is a contract that only one selection in the storage
    /// can be `main` thus such information should be lifted to the
    /// storage.
    ///
    /// Here are some rules:
    /// 1. on merge, if one selection was marked as `main`, resulting
    ///    selection becomes `main`;
    /// 2. on placing selections under existing ones, the new `main`
    ///    is that selection which was placed under previous `main`
    /// 3. if creating selections from a bigger one (search, split)
    ///    the last created selection is the new `main` (not sure if
    ///    it sounds as it might be better to mark `main` the last one
    ///    within parts created from a bigger `main` not overall last,
    ///    but it is how Kakoune does)
    main_selection_ptr: PositionRaw,
}

impl SelectionStorage {
    /// For a fresh buffer there is only one selection in the beginning of it
    pub(crate) fn new() -> Self {
        let selection: SelectionRaw = Default::default();
        let from = selection.from;
        let mut tree = BTreeSet::new();
        tree.insert(SelectionIntersect(selection));

        SelectionStorage {
            selections_tree: tree,
            main_selection_ptr: from,
        }
    }

    /// Add a selection to the storage.
    /// If storage contains a selection which overlaps with the input
    /// they will be merged. This check is run twice: for head and for
    /// tail.
    pub(crate) fn add_selection(&mut self, ns: SelectionRaw) {
        if let Some(mut s) = self.find_hit_take(ns.from) {
            if self.main_selection_ptr == ns.from {
                self.main_selection_ptr = s.from;
            }
            s.to = ns.to;
            // Here is a recursive call to verify that the new selection
            // has no overlaps
            self.add_selection(s);
        } else if let Some(mut s) = self.find_hit_take(ns.to) {
            if self.main_selection_ptr == s.from {
                self.main_selection_ptr = ns.from;
            }
            s.from = ns.from;
            self.add_selection(s);
        } else {
            self.selections_tree.insert(ns.into());
        }
    }

    /// Finds a selection which covers input position and moves it out of the storage.
    fn find_hit_take(&mut self, s: PositionRaw) -> Option<SelectionRaw> {
        self.selections_tree
            .take(&SelectionRaw::from(s).into())
            .map(|si| si.0)
    }

    /// Apply functions to each of selections making a new tree in place of the old one.
    fn apply_to_selections<'a, F>(&mut self, f: F) -> PartialSelectionDeltas
    where
        F: Fn(SelectionRaw) -> SelectionRaw,
    {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());
        let mut delta_selection_pairs = Vec::with_capacity(selections_old.len());
        for s in selections_old {
            let old = s.0.clone();
            let new = f(s.0);
            self.add_selection(new.clone());
            delta_selection_pairs.push((old, new));
        }
        PartialSelectionDeltas::new(delta_selection_pairs)
    }

    /// Swap selections' cursor.
    pub(crate) fn swap_cursor(&mut self) -> PartialSelectionDeltas {
        self.apply_to_selections(move |s| s.swap_cursor())
    }

    /// Move left all selections.
    pub(crate) fn move_left(
        &mut self,
        n: usize,
        extend: bool,
        line_length: &impl LineLength,
    ) -> PartialSelectionDeltas {
        self.apply_to_selections(move |s| s.move_left(n, extend, line_length))
    }

    /// Move right all selections.
    pub(crate) fn move_right(
        &mut self,
        n: usize,
        extend: bool,
        line_length: &impl LineLength,
    ) -> PartialSelectionDeltas {
        self.apply_to_selections(move |s| s.move_right(n, extend, line_length))
    }

    /// Move up all selections.
    pub(crate) fn move_up(
        &mut self,
        n: usize,
        extend: bool,
        line_length: &impl LineLength,
    ) -> PartialSelectionDeltas {
        self.apply_to_selections(move |s| s.move_up(n, extend, line_length))
    }

    /// Move down all selections.
    pub(crate) fn move_down(
        &mut self,
        n: usize,
        extend: bool,
        line_length: &impl LineLength,
    ) -> PartialSelectionDeltas {
        self.apply_to_selections(move |s| s.move_down(n, extend, line_length))
    }

    /// Create an iterator
    pub(crate) fn iter(&self) -> impl DoubleEndedIterator<Item = SelectionRaw> + '_ {
        self.selections_tree.iter().map(|x| x.0.clone())
    }

    /// Nudge all selections on `line` after specific column left.
    pub(crate) fn move_left_on_line(&mut self, line: usize, after: usize, n: usize) {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());

        let (on_the_line, others): (Vec<SelectionRaw>, Vec<SelectionRaw>) = selections_old
            .into_iter()
            .map(|x| x.0)
            .partition(|x| x.from.line == line.into() && x.from.col > after.into());
        for mut s in on_the_line {
            s.nudge_left(n);
            self.add_selection(s);
        }
        for s in others {
            self.add_selection(s);
        }
    }

    /// Get a left neighbour for the selection.
    /// It will be `None` if called for the first selection in the buffer.
    pub(crate) fn get_first_before(&self, before: &SelectionRaw) -> Option<SelectionRaw> {
        self.iter().rev().find(|s| s.to < before.from)
    }

    // /// Get a right neighbour for the selection.
    // /// It will be `None` if called for the last selection in the buffer.
    // pub(crate) fn get_first_after(&self, after: &SelectionRaw) -> Option<SelectionRaw> {
    //     self.iter().find(|s| s.from > after.to)
    // }

    /// Compute selection storage after the selection deletion.
    pub(crate) fn apply_delete<L: LineLength>(
        &mut self,
        mut to_delete: SelectionRaw,
        line_length: L,
    ) {
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

        // As all API operations are atomic there must be no inconsistency between
        // selections and text after each operation
        let lines_delta = to_line - from_line
            + if to_col
                == line_length
                    .line_length(to_line)
                    .expect("Selection reached inconsistency")
            {
                1
            } else {
                0
            };

        self.move_left_on_line(to_line, to_col, chars_delta);

        if lines_delta > 0 {
            let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());

            let (selections_after, others): (Vec<SelectionRaw>, Vec<SelectionRaw>) = selections_old
                .into_iter()
                .map(|x| x.0)
                .partition(|x| x.from > to_delete.to);

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

    /// Place a new selection under each existing one with the same columns if it will fit the line.
    /// If the next line is too short to put a selection then it will use matching subsequent line.
    pub(crate) fn place_selection_under<L: LineLength + Clone>(&mut self, line_length: L) {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());
        for s in selections_old.into_iter().map(|x| x.0) {
            if let Some(selection_under) = s.create_selection_under(line_length.clone()) {
                if self.main_selection_ptr == s.from {
                    self.main_selection_ptr = selection_under.from;
                }
                self.add_selection(selection_under);
            }
            self.add_selection(s);
        }
    }

    /// Find a selection that overlaps with the input selection and replace it.
    /// Used to shrink the selection to be cursor-sized.
    pub(crate) fn replace_selection(&mut self, to: SelectionRaw) {
        // TODO: perhaps we need some check here to verify that a replaced
        // selection won't overlap the next one
        self.selections_tree.replace(SelectionIntersect(to));
    }

    /// Move selections right, accumulating movement from a previous selection,
    /// independently for each line.
    /// For instance, if `n = 3`, then first selection will be moved right by 3,
    /// next -- by 6, then by 9 and so on; other lines start with 3 too.
    pub(crate) fn move_right_incremental(&mut self, n: usize) {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());

        let line_grouped = selections_old
            .into_iter()
            .map(|x| x.0)
            .group_by(|x| x.from.line);
        for (_, group) in &line_grouped {
            let mut offset = n;
            for mut s in group {
                if (s.cursor_direction == CursorDirection::Backward) || s.is_point() {
                    s.from.col.add_assign(offset);
                }
                s.to.col.add_assign(offset);
                offset += n;
                self.add_selection(s);
            }
        }
    }

    /// Move selections down, accumulating movement from a previous selection.
    /// For instance, if `n = 3`, then first selection will be moved down by 3,
    /// next -- by 6, then by 9 and so on.
    pub(crate) fn move_down_incremental(&mut self, n: usize) {
        let selections_old = std::mem::replace(&mut self.selections_tree, BTreeSet::new());
        let mut offset = n;
        for mut s in selections_old.into_iter().map(|x| x.0) {
            if s.is_point() {
                s.from.line.add_assign(offset);
                s.from.col = 1.into();
                s.to.line.add_assign(offset);
                s.to.col = 1.into();
            } else if s.cursor_direction == CursorDirection::Backward {
                let col_diff = s.to.col - s.from.col + 1.into();
                s.from.line.add_assign(offset);
                s.from.col = 1.into();
                s.to.line.add_assign(offset);
                s.to.col = col_diff;
            } else {
                s.from.line.add_assign(offset - n);
                s.to.line.add_assign(offset);
                s.to.col = 1.into();
            }
            offset += n;
            self.add_selection(s);
        }
    }

    // Test related stuff:

    #[cfg(test)]
    fn find_hit(&self, s: PositionRaw) -> Option<&SelectionRaw> {
        self.selections_tree
            .get(&SelectionRaw::from(s).into())
            .map(|si| &si.0)
    }

    #[cfg(test)]
    pub(crate) fn gen_from_tuples(selections: &[SelectionQuick]) -> Self {
        let mut storage = SelectionStorage::new();
        let mut tree = BTreeSet::new();
        for s in selections {
            tree.insert(SelectionIntersect(SelectionRaw::new_quick(
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

impl From<SelectionRaw> for SelectionIntersect {
    fn from(selection: SelectionRaw) -> Self {
        SelectionIntersect(selection)
    }
}

impl From<SelectionIntersect> for SelectionRaw {
    fn from(selection_intersect: SelectionIntersect) -> Self {
        selection_intersect.0
    }
}

/// Wrapper struct with which overrides Selections' equality behevior:
/// if selections overlap, then they will be marked as equal in terms of
/// `SelectionIntersect` type.
///
/// Note that if there are `a`, `b` and `c` selections which overlap their
/// neighbour on the right, `a` and `c` won't be marked as equal, and it
/// violates transitivity. Thus `SelectionIntersect` is a hack to tweak
/// `BTreeSet` search and will be valid only within storage's `add_selection`
/// implementation, which handles this case manually.
#[derive(Debug)]
pub(crate) struct SelectionIntersect(pub(crate) SelectionRaw);

impl Eq for SelectionIntersect {}

impl PartialEq for SelectionIntersect {
    fn eq(&self, rhs: &Self) -> bool {
        let x = &self.0;
        let y = &rhs.0;
        x.from <= y.to && y.from <= x.to
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
            self.0.from.cmp(&rhs.0.from)
        }
    }
}

// Things required for tests:

#[cfg(test)]
impl PartialEq for SelectionStorage {
    fn eq(&self, rhs: &Self) -> bool {
        let self_vec: Vec<SelectionRaw> =
            self.selections_tree.iter().map(|x| x.0.clone()).collect();
        let rhs_vec: Vec<SelectionRaw> = rhs.selections_tree.iter().map(|x| x.0.clone()).collect();
        self_vec == rhs_vec
    }
}

#[cfg(test)]
type SelectionQuick = (usize, usize, usize, usize, bool);
