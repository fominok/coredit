use crate::selections::storage::SelectionStorage;
use crate::{CreateFromReader, LineLengh, Result};
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
        let ch = self.rope.borrow().line_to_char(line - 1) + col;
        self.rope.borrow_mut().insert(ch, text);
    }

    fn insert(&mut self, text: &str) {
        let l = text.len();
        let mut rope = self.rope.borrow_mut();

        // Perform insertion reversed to prevent selections invalidation
        // on previous iteration if it were moved forward
        for s in self.selection_storage.iter().rev() {
            let cursor = s.get_cursor();
            let ch: usize = rope.line_to_char(Into::<usize>::into(cursor.line) - 1)
                + Into::<usize>::into(cursor.col);
            rope.insert(ch, text);
        }

        self.selection_storage.move_right_incremental(l);
    }
}

impl LineLengh for Rope {
    fn length(&self, line: usize) -> Option<usize> {
        if line > 0 && line < self.count() {
            Some(self.line(line - 1).len_chars())
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
