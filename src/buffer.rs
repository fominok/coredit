use crate::selections::storage::SelectionStorage;
use crate::{CreateFromReader, LineLengh, Result};
use itertools::Itertools;
use ropey::Rope;
use snafu::ResultExt;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::io;
use std::rc::Rc;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Buffer {
    rope: Rc<RefCell<Rope>>,
    selection_storage: SelectionStorage,
}

#[cfg(test)]
impl PartialEq for Buffer {
    fn eq(&self, rhs: &Self) -> bool {
        (*self.rope.borrow() == *rhs.rope.borrow())
            && (self.selection_storage == rhs.selection_storage)
    }
}

impl Buffer {
    pub fn empty() -> Self {
        let rope = Rc::new(RefCell::new(Rope::from_str("")));
        Buffer {
            rope: rope,
            selection_storage: SelectionStorage::new(),
        }
    }

    pub fn from_reader<R: io::Read>(reader: R) -> Result<Self> {
        let rope = Rc::new(RefCell::new(
            Rope::from_reader(reader).context(CreateFromReader)?,
        ));
        Ok(Buffer {
            rope: rope,
            selection_storage: SelectionStorage::new(),
        })
    }

    pub fn move_up(&mut self, n: usize, extend: bool) {
        self.selection_storage.move_up(n, extend, self.rope.clone());
    }

    pub fn move_down(&mut self, n: usize, extend: bool) {
        self.selection_storage
            .move_down(n, extend, self.rope.clone());
    }

    pub fn move_left(&mut self, n: usize, extend: bool) {
        self.selection_storage
            .move_left(n, extend, self.rope.clone());
    }

    pub fn move_right(&mut self, n: usize, extend: bool) {
        self.selection_storage
            .move_right(n, extend, self.rope.clone());
    }

    #[cfg(test)]
    fn insert_for_test(&mut self, line: usize, col: usize, text: &str) {
        let ch = self.rope.borrow().line_to_char(line - 1) + col - 1;
        self.rope.borrow_mut().insert(ch, text);
    }

    #[cfg(test)]
    fn delete_for_test(
        &mut self,
        from_line: usize,
        from_col: usize,
        to_line: usize,
        to_col: usize,
    ) {
        let ch_from = self.rope.borrow().line_to_char(from_line - 1) + from_col - 1;
        let ch_to = self.rope.borrow().line_to_char(to_line - 1) + to_col - 1;
        self.rope.borrow_mut().remove(ch_from..=ch_to);
    }

    pub fn insert(&mut self, text: &str) {
        let mut rope = self.rope.borrow_mut();
        // Perform insertion reversed to prevent selections invalidation
        // on previous iteration if it were moved forward
        for s in self.selection_storage.iter().rev() {
            let cursor = s.get_cursor();
            let ch: usize = rope.line_to_char(Into::<usize>::into(cursor.line) - 1)
                + Into::<usize>::into(cursor.col)
                - 1;
            rope.insert(ch, text);
        }

        // TODO: fix to grapheme clusters
        let insertion_info = text.chars().group_by(|&x| x == '\n');
        for (is_nl, group) in &insertion_info {
            let l = group.count();
            if is_nl {
                self.selection_storage.move_down_incremental(l);
            } else {
                self.selection_storage.move_right_incremental(l);
            }
        }
    }

    pub fn delete(&mut self) {
        let mut rope = self.rope.borrow_mut();
        let mut offset: usize = 0;
        let events: Vec<_> = self
            .selection_storage
            .iter()
            .map(|s| {
                let (from, to) = s.get_bounds();
                let from_ch: usize = rope.line_to_char(Into::<usize>::into(from.line) - 1)
                    + Into::<usize>::into(from.col)
                    - 1;
                let to_ch: usize = rope.line_to_char(Into::<usize>::into(to.line) - 1)
                    + Into::<usize>::into(to.col)
                    - 1;
                DeleteEvent {
                    characters: to_ch - from_ch + 1,
                    newlines: 0,
                    from: from_ch,
                    to: to_ch,
                }
            })
            .collect();

        for e in events.iter() {}
        //let selections_old =
        //    std::mem::replace(&mut self.selection_storage.selections_tree, BTreeSet::new());
        //for mut s in selections_old.into_iter().map(|si| si.0) {
        //    s.nudge_left(offset);
        //    let (from, to) = s.get_bounds();
        //    let from_ch: usize = rope.line_to_char(Into::<usize>::into(from.line) - 1)
        //        + Into::<usize>::into(from.col)
        //        - 1;
        //    let to_ch: usize = rope.line_to_char(Into::<usize>::into(to.line) - 1)
        //        + Into::<usize>::into(to.col)
        //        - 1;
        //    offset += to_ch - from_ch + 1;
        //    if to_ch < rope.len_chars() {
        //        rope.remove(from_ch..=to_ch);
        //    }
        //    self.selection_storage.add_selection(s);
        //}
        ////for (after_line, after_col, n) in changes.into_iter() {
        ////    self.selection_storage.move_left_on_line(after_line, after_col, n);
        ////}
        //self.selection_storage.shrink_to_head();
    }
}

struct DeleteEvent {
    characters: usize,
    newlines: usize,
    from: usize,
    to: usize,
}

impl LineLengh for Rope {
    fn length(&self, line: usize) -> Option<usize> {
        // `line` arg is starting from 1

        // FIXME: \n and \r do not cover all newline things afaik
        // Also desired len is in grapheme clusters not String's .len()
        if line > 0 && line <= self.count() {
            let s = self.line(line - 1).to_string();
            Some(s.trim_end_matches(|x| x == '\n' || x == '\r').len() + 1)
        } else {
            None
        }
    }

    fn count(&self) -> usize {
        self.len_lines()
    }
}

impl<L: LineLengh> LineLengh for RefCell<L> {
    fn length(&self, line: usize) -> Option<usize> {
        self.borrow().length(line)
    }

    fn count(&self) -> usize {
        self.borrow().count()
    }
}
