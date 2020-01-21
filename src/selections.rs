//! Selections implementation
use super::Position;
use std::collections::HashMap;

// implementation notes:
//
// selections are necessarily looks in one direction,
// thus there are no any two selection for on of which
// head is before tail and another with head after tail
//
// TOOD: maybe use BTreeMap with range to find a selection
// which will be intersected with a new one on `add_selection`

/// As selections within the buffer are not independent
/// (can be merged, for instance) this structure is aimed
/// to take special care of it
pub struct SelectionStorage {
    indexed_selections: HashMap<usize, HashMap<usize, Selection>>,
}

impl Default for SelectionStorage {
    fn default() -> Self {
        let mut lines_hm = HashMap::new();
        let mut cols_hm = HashMap::new();
        cols_hm.insert(0, Default::default());
        lines_hm.insert(0, cols_hm);
        SelectionStorage {
            indexed_selections: lines_hm,
        }
    }
}

impl SelectionStorage {
    pub fn add_selection(&mut self, s: Selection) {
        todo!()
    }
}

/// Selection simply is as pair of positions, which are
/// pairs of line/column values. Note that there is no
/// information about underlying text, words and even movements.
#[derive(Default)]
pub struct Selection {
    head: Position,
    tail: Position,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_merge_forward() {

    }
}
