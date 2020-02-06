#[allow(dead_code)]
pub mod selections;
pub mod util;
use ropey::Rope;
use snafu::{ResultExt, Snafu};
use std::io;
use crate::selections::storage::SelectionStorage;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to create buffer with reader: {}", source))]
    CreateFromReader { source: io::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Buffer {
    rope: Rc<RefCell<Rope>>,
    selection_storage: SelectionStorage<Rc<RefCell<Rope>>>,
}

impl Buffer {
    pub fn empty() -> Self {
        Buffer {
            rope: Rc::new(RefCell::new(Rope::from_str(""))),
        }
    }

    pub fn from_reader<R: io::Read>(reader: R) -> Result<Self> {
        Ok(Buffer {
            rope: Rc::new(RefCell::new(Rope::from_reader(reader).context(CreateFromReader)?)),
        })
    }
}

impl LineLengh for Rope {
    fn length(&self, line: usize) -> Option<usize> {
        todo!()
    }

    fn count(&self) -> usize {
        todo!()
    }
}

pub(crate) trait LineLengh {
    fn length(&self, line: usize) -> Option<usize>;
    fn count(&self) -> usize;
}

#[cfg(test)]
use std::collections::HashMap;
#[cfg(test)]
impl LineLengh for HashMap<usize, usize> {
    fn length(&self, line: usize) -> Option<usize> {
        self.get(&line).map(|x| *x)
    }

    fn count(&self) -> usize {
        *self.keys().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {}
