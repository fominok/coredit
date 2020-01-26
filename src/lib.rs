#[allow(dead_code)]
pub mod selections;
pub mod util;
use ropey::Rope;
use snafu::{ResultExt, Snafu};
use std::io;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to create buffer with reader: {}", source))]
    CreateFromReader { source: io::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Buffer {
    rope: Rope,
}

impl Buffer {
    pub fn empty() -> Self {
        Buffer {
            rope: Rope::from_str(""),
        }
    }

    pub fn from_reader<R: io::Read>(reader: R) -> Result<Self> {
        Ok(Buffer {
            rope: Rope::from_reader(reader).context(CreateFromReader)?,
        })
    }
}

pub(crate) trait LineLengh {
    fn lengh(&self, line: usize) -> Option<usize>;
}

use std::collections::HashMap;
#[cfg(test)]
impl LineLengh for HashMap<usize, usize> {
    fn lengh(&self, line: usize) -> Option<usize> {
        self.get(&line).map(|x| *x)
    }
}

#[cfg(test)]
mod tests {}
