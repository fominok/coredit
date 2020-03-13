use coredit::{Buffer, CursorDirection, LineLength, Position, Selection};
use cursive::event::{Event, EventResult};
use cursive::theme;
use cursive::traits::*;
use cursive::{Cursive, Printer};
use std::fs::File;

// This example define a custom view that prints any event it receives.
// This is a handy way to check the input received by cursive.

fn make_style(f: (u8, u8, u8), b: (u8, u8, u8)) -> theme::ColorStyle {
    theme::ColorStyle {
        front: theme::ColorType::Color(theme::Color::RgbLowRes(f.0, f.1, f.2)),
        back: theme::ColorType::Color(theme::Color::RgbLowRes(b.0, b.1, b.2)),
    }
}

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
        }
        previous_pos = int.1.successor(rope);
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
                i += 1;
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

#[derive(Clone, Copy, Debug)]
enum IntervalColor {
    Uncolored,
    Selection,
    Cursor,
}

// Let's implement the `View` trait.
// `View` contains many methods, but only a few are required.
impl View for KeyCodeView {
    fn draw(&self, printer: &Printer) {
        let mut selections_colors: Vec<ColoredInterval> = self
            .buffer
            .selections_iter()
            .map(|s| selection_to_colored_interval_pair(&self.buffer, s))
            .flatten()
            .collect();
        selections_colors = fill_missing_intervals(&self.buffer, &selections_colors);
        selections_colors = split_intervals_by_lines(&self.buffer, &selections_colors);

        for (from, to, color) in selections_colors.into_iter() {
            let ends_on_nl = to.is_line_end(self.buffer.get_rope());
            let mut slice: String = self
                .buffer
                .get_rope()
                .slice(
                    position_to_char_idx(&self.buffer, from)
                        ..=position_to_char_idx(&self.buffer, to) - if ends_on_nl { 1 } else { 0 },
                )
                .to_string();
            if ends_on_nl {
                slice.push(' ');
            }
            printer.with_color(
                match color {
                    IntervalColor::Uncolored => make_style((0, 0, 0), (5, 5, 5)),
                    IntervalColor::Selection => make_style((5, 5, 5), (0, 0, 5)),
                    IntervalColor::Cursor => make_style((5, 5, 5), (0, 0, 0)),
                },
                |printer| {
                    printer.print((from.col.get() - 1, from.line.get() - 1), &slice);
                },
            );
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        EventResult::Consumed(None)
    }
}
