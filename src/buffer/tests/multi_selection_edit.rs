use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_insert_some_characters() {
    let mut buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 3, true),
        (3, 11, 3, 11, true),
        (4, 33, 4, 33, true),
    ]);
    buffer.insert(" (top kek) ");
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (1, 14, 1, 14, true),
        (3, 22, 3, 22, true),
        (4, 44, 4, 44, true),
    ]);
    reference_buffer.insert_for_test(1, 3, " (top kek) ");
    reference_buffer.insert_for_test(3, 11, " (top kek) ");
    reference_buffer.insert_for_test(4, 33, " (top kek) ");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_some_characters_before_selection() {
    let mut buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 13, false),
        (3, 11, 3, 21, false),
        (4, 33, 4, 43, false),
    ]);
    buffer.insert(" (top kek) ");
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (1, 14, 1, 24, false),
        (3, 22, 3, 32, false),
        (4, 44, 4, 54, false),
    ]);
    reference_buffer.insert_for_test(1, 3, " (top kek) ");
    reference_buffer.insert_for_test(3, 11, " (top kek) ");
    reference_buffer.insert_for_test(4, 33, " (top kek) ");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_some_characters_after_selection() {
    let mut buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 13, true),
        (3, 11, 3, 21, true),
        (4, 33, 4, 43, true),
    ]);
    buffer.insert(" (top kek) ");
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 24, true),
        (3, 11, 3, 32, true),
        (4, 33, 4, 54, true),
    ]);
    reference_buffer.insert_for_test(1, 13, " (top kek) ");
    reference_buffer.insert_for_test(3, 21, " (top kek) ");
    reference_buffer.insert_for_test(4, 43, " (top kek) ");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_some_characters_with_newline() {
    let mut buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 3, true),
        (3, 11, 3, 11, true),
        (4, 33, 4, 33, true),
    ]);
    buffer.insert(" (top\nkek) ");
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (2, 6, 2, 6, true),
        (5, 6, 5, 6, true),
        (7, 6, 7, 6, true),
    ]);
    reference_buffer.insert_for_test(1, 3, " (top\nkek) ");
    reference_buffer.insert_for_test(4, 11, " (top\nkek) ");
    reference_buffer.insert_for_test(6, 33, " (top\nkek) ");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_some_characters_before_selection_with_newline() {
    let mut buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 13, false),
        (3, 11, 3, 21, false),
        (4, 33, 4, 43, false),
    ]);
    buffer.insert(" (top\nkek) ");
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (2, 6, 2, 16, false),
        (5, 6, 5, 16, false),
        (7, 6, 7, 16, false),
    ]);
    reference_buffer.insert_for_test(1, 3, " (top\nkek) ");
    reference_buffer.insert_for_test(4, 11, " (top\nkek) ");
    reference_buffer.insert_for_test(6, 33, " (top\nkek) ");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_insert_some_characters_after_selection_with_newline() {
    let mut buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 13, true),
        (3, 11, 3, 21, true),
        (4, 33, 4, 43, true),
    ]);
    buffer.insert(" (top\nkek) ");
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (1, 3, 2, 6, true),
        (4, 11, 5, 6, true),
        (6, 33, 7, 6, true),
    ]);
    reference_buffer.insert_for_test(1, 13, " (top\nkek) ");
    reference_buffer.insert_for_test(4, 21, " (top\nkek) ");
    reference_buffer.insert_for_test(6, 43, " (top\nkek) ");
    assert_eq!(buffer, reference_buffer);
}
// TODO: check multiple newlines in a row
//       check selection over multiple lines insert before/after
//       check possible overlaps (if do iteratively)
