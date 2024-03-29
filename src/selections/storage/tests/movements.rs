use super::*;
use crate::selections::storage::*;
use crate::selections::CursorDirection;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn test_move_left_no_intersections() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 200);
    line_length.insert(2, 200);
    line_length.insert(3, 200);
    line_length.insert(4, 200);
    line_length.insert(5, 200);
    let mut storage = gen_storage();

    storage.move_left(10, false, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        SelectionUnbound::new_quick(1, 20, 1, 20, Default::default()),
        SelectionUnbound::new_quick(2, 20, 2, 20, Default::default()),
        SelectionUnbound::new_quick(5, 120, 5, 120, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_move_right_no_intersections() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 200);
    line_length.insert(2, 200);
    line_length.insert(3, 200);
    line_length.insert(4, 200);
    line_length.insert(5, 200);
    let mut storage = gen_storage();

    storage.move_right(10, false, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        SelectionUnbound::new_quick(1, 40, 1, 40, Default::default()),
        SelectionUnbound::new_quick(2, 40, 2, 40, Default::default()),
        SelectionUnbound::new_quick(5, 140, 5, 140, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_move_down_no_intersections() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 200);
    line_length.insert(2, 200);
    line_length.insert(3, 200);
    line_length.insert(4, 200);
    line_length.insert(5, 200);
    line_length.insert(6, 200);
    let mut storage = gen_storage();

    storage.move_down(1, false, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        SelectionUnbound::new_quick(2, 30, 2, 30, Default::default()),
        SelectionUnbound::new_quick(3, 30, 3, 30, Default::default()),
        SelectionUnbound::new_quick(6, 130, 6, 130, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_move_up_no_intersections() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 200);
    line_length.insert(2, 200);
    line_length.insert(3, 200);
    line_length.insert(4, 200);
    line_length.insert(5, 200);
    line_length.insert(6, 200);
    let mut storage = SelectionStorage::gen_from_tuples(&vec![
        (4, 30, 4, 30, true),
        (5, 30, 5, 30, true),
        (8, 130, 8, 130, true),
    ]);

    storage.move_up(2, false, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        SelectionUnbound::new_quick(2, 30, 2, 30, Default::default()),
        SelectionUnbound::new_quick(3, 30, 3, 30, Default::default()),
        SelectionUnbound::new_quick(6, 130, 6, 130, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_move_left_intersection() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 9);
    line_length.insert(2, 9);
    line_length.insert(3, 9);
    let mut storage =
        SelectionStorage::gen_from_tuples(&vec![(1, 3, 1, 5, true), (2, 5, 2, 7, true)]);

    storage.move_left(12, true, &line_length);

    // They are right one after another but not intersected yet
    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .iter()
        .map(|x| x.0.clone())
        .collect();
    let selections_reference_vec = vec![
        SelectionUnbound::new_quick(1, 1, 1, 3, CursorDirection::Backward),
        SelectionUnbound::new_quick(1, 4, 2, 5, CursorDirection::Backward),
    ];

    assert_eq!(selections_vec, selections_reference_vec);

    // And move a little more
    storage.move_left(1, true, &line_length);
    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![SelectionUnbound::new_quick(
        1,
        1,
        2,
        5,
        CursorDirection::Backward,
    )];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_move_right_intersection() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 9);
    line_length.insert(2, 9);
    line_length.insert(3, 9);
    let mut storage =
        SelectionStorage::gen_from_tuples(&vec![(1, 3, 1, 5, true), (2, 5, 2, 7, true)]);

    storage.move_right(9, true, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .iter()
        .map(|x| x.0.clone())
        .collect();
    let selections_reference_vec = vec![SelectionUnbound::new_quick(
        1,
        3,
        3,
        7,
        CursorDirection::Forward,
    )];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_move_down_intersection() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 9);
    line_length.insert(2, 9);
    line_length.insert(3, 9);
    let mut storage =
        SelectionStorage::gen_from_tuples(&vec![(1, 3, 1, 5, true), (2, 5, 2, 7, true)]);

    storage.move_down(9, true, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .iter()
        .map(|x| x.0.clone())
        .collect();
    let selections_reference_vec = vec![SelectionUnbound::new_quick(
        1,
        3,
        3,
        7,
        CursorDirection::Forward,
    )];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_move_up_intersection() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 9);
    line_length.insert(2, 9);
    line_length.insert(3, 9);
    line_length.insert(4, 9);
    let mut storage =
        SelectionStorage::gen_from_tuples(&vec![(3, 3, 3, 5, true), (4, 5, 4, 7, true)]);

    storage.move_up(1, true, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .iter()
        .map(|x| x.0.clone())
        .collect();
    let selections_reference_vec = vec![
        SelectionUnbound::new_quick(2, 5, 3, 3, CursorDirection::Backward),
        SelectionUnbound::new_quick(3, 7, 4, 5, CursorDirection::Backward),
    ];

    assert_eq!(selections_vec, selections_reference_vec);

    storage.move_up(1, true, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .iter()
        .map(|x| x.0.clone())
        .collect();

    let selections_reference_vec = vec![SelectionUnbound::new_quick(
        1,
        5,
        4,
        5,
        CursorDirection::Backward,
    )];

    assert_eq!(selections_vec, selections_reference_vec);
}

#[test]
fn test_move_up_intersection_sticky() {
    // On merge sticky is preserved only for the first selection (by order)
    let mut line_length = HashMap::new();
    line_length.insert(1, 8);
    line_length.insert(2, 4);
    line_length.insert(3, 8);
    let mut storage =
        SelectionStorage::gen_from_tuples(&vec![(2, 2, 2, 4, false), (3, 7, 3, 8, false)]);

    storage.move_up(1, true, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .iter()
        .map(|x| x.0.clone())
        .collect();
    let selections_reference_vec = vec![SelectionUnbound::new_quick(
        1,
        2,
        3,
        8,
        CursorDirection::Backward,
    )];

    assert_eq!(selections_vec, selections_reference_vec);
}
