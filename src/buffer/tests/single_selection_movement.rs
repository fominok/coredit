use super::*;
use pretty_assertions::assert_eq;

#[test]
fn move_right_simple() {
    // First line length is 59 + newline
    let mut buffer = load_buffer();
    buffer.move_right(30, false);
    let reference_buffer = load_buffer_with_selections(&vec![(1, 31, 1, 31, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_right_line_overflow() {}
