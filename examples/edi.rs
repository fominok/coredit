use coredit::{BindedPosition, BindedSelection, Buffer, CursorDirection};
use cursive::event::{self, Event, EventResult};
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

fn position_to_char_idx(b: &Buffer, p: BindedPosition) -> usize {
    ////let line_length = b.get_rope().line_length(p.line).unwrap();
    b.get_rope().line_to_char(p.line() - 1) + p.col() - 1
}

type ColoredInterval<'a> = (BindedPosition<'a>, BindedPosition<'a>, IntervalColor);

fn selection_to_colored_interval_pair<'a>(s: BindedSelection<'a>) -> Vec<ColoredInterval<'a>> {
    let is_point = s.is_point();
    let cursor_direction = s.cursor_direction();
    let (from, to) = s.coords();
    if is_point {
        vec![(to, to, IntervalColor::Cursor)]
    } else {
        match cursor_direction {
            CursorDirection::Forward => vec![
                (from, to.predecessor().unwrap(), IntervalColor::Selection),
                (to, to, IntervalColor::Cursor),
            ],
            CursorDirection::Backward => vec![
                (from, from, IntervalColor::Cursor),
                (from.successor().unwrap(), to, IntervalColor::Selection),
            ],
        }
    }
}

fn fill_missing_intervals<'a>(
    b: &'a Buffer,
    intervals: Vec<ColoredInterval<'a>>,
) -> Vec<ColoredInterval<'a>> {
    let last_pos = b.create_position(b.lines_count(), b.line_length(b.lines_count()).unwrap());
    let mut previous_pos = Some(b.create_position(1, 1));
    let mut result = vec![];

    for int in intervals.iter() {
        if let Some(pos) = previous_pos {
            if int.0 > pos {
                result.push((pos, int.0.predecessor().unwrap(), IntervalColor::Uncolored));
            }
            previous_pos = int.1.successor();
            result.push(*int);
        } else {
            break;
        }
    }

    // Finish with the last inverval
    if let Some(pos) = previous_pos {
        result.push((pos, last_pos, IntervalColor::Uncolored));
    }
    result
}

fn split_intervals_by_lines<'a>(
    b: &'a Buffer,
    intervals: Vec<ColoredInterval<'a>>,
) -> Vec<ColoredInterval<'a>> {
    let mut result = vec![];
    for int in intervals.iter() {
        if int.0.line() == int.1.line() {
            result.push(*int);
        } else {
            // Put first possibly non-full line
            result.push((
                int.0,
                b.create_position(int.0.line(), b.line_length(int.0.line()).unwrap()),
                int.2,
            ));

            // Put multiple fill lines in between
            let mut i = int.0.line() + 1;
            while i < int.1.line() {
                result.push((
                    b.create_position(i, 1),
                    b.create_position(i, b.line_length(i).unwrap()),
                    int.2,
                ));
                i += 1;
            }

            // Put the last possibly non-full line
            result.push((b.create_position(int.1.line(), 1), int.1, int.2));
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
            .map(|s| selection_to_colored_interval_pair(s))
            .flatten()
            .collect();
        selections_colors = fill_missing_intervals(&self.buffer, selections_colors);
        selections_colors = split_intervals_by_lines(&self.buffer, selections_colors);

        for (from, to, color) in selections_colors.into_iter() {
            let ends_on_nl = to.is_line_end();
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
                    printer.print((from.col() - 1, from.line() - 1), &slice);
                },
            );
        }
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        match e {
            Event::Key(k) => match k {
                event::Key::Left => self.buffer.move_left(1, false),
                event::Key::Right => self.buffer.move_right(1, false),
                event::Key::Up => self.buffer.move_up(1, false),
                event::Key::Down => self.buffer.move_down(1, false),
                event::Key::Del => self.buffer.delete(),
                event::Key::Backspace => {
                    self.buffer.delete();
                    self.buffer.move_left(1, false);
                }
                _ => {}
            },
            Event::Shift(k) => match k {
                event::Key::Left => self.buffer.move_left(1, true),
                event::Key::Right => self.buffer.move_right(1, true),
                event::Key::Up => self.buffer.move_up(1, true),
                event::Key::Down => self.buffer.move_down(1, true),
                _ => {}
            },
            Event::AltChar(c) => match c {
                'c' => self.buffer.place_selection_under(),
                _ => {}
            },
            Event::Char(c) => self.buffer.insert(&c.to_string()),
            _ => {}
        }
        EventResult::Consumed(None)
    }
}
