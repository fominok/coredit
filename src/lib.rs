pub mod selections;
pub mod util;
use crate::util::PositiveUsize;
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
    fn lengh(&self, line: usize) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
}
