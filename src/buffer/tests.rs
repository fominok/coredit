mod multi_selection_insert;
mod multi_selection_movement;
mod single_selection_insert;
mod single_selection_movement;
use super::Buffer;
use crate::selections::storage::SelectionStorage;
use std::fs::File;

const THREE_LINES_TEXT: &'static str = "test_data/three_lines_with_empty.txt";

fn load_buffer() -> Buffer {
    let f = File::open(THREE_LINES_TEXT).unwrap();
    Buffer::from_reader(f).unwrap()
}

fn load_buffer_with_selections(selections: &[(usize, usize, usize, usize, bool)]) -> Buffer {
    let mut buffer = load_buffer();
    let storage = SelectionStorage::gen_from_tuples(selections);
    buffer.selection_storage = storage;
    buffer
}
