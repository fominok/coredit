mod multi_selection_delete;
mod multi_selection_insert;
mod multi_selection_movement;
mod single_selection_delete;
mod single_selection_deltas;
mod single_selection_insert;
mod single_selection_movement;

use super::Buffer;
use crate::selections::storage::SelectionStorage;
use std::fs::File;

const TEXT: &'static str = "test_data/sample_text.txt";

fn load_buffer() -> Buffer {
    let f = File::open(TEXT).unwrap();
    Buffer::from_reader(f).unwrap()
}

fn load_buffer_with_selections(selections: &[(usize, usize, usize, usize, bool)]) -> Buffer {
    let mut buffer = load_buffer();
    let storage = SelectionStorage::gen_from_tuples(selections);
    buffer.selection_storage = storage;
    buffer
}
