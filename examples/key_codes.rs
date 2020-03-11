use coredit::{Buffer, CursorDirection, LineLength, Position, Selection};
use cursive::event::{Event, EventResult};
use cursive::traits::*;
use cursive::{Cursive, Printer};
use std::fs::File;

// This example define a custom view that prints any event it receives.
// This is a handy way to check the input received by cursive.

fn position_to_char_idx(b: &Buffer, p: Position) -> usize {
    ////let line_length = b.get_rope().line_length(p.line).unwrap();
    b.get_rope().line_to_char(p.line.get() - 1) + p.col.get() - 1
}

type ColoredInterval = (Position, Position, IntervalColor);

fn selection_to_colored_interval_pair(b: &Buffer, s: Selection) -> Vec<ColoredInterval> {
    match s.cursor_direction {
        CursorDirection::Forward => vec![
            (
                s.from,
                s.to.predecessor(b.get_rope()),
                IntervalColor::Selection,
            ),
            (s.to, s.to, IntervalColor::Cursor),
        ],
        CursorDirection::Backward => vec![
            (s.from, s.from, IntervalColor::Cursor),
            (
                s.from.successor(b.get_rope()),
                s.to,
                IntervalColor::Selection,
            ),
        ],
    }
}

fn fill_missing_intervals(b: &Buffer, intervals: &[ColoredInterval]) -> Vec<ColoredInterval> {
    let rope = b.get_rope();
    let last_pos = Position {
        line: rope.count().into(),
        col: rope.line_length(rope.count()).unwrap().into(),
    };
    let mut previous_pos = Position {
        line: 1.into(),
        col: 1.into(),
    };
    let mut result = vec![];

    for int in intervals.iter() {
        if int.0 > previous_pos {
            result.push((
                previous_pos,
                int.0.predecessor(rope),
                IntervalColor::Uncolored,
            ));
            previous_pos = int.1.successor(rope);
        }
        result.push(*int);
    }

    // Finish with the last inverval
    result.push((previous_pos, last_pos, IntervalColor::Uncolored));
    result
}

fn split_intervals_by_lines(b: &Buffer, intervals: &[ColoredInterval]) -> Vec<ColoredInterval> {
    let rope = b.get_rope();
    let mut result = vec![];
    for int in intervals.iter() {
        if int.0.line == int.1.line {
            result.push(*int);
        } else {
            // Put first possibly non-full line
            result.push((
                int.0,
                Position {
                    line: int.0.line,
                    col: rope.line_length(int.0.line.get()).unwrap().into(),
                },
                int.2,
            ));

            // Put multiple fill lines in between
            let mut i = int.0.line.get() + 1;
            while i < int.1.line.get() {
                result.push((
                    Position {
                        line: i.into(),
                        col: 1.into(),
                    },
                    Position {
                        line: i.into(),
                        col: rope.line_length(i).unwrap().into(),
                    },
                    int.2,
                ));
            }

            // Put the last possibly non-full line
            result.push((
                Position {
                    line: int.1.line,
                    col: 1.into(),
                },
                int.1,
                int.2,
            ));
        }
    }
    result
}

fn main() {
    better_panic::install();
    let sample_file = File::open("test_data/sample_text.txt").unwrap();

    let mut buffer = Buffer::from_reader(sample_file).unwrap();
    buffer.move_right(5, false);
    buffer.move_right(10, true);
    buffer.swap_cursor();

    let mut siv = Cursive::default();
    siv.add_layer(KeyCodeView::new(buffer).full_width().full_height());

    let mut theme = siv.current_theme().clone();
    theme.shadow = false;
    siv.set_theme(theme);

    siv.run();
}

// Our view will have a small history of the last events.
struct KeyCodeView {
    buffer: Buffer,
}

impl KeyCodeView {
    fn new(buffer: Buffer) -> Self {
        KeyCodeView { buffer }
    }
}

#[derive(Clone, Copy)]
enum IntervalColor {
    Uncolored,
    Selection,
    Cursor,
}

// Let's implement the `View` trait.
// `View` contains many methods, but only a few are required.
impl View for KeyCodeView {
    fn draw(&self, printer: &Printer) {
        let mut colored_intervals = vec![];
        let mut selections_colors = self
            .buffer
            .selections_iter()
            .map(|s| selection_to_colored_interval_pair(&self.buffer, s))
            .flatten();
        let first = selections_colors.next().unwrap();
        let mut last_pos = first.1;
        if first.0
            == (Position {
                col: 1.into(),
                line: 1.into(),
            })
        {
            colored_intervals.push(first);
        } else {
            colored_intervals.push((
                Position {
                    line: 1.into(),
                    col: 1.into(),
                },
                first.0.predecessor(self.buffer.get_rope()),
                IntervalColor::Uncolored,
            ));
            colored_intervals.push(first);
        }
        for s in selections_colors {
            if s.0 > last_pos.successor(self.buffer.get_rope()) {
                colored_intervals.push((
                    last_pos.successor(self.buffer.get_rope()),
                    s.0.predecessor(self.buffer.get_rope()),
                    IntervalColor::Uncolored,
                ));
            }
            last_pos = s.1;
            colored_intervals.push(s);
        }

        if self.buffer.get_rope().len_chars() > position_to_char_idx(&self.buffer, last_pos) {
            colored_intervals.push((
                last_pos.successor(self.buffer.get_rope()),
                Position {
                    col: 228.into(),
                    line: 228.into(),
                },
                IntervalColor::Uncolored,
            ));
        }

        // for (i, line) in self.buffer.get_rope().lines_at(0).enumerate() {
        //     printer.print((0, i), &line.to_string());
        // }
        for (from, to, color) in colored_intervals.into_iter() {
            let slices = &self
                .buffer
                .get_rope()
                .slice(
                    position_to_char_idx(&self.buffer, from)
                        ..=position_to_char_idx(&self.buffer, to),
                )
                .to_string();
            let mut slices_split = slices.split("\n");
            printer.print(
                (from.col.get() - 1, from.line.get() - 1),
                slices_split.next().unwrap(),
            );
            for (offset, slice) in slices_split.enumerate() {
                printer.print((0, from.line.get() - 1 + offset), slice);
            }
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        EventResult::Consumed(None)
    }
}
