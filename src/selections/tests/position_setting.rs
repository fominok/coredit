use super::super::*;
use pretty_assertions::assert_eq;

#[test]
fn test_set_straight_ahead() {
    let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
    forward.set(6, 45, true);
    assert_eq!(
        forward,
        Selection::new_quick(4, 10, 6, 45, CursorDirection::Forward)
    );

    let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
    backward.set(4, 5, true);
    assert_eq!(
        backward,
        Selection::new_quick(4, 5, 6, 20, CursorDirection::Backward)
    );
}

#[test]
fn test_set_shrink() {
    let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
    forward.set(5, 20, true);
    assert_eq!(
        forward,
        Selection::new_quick(4, 10, 5, 20, CursorDirection::Forward)
    );

    let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
    backward.set(5, 15, true);
    assert_eq!(
        backward,
        Selection::new_quick(5, 15, 6, 20, CursorDirection::Backward)
    );
}

#[test]
fn test_set_reverse() {
    let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
    forward.set(3, 30, true);
    assert_eq!(
        forward,
        Selection::new_quick(3, 30, 4, 10, CursorDirection::Backward)
    );

    let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
    backward.set(6, 35, true);
    assert_eq!(
        backward,
        Selection::new_quick(6, 20, 6, 35, CursorDirection::Forward)
    );
}

#[test]
fn test_non_expand() {
    let mut forward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Forward);
    forward.set(6, 21, false);
    assert_eq!(
        forward,
        Selection::new_quick(6, 21, 6, 21, CursorDirection::Forward)
    );

    let mut backward = Selection::new_quick(4, 10, 6, 20, CursorDirection::Backward);
    backward.set(2, 5, false);
    assert_eq!(
        backward,
        Selection::new_quick(2, 5, 2, 5, CursorDirection::Forward)
    );
}
