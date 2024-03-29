use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_insert() {
    let mut buffer = load_buffer();
    buffer.move_right(30, false);
    buffer.insert(" awesome crate named");
    let mut reference_buffer = load_buffer_with_selections(&vec![(1, 51, 1, 51, true)]);
    reference_buffer.insert_for_test(1, 31, " awesome crate named");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_before_selection() {
    let mut buffer = load_buffer();
    buffer.move_right(40, false);
    buffer.move_left(10, true);
    buffer.insert(" awesome crate named");
    let mut reference_buffer = load_buffer_with_selections(&vec![(1, 51, 1, 61, false)]);
    reference_buffer.insert_for_test(1, 31, " awesome crate named");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_after_selection() {
    let mut buffer = load_buffer();
    buffer.move_right(20, false);
    buffer.move_right(10, true);
    buffer.insert(" awesome crate named");
    let mut reference_buffer = load_buffer_with_selections(&vec![(1, 21, 1, 51, true)]);
    reference_buffer.insert_for_test(1, 31, " awesome crate named");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_with_newline() {
    let mut buffer = load_buffer();
    buffer.move_right(30, false);
    buffer.insert(" awesome\ncrate named");
    let mut reference_buffer = load_buffer_with_selections(&vec![(2, 12, 2, 12, true)]);
    reference_buffer.insert_for_test(1, 31, " awesome\ncrate named");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_before_selection_with_newline() {
    let mut buffer = load_buffer();
    buffer.move_right(40, false);
    buffer.move_left(10, true);
    buffer.insert(" awesome\ncrate named");
    let mut reference_buffer = load_buffer_with_selections(&vec![(2, 12, 2, 22, false)]);
    reference_buffer.insert_for_test(1, 31, " awesome\ncrate named");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_after_selection_with_newline() {
    let mut buffer = load_buffer();
    buffer.move_right(20, false);
    buffer.move_right(10, true);
    buffer.insert(" awesome\ncrate named");
    let mut reference_buffer = load_buffer_with_selections(&vec![(1, 21, 2, 12, true)]);
    reference_buffer.insert_for_test(1, 31, " awesome\ncrate named");
    assert_eq!(buffer, reference_buffer);
}
