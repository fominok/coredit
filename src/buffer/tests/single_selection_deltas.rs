use super::*;
use crate::{selections::SelectionUnbound, Delta};
use itertools::izip;
use pretty_assertions::assert_eq;

fn assert_selections_changed(
    deltas: Vec<Delta>,
    selections_before: Vec<SelectionUnbound>,
    selections_after: Vec<SelectionUnbound>,
) {
    let buffer = deltas[0].buffer();
    assert!(deltas.len() == selections_before.len() && deltas.len() == selections_after.len());
    for (d, sb, sa) in izip!(
        deltas.into_iter(),
        selections_before.into_iter(),
        selections_after.into_iter()
    ) {
        assert_eq!(
            d,
            Delta::SelectionChanged {
                old: sb.binded(buffer),
                new: sa.binded(buffer)
            }
        );
    }
}

#[test]
fn test_selection_changed_deltas() {
    let mut buffer = load_buffer();
    // It should be raw as buffer will be modified next
    let selections_before: Vec<SelectionUnbound> =
        buffer.selections_iter().map(|s| s.selection).collect();
    let deltas = { buffer.move_right(30, false) };
    let selections_after: Vec<SelectionUnbound> = deltas[0]
        .buffer()
        .selections_iter()
        .map(|s| s.selection)
        .collect();
    assert_selections_changed(deltas, selections_before, selections_after);
}
