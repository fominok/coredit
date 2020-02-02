use super::super::*;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn test_move_up_enough_length() {
    let mut line_length = HashMap::new();
    line_length.insert(4, 40);
    line_length.insert(5, 30);
    let mut selection = Selection::new_quick(5, 10, 5, 20, CursorDirection::Forward);
    selection.move_up(1, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 20, 5, 10, CursorDirection::Backward),
    );
}

#[test]
fn test_move_up_first_line() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 40);
    let mut selection = Selection::new_quick(1, 10, 1, 20, CursorDirection::Forward);
    selection.move_up(1, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(1, 10, 1, 20, CursorDirection::Forward),
    );
}

#[test]
fn test_move_up_preserve_column() {
    let mut line_length = HashMap::new();
    line_length.insert(3, 50);
    line_length.insert(4, 30);
    line_length.insert(5, 50);
    let mut selection = Selection::new_quick(5, 10, 5, 40, CursorDirection::Forward);

    // On the first move it should be the end of line if it is shorter
    selection.move_up(1, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 30, 5, 10, CursorDirection::Backward).with_sticky(40),
    );

    // On the next move it should return to its sticky postition if line is long enough
    selection.move_up(1, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(3, 40, 5, 10, CursorDirection::Backward),
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
    selection.move_up(1, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 30, 5, 10, CursorDirection::Backward).with_sticky(40),
    );

    selection.move_left(1, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(4, 29, 5, 10, CursorDirection::Backward).with_sticky(40),
    );

    // On the next move it should retain its column as was moved left
    selection.move_up(1, &line_length);
    assert_eq!(
        selection,
        Selection::new_quick(3, 40, 5, 10, CursorDirection::Backward),
    );
}
