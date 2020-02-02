use super::*;
use crate::selections::storage::*;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn test_move_left_no_intersections() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 200);
    line_length.insert(2, 200);
    line_length.insert(3, 200);
    let mut storage = gen_storage(&line_length);

    storage.move_left(10, false);

    let selections_vec: Vec<Selection> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();
    let selections_reference_vec = vec![
        Selection::new_quick(1, 20, 1, 20, Default::default()),
        Selection::new_quick(2, 20, 2, 20, Default::default()),
        Selection::new_quick(5, 120, 5, 120, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}
