mod merge;
mod movements;
use super::*;

fn gen_storage<'a, 'b: 'a, L: LineLengh>(line_length: &'b L) -> SelectionStorage<'a, L> {
    let mut storage: SelectionStorage<L> = SelectionStorage::new(line_length);
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
