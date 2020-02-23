use super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_delete() {
    let mut buffer = load_buffer_with_selections(&vec![(3, 20, 3, 20, true)]);
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![(3, 20, 3, 20, true)]);
    reference_buffer.delete_for_test(3, 20, 3, 20);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_delete_backward_selection() {
    let mut buffer = load_buffer_with_selections(&vec![(3, 10, 3, 20, false)]);
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![(3, 10, 3, 10, true)]);
    reference_buffer.delete_for_test(3, 10, 3, 20);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_delete_forward_selection() {
    let mut buffer = load_buffer_with_selections(&vec![(3, 10, 3, 20, true)]);
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![(3, 10, 3, 10, true)]);
    reference_buffer.delete_for_test(3, 10, 3, 20);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_delete_with_newline() {
    let mut buffer = load_buffer_with_selections(&vec![(3, 20, 3, 20, true)]);
    buffer.delete();
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![(3, 20, 3, 20, true)]);
    reference_buffer.delete_for_test(3, 20, 3, 20);
    reference_buffer.delete_for_test(3, 20, 3, 20);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_delete_backward_selection_with_newline() {
    let mut buffer = load_buffer_with_selections(&vec![(3, 10, 3, 21, false)]);
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![(3, 10, 3, 10, true)]);
    reference_buffer.delete_for_test(3, 10, 3, 21);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_delete_forward_selection_with_newline() {
    let mut buffer = load_buffer_with_selections(&vec![(3, 10, 3, 21, true)]);
    buffer.delete();
    let mut reference_buffer = load_buffer_with_selections(&vec![(3, 10, 3, 10, true)]);
    reference_buffer.delete_for_test(3, 10, 3, 21);
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_delete_everything() {
    let mut buffer = load_buffer();
    for _ in 0..1337 {
        buffer.delete();
    }
    let reference_buffer = Buffer::empty();
    assert_eq!(buffer, reference_buffer);
}

#[test]
fn test_delete_everything_after() {
    let mut buffer = load_buffer();
    buffer.move_right(10, false);
    for _ in 0..1337 {
        buffer.delete();
    }
    let mut reference_buffer = Buffer::from_reader("This will ".as_bytes()).unwrap();
    reference_buffer.move_right(10, false);
    assert_eq!(buffer, reference_buffer);
}
