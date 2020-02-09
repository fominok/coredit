use super::*;
use pretty_assertions::assert_eq;

#[test]
fn move_right_simple() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 11, 1, 11, true), (3, 5, 3, 5, true)]);
    buffer.move_right(5, false);
    let reference_buffer =
        load_buffer_with_selections(&vec![(1, 16, 1, 16, true), (3, 10, 3, 10, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_right_merge() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 1, 1, 4, true), (3, 6, 3, 9, true)]);
    buffer.move_right(63, true);
    let reference_buffer = load_buffer_with_selections(&vec![(1, 1, 4, 51, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_left_simple() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 11, 1, 11, true), (3, 5, 3, 5, true)]);
    buffer.move_left(5, false);
    let reference_buffer =
        load_buffer_with_selections(&vec![(1, 6, 1, 6, true), (2, 1, 2, 1, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_left_merge() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 54, 1, 59, true), (3, 14, 3, 19, true)]);
    buffer.move_left(31, true);
    let reference_buffer = load_buffer_with_selections(&vec![(1, 28, 3, 14, false)]);
    assert_eq!(buffer, reference_buffer);
}
