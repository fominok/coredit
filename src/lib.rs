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

struct Selection {
    head: Position,
    tail: Position,
}

struct Position {
    line: PositiveUsize,
    col: PositiveUsize,
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn just_check_how_lines_work() {
        let file = File::open("test_data/three_lines_with_empty.txt").unwrap();
        let buf = Buffer::from_reader(file).unwrap();
        for line in buf.rope.lines() {
            dbg!(line.len_bytes());
        }
    }
}
