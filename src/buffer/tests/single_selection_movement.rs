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
fn move_right_line_overflow() {
    // Actually the second line will be overflowed too
    let mut buffer = load_buffer();
    buffer.move_right(62, false);
    let reference_buffer = load_buffer_with_selections(&vec![(3, 2, 3, 2, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_right_file_overflow() {
    // Actually the second line will be overflowed too
    let mut buffer = load_buffer();
    buffer.move_right(1337, false);
    let reference_buffer = load_buffer_with_selections(&vec![(4, 83, 4, 83, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_left_file_overflow() {
    // Actually the second line will be overflowed too
    let mut buffer = load_buffer();
    buffer.move_right(62, false);
    let reference_buffer = load_buffer_with_selections(&vec![(3, 2, 3, 2, true)]);
    assert_eq!(buffer, reference_buffer);
    buffer.move_left(420, false);
    let reference_buffer = load_buffer_with_selections(&vec![(1, 1, 1, 1, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_up_sticky_and_overflow() {
    let mut buffer = load_buffer_with_selections(&vec![(4, 50, 4, 50, true)]);
    buffer.move_up(1, false);
    buffer.move_left(1, false);
    buffer.move_up(322, false);
    let reference_buffer = load_buffer_with_selections(&vec![(1, 20, 1, 20, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_down_sticky_and_overflow() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 50, 1, 50, true)]);
    buffer.move_down(2, false);
    buffer.move_left(1, false);
    buffer.move_down(420, false);
    let reference_buffer = load_buffer_with_selections(&vec![(4, 20, 4, 20, true)]);
    assert_eq!(buffer, reference_buffer);
}
