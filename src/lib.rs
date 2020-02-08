mod buffer;
mod selections;
mod util;
pub use buffer::Buffer;
use snafu::Snafu;
use std::io;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to create buffer with reader: {}", source))]
    CreateFromReader { source: io::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) trait LineLengh {
    fn length(&self, line: usize) -> Option<usize>;
    fn count(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    impl LineLengh for HashMap<usize, usize> {
        fn length(&self, line: usize) -> Option<usize> {
            self.get(&line).map(|x| *x)
        }

        fn count(&self) -> usize {
            *self.keys().max().unwrap_or(&0)
        }
    }
}
