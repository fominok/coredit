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

#[test]
fn move_up_simple() {
    let mut buffer = load_buffer_with_selections(&vec![(4, 5, 4, 5, true), (4, 10, 4, 10, true)]);
    buffer.move_up(1, false);
    let reference_buffer =
        load_buffer_with_selections(&vec![(3, 5, 3, 5, true), (3, 10, 3, 10, true)]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_up_sticky_no_merge() {
    // Merging and sticky behaves differently with N argument
    // and doing so N times with argument 1
    let mut buffer = load_buffer_with_selections(&vec![
        (3, 5, 3, 5, true),
        (3, 6, 3, 6, true),
        (3, 7, 3, 7, true),
        (3, 8, 3, 8, true),
        (3, 9, 3, 9, true),
    ]);
    buffer.move_up(2, false);
    let reference_buffer = load_buffer_with_selections(&vec![
        (1, 5, 1, 5, true),
        (1, 6, 1, 6, true),
        (1, 7, 1, 7, true),
        (1, 8, 1, 8, true),
        (1, 9, 1, 9, true),
    ]);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn move_up_sticky_separately_merge() {
    // While they were merged the stickiness is preserved
    // for the selection 1 by order
    let mut buffer = load_buffer_with_selections(&vec![
        (3, 5, 3, 5, true),
        (3, 6, 3, 6, true),
        (3, 7, 3, 7, true),
        (3, 8, 3, 8, true),
        (3, 9, 3, 9, true),
    ]);
    buffer.move_up(1, false);
    buffer.move_up(1, false);
    let reference_buffer = load_buffer_with_selections(&vec![(1, 5, 1, 5, true)]);
    assert_eq!(buffer, reference_buffer);
}
