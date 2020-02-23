use super::*;
use better_panic;
use pretty_assertions::assert_eq;

#[test]
fn test_delete_single_line() {
    let mut buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 4, true),
        (1, 7, 1, 8, true),
        (1, 56, 1, 57, true),
    ]);
    buffer.delete();
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 3, true),
        (1, 4, 1, 4, true),
        (1, 50, 1, 50, true),
    ]);
    reference_buffer.delete_for_test(1, 1, 1, 59);
    reference_buffer.insert_for_test(1, 1, "Thw be used to check how ropey represents empty ls");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_delete_single_line_merge() {
    let mut buffer = load_buffer_with_selections(&vec![
        (1, 3, 1, 4, true),
        (1, 7, 1, 8, true),
        (1, 56, 1, 57, true),
    ]);
    buffer.delete();
    buffer.delete();
    buffer.delete();
    let mut reference_buffer =
        load_buffer_with_selections(&vec![(1, 3, 1, 3, true), (1, 48, 1, 48, true)]);
    reference_buffer.delete_for_test(1, 1, 1, 59);
    reference_buffer.insert_for_test(1, 1, "Thbe used to check how ropey represents empty l");
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_deletion_multiple_lines() {
    let mut buffer = load_buffer_with_selections(&vec![
        (3, 10, 3, 20, true),
        (4, 10, 4, 20, true),
        (5, 10, 5, 20, true),
    ]);
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (3, 10, 3, 10, true),
        (4, 10, 4, 10, true),
        (5, 10, 5, 10, true),
    ]);
    reference_buffer.delete_for_test(3, 10, 3, 20);
    reference_buffer.delete_for_test(4, 10, 4, 20);
    reference_buffer.delete_for_test(5, 10, 5, 20);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_deletion_multiple_multiline_selections() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 1, 3, 5, true), (4, 1, 6, 5, true)]);
    buffer.delete();
    let mut reference_buffer =
        load_buffer_with_selections(&vec![(1, 1, 1, 1, true), (2, 1, 2, 1, true)]);
    reference_buffer.delete_for_test(1, 1, 3, 5);
    reference_buffer.delete_for_test(2, 1, 4, 5);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_deletion_multiple_selections_end_on_newline() {
    let mut buffer = load_buffer_with_selections(&vec![
        (3, 15, 3, 21, true),
        (4, 15, 4, 21, true),
        (5, 15, 5, 21, true),
    ]);
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![
        (3, 15, 3, 15, true),
        (3, 29, 3, 29, true),
        (4, 15, 4, 15, true),
    ]);
    reference_buffer.delete_for_test(3, 15, 3, 21);
    reference_buffer.delete_for_test(3, 29, 3, 35);
    reference_buffer.delete_for_test(4, 15, 4, 21);
    assert_eq!(buffer, reference_buffer);
}

#[test]
#[ignore]
fn test_deletion_multiple_multiline_selections_sharing_same_line() {
    let mut buffer = load_buffer_with_selections(&vec![(1, 55, 4, 34, true), (4, 63, 6, 16, true)]);
    buffer.delete();
    let mut reference_buffer =
        load_buffer_with_selections(&vec![(1, 55, 1, 55, true), (1, 83, 1, 83, true)]);
    reference_buffer.delete_for_test(1, 55, 4, 34);
    reference_buffer.delete_for_test(1, 83, 3, 16);
    assert_eq!(buffer, reference_buffer);
}

#[test]
#[ignore]
fn test_deletion_merge() {
    better_panic::install();

    let mut buffer = load_buffer_with_selections(&vec![
        (1, 5, 1, 10, true),
        (3, 5, 3, 10, true),
        (4, 5, 4, 10, true),
    ]);
    for _ in 1..1337 {
        buffer.delete();
    }
    let mut reference_buffer = Buffer::from_reader("This ".as_bytes()).unwrap();
    reference_buffer.move_right(4, false);
    assert_eq!(buffer, reference_buffer);
}
