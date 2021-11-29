use super::super::*;
use pretty_assertions::assert_eq;
use std::collections::HashMap;

#[test]
fn test_main_selection_merge() {
    let mut line_length = HashMap::new();
    line_length.insert(1, 30);
    line_length.insert(2, 30);
    line_length.insert(3, 30);
    line_length.insert(4, 30);
    line_length.insert(5, 30);

    let mut storage = SelectionStorage::new();
    let mut tree = BTreeSet::new();

    tree.insert(SelectionIntersect(SelectionUnbound::new_quick(
        1,
        10,
        1,
        20,
        Default::default(),
    )));
    tree.insert(SelectionIntersect(SelectionUnbound::new_quick(
        3,
        10,
        3,
        20,
        Default::default(),
    )));
    tree.insert(SelectionIntersect(SelectionUnbound::new_quick(
        4,
        2,
        4,
        3,
        Default::default(),
    )));

    storage.selections_tree = tree;
    storage.main_selection_ptr = PositionUnbound {
        line: 4.into(),
        col: 2.into(),
    };

    storage.move_down(1, true, &line_length);

    let selections_vec: Vec<SelectionUnbound> = storage
        .selections_tree
        .into_iter()
        .map(|x| x.into())
        .collect();

    let selections_reference_vec = vec![
        SelectionUnbound::new_quick(1, 10, 2, 20, Default::default()),
        SelectionUnbound::new_quick(3, 10, 5, 3, Default::default()),
    ];

    assert_eq!(selections_vec, selections_reference_vec);
    assert_eq!(
        storage.main_selection_ptr,
        PositionUnbound {
            line: 3.into(),
            col: 10.into()
        }
    );
}
