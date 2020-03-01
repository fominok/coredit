use crate::selections::storage::SelectionStorage;
use crate::selections::Selection;
use crate::{CreateFromReader, LineLength, Result};
use itertools::Itertools;
use ropey::Rope;
use snafu::ResultExt;
use std::fmt;
use std::io;

#[cfg(test)]
mod tests;

/// The main structure to store and process text.
///
/// As usual for editors, could be created empty or from a reader.
#[derive(Debug)]
pub struct Buffer {
    rope: Rope,
    selection_storage: SelectionStorage,
}

#[cfg(test)]
impl PartialEq for Buffer {
    fn eq(&self, rhs: &Self) -> bool {
        (self.rope == rhs.rope) && (self.selection_storage == rhs.selection_storage)
    }
}

impl Buffer {
    /// Create an empty `Buffer`.
    ///
    /// ```
    /// # use coredit::Buffer;
    /// let mut buffer = Buffer::empty();
    /// assert_eq!(buffer.to_string(), "".to_string());
    /// ```
    pub fn empty() -> Self {
        let rope = Rope::from_str("");
        Buffer {
            rope,
            selection_storage: SelectionStorage::new(),
        }
    }

    /// Create `Buffer` from `Reader`.
    ///
    /// ```
    /// # use coredit::Buffer;
    /// use std::fs::File;
    ///
    /// let file = File::open("test_data/one_liner.txt").unwrap();
    /// let mut buffer = Buffer::from_reader(file).unwrap();
    /// assert!(buffer.to_string().starts_with("That was easy"));
    /// ```
    pub fn from_reader<R: io::Read>(reader: R) -> Result<Self> {
        let rope = Rope::from_reader(reader).context(CreateFromReader)?;
        Ok(Buffer {
            rope,
            selection_storage: SelectionStorage::new(),
        })
    }

    // /// Return Ropey's `Lines` iterator from line numbered `from_line`.
    // /// Note that first line has index 1.
    // pub fn lines_at(&self, from_line: usize) -> ropey::iter::Lines {
    //     self.rope.lines_at(from_line.saturating_sub(1))
    // }

    /// Expose underlying Rope read-only way
    pub fn get_rope(&self) -> &Rope {
        &self.rope
    }

    /// Return an iterator over selections
    pub fn selections_iter(&self) -> impl Iterator<Item = Selection> + '_ {
        self.selection_storage.iter()
    }

    /// Move all cursors up by `n`, shrinking selections to length 1
    /// if `extend` is not set.
    pub fn move_up(&mut self, n: usize, extend: bool) {
        self.selection_storage.move_up(n, extend, &self.rope);
    }

    /// Move all cursors down by `n`, shrinking selections to length 1
    /// if `extend` is not set.
    pub fn move_down(&mut self, n: usize, extend: bool) {
        self.selection_storage.move_down(n, extend, &self.rope);
    }

    /// Move all cursors left by `n`, shrinking selections to length 1
    /// if `extend` is not set.
    pub fn move_left(&mut self, n: usize, extend: bool) {
        self.selection_storage.move_left(n, extend, &self.rope);
    }

    /// Move all cursors right by `n`, shrinking selections to length 1
    /// if `extend` is not set.
    pub fn move_right(&mut self, n: usize, extend: bool) {
        self.selection_storage.move_right(n, extend, &self.rope);
    }

    /// Insert `text` on all cursors.
    ///
    /// If selection's cursor is in front, then the selection will be moved
    /// right; otherwise it will be extened.
    pub fn insert(&mut self, text: &str) {
        // Perform insertion reversed to prevent selections invalidation
        // on previous iteration if it were moved forward
        for s in self.selection_storage.iter().rev() {
            let cursor = s.get_cursor();
            let ch: usize = self.rope.line_to_char(cursor.line.get() - 1) + cursor.col.get() - 1;
            self.rope.insert(ch, text);
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

    /// Delete selected text for all selections.
    ///
    /// All selections will have length equal 1.
    pub fn delete(&mut self) {
        let mut current_selection = self.selection_storage.iter().rev().next();

        while let Some(s) = current_selection.take() {
            let (from, to) = s.get_bounds();
            let from_line: usize = from.line.into();
            let to_line: usize = to.line.into();

            let from_ch: usize = self.rope.line_to_char(from_line - 1) + from.col.get() - 1;
            let to_ch: usize = {
                if to.col.get()
                    == self
                        .rope
                        .length(to_line)
                        .expect("Selection reached inconsistency")
                {
                    self.rope.line_to_char(to_line).saturating_sub(1)
                } else {
                    self.rope.line_to_char(to_line - 1) + to.col.get() - 1
                }
            };

            current_selection = self.selection_storage.get_first_before(&s);
            self.selection_storage.apply_delete(s, &self.rope);
            if to_ch < self.rope.len_chars() {
                self.rope.remove(from_ch..=to_ch);
            }
        }
    }

    // Helper functions for testing

    #[cfg(test)]
    fn insert_for_test(&mut self, line: usize, col: usize, text: &str) {
        let ch = self.rope.line_to_char(line - 1) + col - 1;
        self.rope.insert(ch, text);
    }

    #[cfg(test)]
    fn delete_for_test(
        &mut self,
        from_line: usize,
        from_col: usize,
        to_line: usize,
        to_col: usize,
    ) {
        let ch_from = self.rope.line_to_char(from_line - 1) + from_col - 1;
        let ch_to: usize = {
            let to_line_length = self.rope.length(to_line);
            if to_col == to_line_length.unwrap() {
                self.rope.line_to_char(to_line) - 1
            } else {
                self.rope.line_to_char(to_line - 1) + to_col - 1
            }
        };
        self.rope.remove(ch_from..=ch_to);
    }
}

impl LineLength for Rope {
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

impl LineLength for &Rope {
    fn length(&self, line: usize) -> Option<usize> {
        (*self).length(line)
    }

    fn count(&self) -> usize {
        (*self).len_lines()
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rope)
    }
}
