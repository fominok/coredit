use super::super::*;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn test_place_selections_under() {
    let mut line_length = HashMap::new();
    line_length.insert(4, 83);
    line_length.insert(5, 25);
    line_length.insert(6, 84);
    line_length.insert(7, 72);
    line_length.insert(8, 53);

    let mut storage = SelectionStorage::new();
    let mut tree = BTreeSet::new();

    tree.insert(SelectionIntersect(Selection::new_quick(
        4,
        7,
        4,
        8,
        Default::default(),
    )));
    tree.insert(SelectionIntersect(Selection::new_quick(
        4,
        76,
        4,
        77,
        Default::default(),
    )));
    tree.insert(SelectionIntersect(Selection::new_quick(
        4,
        81,
        4,
        82,
        Default::default(),
    )));

    storage.selections_tree = tree;

    storage.place_selection_under(&line_length);
    storage.place_selection_under(&line_length);

    let selections_vec: Vec<Selection> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();

    let selections_reference_vec = vec![
        Selection::new_quick(4, 7, 4, 8, Default::default()),
        Selection::new_quick(4, 76, 4, 77, Default::default()),
        Selection::new_quick(4, 81, 4, 82, Default::default()),
        Selection::new_quick(5, 7, 5, 8, Default::default()),
        Selection::new_quick(6, 7, 6, 8, Default::default()),
        Selection::new_quick(6, 76, 6, 77, Default::default()),
        Selection::new_quick(6, 81, 6, 82, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
}
