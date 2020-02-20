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
#[ignore]
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

// TODO: test selection tail on another line

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
