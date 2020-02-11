use super::*;
use pretty_assertions::assert_eq;

#[test]
fn insert_some_characters() {
    let mut buffer = load_buffer();
    buffer.move_right(30, false);
    buffer.insert("awesome crate named ");
    let mut reference_buffer = load_buffer_with_selections(&vec![(1, 51, 1, 51, true)]);
    reference_buffer.insert_for_test(1, 31, "awesome crate named ");
    assert_eq!(buffer, reference_buffer);
}
