use super::*;
use better_panic;
use pretty_assertions::assert_eq;

#[test]
fn test_deletion() {
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
