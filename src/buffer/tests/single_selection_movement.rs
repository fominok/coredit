use super::*;
use crate::selections::{CursorDirection, SelectionRaw};
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
fn move_left_simple() {
    let mut buffer = load_buffer();
    buffer.move_right(5, false);
    buffer.move_right(30, true);
    buffer.swap_cursor();
    buffer.move_left(1, false);
    let reference_buffer = load_buffer_with_selections(&vec![(1, 5, 1, 5, true)]);
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
    let reference_buffer = load_buffer_with_selections(&vec![(8, 53, 8, 53, true)]);
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
fn move_up_sticky_n_times() {
    let mut buffer = load_buffer_with_selections(&vec![(7, 37, 7, 37, true)]);
    buffer.move_up(1, false);
    buffer.move_up(1, false);
    buffer.move_up(1, false);
    buffer.move_up(1, false);
    buffer.move_up(1, false);
    buffer.move_up(1, false);
    let reference_buffer = load_buffer_with_selections(&vec![(1, 37, 1, 37, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_down_sticky_n_times() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 37, 1, 37, true)]);
    buffer.move_down(1, false);
    assert_eq!(
        buffer
            .internal_selections_iter()
            .collect::<Vec<SelectionRaw>>(),
        vec![SelectionRaw::new_quick(2, 1, 2, 1, CursorDirection::Forward).with_sticky(37)]
    );
    buffer.move_down(1, false);
    assert_eq!(
        buffer
            .internal_selections_iter()
            .collect::<Vec<SelectionRaw>>(),
        vec![SelectionRaw::new_quick(3, 21, 3, 21, CursorDirection::Forward).with_sticky(37)]
    );
    buffer.move_down(1, false);
    assert_eq!(
        buffer
            .internal_selections_iter()
            .collect::<Vec<SelectionRaw>>(),
        vec![SelectionRaw::new_quick(
            4,
            37,
            4,
            37,
            CursorDirection::Forward
        )]
    );
    buffer.move_down(1, false);
    assert_eq!(
        buffer
            .internal_selections_iter()
            .collect::<Vec<SelectionRaw>>(),
        vec![SelectionRaw::new_quick(5, 25, 5, 25, CursorDirection::Forward).with_sticky(37)]
    );
    buffer.move_down(1, false);
    assert_eq!(
        buffer
            .internal_selections_iter()
            .collect::<Vec<SelectionRaw>>(),
        vec![SelectionRaw::new_quick(
            6,
            37,
            6,
            37,
            CursorDirection::Forward
        )]
    );
    buffer.move_down(1, false);
    let reference_buffer = load_buffer_with_selections(&vec![(7, 37, 7, 37, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_down_sticky_and_overflow() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 50, 1, 50, true)]);
    buffer.move_down(2, false);
    buffer.move_left(1, false);
    buffer.move_down(420, false);
    let reference_buffer = load_buffer_with_selections(&vec![(8, 20, 8, 20, true)]);
    assert_eq!(buffer, reference_buffer);
}
