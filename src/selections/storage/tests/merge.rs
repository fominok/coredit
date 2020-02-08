use super::*;
use crate::selections::storage::*;
use crate::selections::CursorDirection;

#[test]
fn test_selection_intersect_partial_eq_forward() {
    let a = Selection::new_quick(87, 7, 88, 8, Default::default());
    let b = Selection::new_quick(87, 97, 105, 35, Default::default());
    assert!(SelectionIntersect(a) == SelectionIntersect(b))
}

#[test]
fn test_selection_intersect_partial_eq_backward() {
    let b = Selection::new_quick(87, 7, 88, 8, Default::default());
    let a = Selection::new_quick(87, 97, 105, 35, Default::default());
    assert!(SelectionIntersect(a) == SelectionIntersect(b))
}

#[test]
fn test_selection_intersect_partial_eq_corner() {
    let a = Selection::new_quick(87, 7, 88, 8, Default::default());
    let b = Selection::new_quick(88, 8, 105, 35, Default::default());
    assert!(SelectionIntersect(a) == SelectionIntersect(b))
}

#[test]
fn test_selection_intersect_ord_lt() {
    let a = Selection::new_quick(87, 7, 88, 8, Default::default());
    let b = Selection::new_quick(88, 9, 105, 35, Default::default());
    assert!(SelectionIntersect(a) < SelectionIntersect(b))
}

#[test]
fn test_selection_intersect_ord_gt() {
    let b = Selection::new_quick(87, 7, 88, 8, Default::default());
    let a = Selection::new_quick(88, 9, 105, 35, Default::default());
    assert!(SelectionIntersect(a) > SelectionIntersect(b))
}

#[test]
fn test_selection_storage_search_some() {
    let storage = gen_storage();
    assert_eq!(
        *storage
            .find_hit(Position {
                line: 3.into(),
                col: 100.into()
            })
            .unwrap(),
        Selection {
            head: Position {
                line: 3.into(),
                col: 10.into()
            },
            tail: Position {
                line: 5.into(),
                col: 130.into()
            },
            cursor_direction: CursorDirection::Forward,
            sticky_column: None,
        }
    );
}

#[test]
fn test_selection_storage_search_none() {
    let storage = gen_storage();
    assert!(storage
        .find_hit(Position {
            line: 2.into(),
            col: 50.into()
        })
        .is_none());
}

#[test]
fn test_merge_head() {
    let mut storage = gen_storage();
    let s = Selection::new_quick(2, 25, 2, 100, Default::default());
    storage.add_selection(s);

    // Unwrapped from newtype to provide intuitive comparison
    let selections_vec: Vec<Selection> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        Selection::new_quick(1, 10, 1, 30, Default::default()),
        Selection::new_quick(2, 10, 2, 100, Default::default()),
        Selection::new_quick(3, 10, 5, 130, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_merge_tail() {
    let mut storage = gen_storage();
    let s = Selection::new_quick(2, 50, 4, 20, Default::default());
    storage.add_selection(s);

    // Unwrapped from newtype to provide intuitive comparison
    let selections_vec: Vec<Selection> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        Selection::new_quick(1, 10, 1, 30, Default::default()),
        Selection::new_quick(2, 10, 2, 30, Default::default()),
        Selection::new_quick(2, 50, 5, 130, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_merge_miss() {
    let mut storage = gen_storage();
    let s = Selection::new_quick(2, 40, 3, 5, Default::default());
    storage.add_selection(s);

    // Unwrapped from newtype to provide intuitive comparison
    let selections_vec: Vec<Selection> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        Selection::new_quick(1, 10, 1, 30, Default::default()),
        Selection::new_quick(2, 10, 2, 30, Default::default()),
        Selection::new_quick(2, 40, 3, 5, Default::default()),
        Selection::new_quick(3, 10, 5, 130, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_merge_both() {
    let mut storage = gen_storage();
    let s = Selection::new_quick(2, 20, 3, 20, Default::default());
    storage.add_selection(s);

    // Unwrapped from newtype to provide intuitive comparison
    let selections_vec: Vec<Selection> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        Selection::new_quick(1, 10, 1, 30, Default::default()),
        Selection::new_quick(2, 10, 5, 130, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}
