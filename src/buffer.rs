use crate::selections::storage::SelectionStorage;
use crate::selections::Selection;
use crate::{CreateFromReader, LineLengh, Result};
use itertools::Itertools;
use ropey::Rope;
use snafu::ResultExt;
use std::cell::RefCell;
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
        let mut current_selection = self.selection_storage.iter().rev().next();

        while let Some(mut s) = current_selection.take() {
            // Cases:
            // 1. Selection ends on the same line as starts: selections on the same
            //    line will be moved on chars delta
            // 2. Selection ends on the newline character of the same line:
            //    the next line will be concated to selection head from its beginning;
            //    Other lines below are moved up;
            // 3. Selection goes through multiple lines and ends somewhere on it:
            //    tail line will be concatenated to selection head since tail pos;
            //    Other lines below are moved up;
            // 4. Selection goes through multiple lines and ends on newline char:
            //    the next line after tail will be concated to selection head from
            //    its beginning;
            //    Other lines below are moved up;
            //
            // So everything below tail should be moved up by lines delta;
            // If tail ends with newline then concat next from beginning to head
            // If tail ends on another line but not its end then concat from that pos
            //    to head

            let mut rope = self.rope.borrow_mut();
            let (from, to) = s.get_bounds();
            // To check number of newlines we need a head tail difference in lines
            // plus check if tail ends on newline
            let from_line: usize = from.line.into();
            let to_line: usize = to.line.into();

            let from_ch: usize =
                rope.line_to_char(from_line - 1) + Into::<usize>::into(from.col) - 1;
            let to_ch: usize = rope.line_to_char(to_line - 1) + Into::<usize>::into(to.col) - 1;

            let ends_in_nl = rope.length(to_line).unwrap() < to.col.into();

            // FIXME: this delta is wrong:
            // chars should be head line length -  head column + tail column
            // newlines seems correct for now

            let chars_delta = to_ch - from_ch + 1;
            let lines_delta = to_line - from_line + if ends_in_nl { 1 } else { 0 };
            current_selection = self.selection_storage.get_first_before(&s);
            self.selection_storage.apply_delete(s, rope);
            let mut rope = self.rope.borrow_mut();
            if to_ch < rope.len_chars() {
                rope.remove(from_ch..=to_ch);
            }
        }
    }
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
