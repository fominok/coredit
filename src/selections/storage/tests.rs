mod merge;
mod movements;
use super::*;
use crate::selections::CursorDirection;
use std::ops::Deref;

fn gen_storage() -> SelectionStorage {
    let mut storage = SelectionStorage::new();
    let mut tree = BTreeSet::new();
    tree.insert(SelectionIntersect(Selection::new_quick(
        1,
        10,
        1,
        30,
        Default::default(),
    )));
    tree.insert(SelectionIntersect(Selection::new_quick(
        2,
        10,
        2,
        30,
        Default::default(),
    )));
    tree.insert(SelectionIntersect(Selection::new_quick(
        3,
        10,
        5,
        130,
        Default::default(),
    )));
    storage.selections_tree = tree;

    storage
}

type SelectionQuick = (usize, usize, usize, usize, bool);

fn gen_storage_from_tuples(selections: &[SelectionQuick]) -> SelectionStorage {
    let mut storage = SelectionStorage::new();
    let mut tree = BTreeSet::new();
    for s in selections {
        tree.insert(SelectionIntersect(Selection::new_quick(
            s.0,
            s.1,
            s.2,
            s.3,
            if s.4 {
                CursorDirection::Forward
            } else {
                CursorDirection::Backward
            },
        )));
    }
    storage.selections_tree = tree;

    storage
}
