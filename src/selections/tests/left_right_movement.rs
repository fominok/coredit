use super::super::*;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn test_move_left_one_line() {
    let line_length = HashMap::new();
    let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
    selection.move_left(5, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 10, 6, 15, CursorDirection::Forward),
    );
}

#[test]
fn test_move_left_multiple_lines() {
    let mut line_length = HashMap::new();
    line_length.insert(6, 322);
    line_length.insert(5, 40);
    line_length.insert(4, 30);
    let mut selection = Selection::new_quick(2, 20, 6, 20, CursorDirection::Forward);
    selection.move_left(80, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(2, 20, 4, 10, CursorDirection::Forward),
    );
}

#[test]
fn test_move_left_multiple_lines_until_beginning() {
    let mut line_length = HashMap::new();
    line_length.insert(6, 322);
    line_length.insert(5, 40);
    line_length.insert(4, 30);
    line_length.insert(3, 30);
    line_length.insert(2, 30);
    line_length.insert(1, 30);
    let mut selection = Selection::new_quick(2, 20, 6, 20, CursorDirection::Backward);
    selection.move_left(1337, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 1, 6, 20, CursorDirection::Backward),
    );
}

#[test]
fn test_move_left_one_line_until_beginning() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 322);
    let mut selection = Selection::new_quick(1, 20, 1, 70, CursorDirection::Backward);
    selection.move_left(1337, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 1, 1, 70, CursorDirection::Backward),
    );
}

#[test]
fn test_move_left_one_empty_line() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 1);
    let mut selection = Selection::new_quick(1, 1, 1, 1, CursorDirection::Forward);
    selection.move_left(1337, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 1, 1, 1, CursorDirection::Forward),
    );
}

#[test]
fn test_move_left_multiple_lines_reversed() {
    let mut line_length = HashMap::new();
    line_length.insert(6, 322);
    line_length.insert(5, 40);
    line_length.insert(4, 30);
    let mut selection = Selection::new_quick(5, 20, 6, 20, CursorDirection::Forward);
    selection.move_left(80, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 10, 5, 20, CursorDirection::Backward),
    );
}

#[test]
fn test_move_right_one_line() {
    let mut line_length = HashMap::new();
    line_length.insert(6, 50);
    let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
    selection.move_right(5, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 10, 6, 25, CursorDirection::Forward),
    );
}

#[test]
fn test_move_right_multiple_lines() {
    let mut line_length = HashMap::new();
    line_length.insert(6, 30);
    line_length.insert(7, 35);
    line_length.insert(8, 335);
    let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
    selection.move_right(70, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 10, 8, 25, CursorDirection::Forward),
    );
}

#[test]
fn test_move_right_multiple_lines_until_end() {
    let mut line_length = HashMap::new();
    line_length.insert(6, 30);
    line_length.insert(7, 35);
    line_length.insert(8, 335);
    let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
    selection.move_right(700, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 10, 8, 335, CursorDirection::Forward),
    );
}

#[test]
fn test_move_right_one_line_until_end() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 50);
    let mut selection = Selection::new_quick(1, 10, 1, 20, CursorDirection::Forward);
    selection.move_right(500, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 10, 1, 50, CursorDirection::Forward),
    );
}

#[test]
fn test_move_right_one_empty_line() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 1);
    let mut selection = Selection::new_quick(1, 1, 1, 1, CursorDirection::Forward);
    selection.move_right(420, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 1, 1, 1, CursorDirection::Forward),
    );
}

#[test]
fn test_move_right_multiple_lines_reversed() {
    let mut line_length = HashMap::new();
    line_length.insert(4, 30);
    line_length.insert(5, 80);
    line_length.insert(6, 30);
    line_length.insert(7, 35);
    line_length.insert(8, 335);
    let mut selection = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
    selection.move_right(140, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(6, 20, 7, 10, CursorDirection::Forward),
    );
}

#[test]
fn test_move_right_one_in_the_end() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 30);
    let mut selection = Selection::new_quick(1, 10, 1, 30, CursorDirection::Forward);
    selection.move_right(1, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 10, 1, 30, CursorDirection::Forward),
    );
}
