use super::super::*;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

// Up movements

#[test]
fn test_move_up_enough_length() {
    let mut line_length = HashMap::new();
    line_length.insert(4, 40);
    line_length.insert(5, 30);
    let mut selection = Selection::new_quick(5, 10, 5, 20, CursorDirection::Forward);
    selection = selection.move_up(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 20, 5, 10, CursorDirection::Backward),
    );
}

#[test]
fn test_move_up_until_first_line() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 40);
    line_length.insert(2, 40);
    line_length.insert(3, 40);
    let mut selection = Selection::new_quick(3, 10, 3, 20, CursorDirection::Forward);
    selection = selection.move_up(322, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 20, 3, 10, CursorDirection::Backward),
    );
}

#[test]
fn test_move_up_preserve_column() {
    let mut line_length = HashMap::new();
    line_length.insert(2, 322);
    line_length.insert(3, 20);
    line_length.insert(4, 30);
    line_length.insert(5, 50);
    let mut selection = Selection::new_quick(5, 10, 5, 40, CursorDirection::Forward);

    // On the first move it should be the end of line if it is shorter
    selection = selection.move_up(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 30, 5, 10, CursorDirection::Backward).with_sticky(40),
    );

    // On the next move it should return to its sticky postition if line is long enough
    selection = selection.move_up(2, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(2, 40, 5, 10, CursorDirection::Backward),
    );
}

#[test]
fn test_move_up_drop_column_on_left_right() {
    let mut line_length = HashMap::new();
    line_length.insert(3, 50);
    line_length.insert(4, 30);
    line_length.insert(5, 50);
    let mut selection = Selection::new_quick(5, 10, 5, 40, CursorDirection::Forward);

    // On the first move it should be the end of line if it is shorter
    selection = selection.move_up(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 30, 5, 10, CursorDirection::Backward).with_sticky(40),
    );

    selection = selection.move_left(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 29, 5, 10, CursorDirection::Backward),
    );

    // On the next move it should retain its column as was moved left
    selection = selection.move_up(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(3, 29, 5, 10, CursorDirection::Backward),
    );
}

#[test]
fn test_move_up_preserve_column_drop_selection_once() {
    let mut line_length = HashMap::new();
    line_length.insert(2, 322);
    line_length.insert(3, 20);
    line_length.insert(4, 30);
    line_length.insert(5, 50);
    let mut selection = Selection::new_quick(5, 10, 5, 40, CursorDirection::Forward);

    // On the first move it should be the end of line if it is shorter
    selection = selection.move_up(1, false, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 30, 4, 30, CursorDirection::Forward).with_sticky(40),
    );

    // On the next move it should return to its sticky postition if line is long enough
    selection = selection.move_up(2, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(2, 40, 4, 30, CursorDirection::Backward),
    );
}

// Down movements

#[test]
fn test_move_down_enough_length() {
    let mut line_length = HashMap::new();
    line_length.insert(4, 40);
    line_length.insert(5, 30);
    let mut selection = Selection::new_quick(4, 10, 4, 20, CursorDirection::Forward);
    selection = selection.move_down(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 10, 5, 20, CursorDirection::Forward),
    );
}

#[test]
fn test_move_down_until_last_line() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 40);
    line_length.insert(2, 50);
    line_length.insert(3, 50);
    let mut selection = Selection::new_quick(1, 10, 1, 20, CursorDirection::Forward);
    selection = selection.move_down(420, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 10, 3, 20, CursorDirection::Forward),
    );
}

#[test]
fn test_move_down_preserve_column() {
    let mut line_length = HashMap::new();
    line_length.insert(2, 50);
    line_length.insert(3, 20);
    line_length.insert(4, 30);
    line_length.insert(5, 50);
    let mut selection = Selection::new_quick(2, 10, 2, 40, CursorDirection::Forward);

    // On the first move it should be the end of line if it is shorter
    selection = selection.move_down(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(2, 10, 3, 20, CursorDirection::Forward).with_sticky(40),
    );

    // On the next move it should return to its sticky postition if line is long enough
    selection = selection.move_down(2, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(2, 10, 5, 40, CursorDirection::Forward),
    );
}

#[test]
fn test_move_down_drop_column_on_left_right() {
    let mut line_length = HashMap::new();
    line_length.insert(3, 50);
    line_length.insert(4, 30);
    line_length.insert(5, 50);
    let mut selection = Selection::new_quick(3, 10, 3, 40, CursorDirection::Forward);

    // On the first move it should be the end of line if it is shorter
    selection = selection.move_down(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(3, 10, 4, 30, CursorDirection::Forward).with_sticky(40),
    );

    selection = selection.move_left(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(3, 10, 4, 29, CursorDirection::Forward),
    );

    // On the next move it should return to its sticky postition if line is long enough
    selection = selection.move_down(1, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(3, 10, 5, 29, CursorDirection::Forward),
    );
}

#[test]
fn test_move_down_preserve_column_drop_selection_once() {
    let mut line_length = HashMap::new();
    line_length.insert(2, 50);
    line_length.insert(3, 20);
    line_length.insert(4, 30);
    line_length.insert(5, 50);
    let mut selection = Selection::new_quick(2, 10, 2, 40, CursorDirection::Forward);

    // On the first move it should be the end of line if it is shorter
    selection = selection.move_down(1, false, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(3, 20, 3, 20, CursorDirection::Forward).with_sticky(40),
    );

    // On the next move it should return to its sticky postition if line is long enough
    selection = selection.move_down(2, true, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(3, 20, 5, 40, CursorDirection::Forward),
    );
}

#[test]
fn test_create_under_multi_line() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 50);
    line_length.insert(2, 20);
    line_length.insert(3, 30);
    line_length.insert(4, 30);
    line_length.insert(5, 50);
    line_length.insert(6, 50);
    line_length.insert(7, 50);
    line_length.insert(8, 50);
    line_length.insert(9, 50);
    let selection = Selection::new_quick(1, 40, 3, 10, CursorDirection::Forward);

    // On the first move it should be the end of line if it is shorter

    let new_selection = selection.create_selection_under(&line_length).unwrap();
    assert_eq!(
        new_selection,
        Selection::new_quick(5, 40, 7, 10, CursorDirection::Forward),
    );
}

#[test]
fn test_create_under_single_line() {
    let mut line_length = HashMap::new();
    line_length.insert(2, 50);
    line_length.insert(3, 20);
    line_length.insert(4, 30);

    let selection = Selection::new_quick(2, 10, 2, 20, CursorDirection::Backward);
    let new_selection = selection.create_selection_under(&line_length).unwrap();
    assert_eq!(
        new_selection,
        Selection::new_quick(3, 10, 3, 20, CursorDirection::Backward),
    );

    let selection = Selection::new_quick(2, 10, 2, 29, CursorDirection::Backward);
    let new_selection = selection.create_selection_under(&line_length).unwrap();
    assert_eq!(
        new_selection,
        Selection::new_quick(4, 10, 4, 29, CursorDirection::Backward),
    );
}
