mod add_selections;
mod merge;
mod misc;
mod movements;
use super::*;

fn gen_storage() -> SelectionStorage {
    let mut storage = SelectionStorage::new();
    let mut tree = BTreeSet::new();
    tree.insert(SelectionIntersect(SelectionUnbound::new_quick(
        1,
        10,
        1,
        30,
        Default::default(),
    )));
    tree.insert(SelectionIntersect(SelectionUnbound::new_quick(
        2,
        10,
        2,
        30,
        Default::default(),
    )));
    tree.insert(SelectionIntersect(SelectionUnbound::new_quick(
        3,
        10,
        5,
        130,
        Default::default(),
    )));
    storage.selections_tree = tree;

    storage
}
