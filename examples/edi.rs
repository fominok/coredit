use better_panic;
use coredit::{Buffer, CursorDirection, LineLength, Position};
use std::convert::TryInto;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// TODO: draw line and selections part in it simultaneously as insertion with NL
// as directly as characters doesn't handle CR

fn position_to_char_idx(b: &Buffer, p: Position) -> usize {
    //let line_length = b.get_rope().line_length(p.line.get()).unwrap();
    b.get_rope().line_to_char(p.line.get() - 1) + p.col.get() - 1
}

fn main() {
    better_panic::install();
    let sample_file = File::open("test_data/sample_text.txt").unwrap();
    let mut buffer = Buffer::from_reader(sample_file).unwrap();
    buffer.move_right(5, false);
    buffer.move_right(10, true);
    buffer.swap_cursor();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "Help:\r\nShift-C: place new cursors under existing ones\r\nESC: quit\r\nPress any key to continue...\r\n").unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            // Key::Char('q') => break,
            // Key::Char(c) => println!("{}", c),
            // Key::Alt(c) => println!("^{}", c),
            // Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => break,
            //Key::Left => buffer.move_left(1, false),
            //Key::Right => buffer.move_right(1, false),
            //Key::Up => buffer.move_up(1, false),
            //Key::Down => buffer.move_down(1, false),
            Key::Alt(c) => match c {
                'h' => buffer.move_left(1, false),
                'l' => buffer.move_right(1, false),
                'k' => buffer.move_up(1, false),
                'j' => buffer.move_down(1, false),
                _ => {}
            },
            Key::Char(c) => match c {
                'H' => buffer.move_left(1, true),
                'L' => buffer.move_right(1, true),
                'K' => buffer.move_up(1, true),
                'J' => buffer.move_down(1, true),
                _ => {}
            },
            // Key::Backspace => println!("×"),
            _ => {}
        }

        write!(
            stdout,
            "{}{}{}",
            color::Bg(color::Reset),
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        for line in buffer.get_rope().lines_at(0) {
            write!(stdout, "{}\r", line).unwrap();
        }

        for s in buffer.selections_iter() {
            let from_ch = position_to_char_idx(&buffer, s.head);
            let to_ch = position_to_char_idx(&buffer, s.tail);
            write!(
                stdout,
                "{} {} {}",
                termion::cursor::Goto(50, 50),
                from_ch,
                to_ch
            )
            .unwrap();
            let mut first_char = buffer.get_rope().char(from_ch);
            if buffer.get_rope().line_length(s.head.line.get()).unwrap() == s.head.col.get() {
                first_char = ' ';
            }

            // Highlight cursor if selection is reversed
            if s.cursor_direction == CursorDirection::Forward {
                write!(stdout, "{}", color::Bg(color::Blue)).unwrap();
            } else {
                write!(stdout, "{}", color::Bg(color::Green)).unwrap();
            }
            write!(
                stdout,
                "{}{}",
                termion::cursor::Goto(
                    s.head.col.get().try_into().unwrap(),
                    s.head.line.get().try_into().unwrap(),
                ),
                first_char
            )
            .unwrap();
            if from_ch != to_ch {
                let substr = buffer.get_rope().slice(from_ch + 1..to_ch);
                let last_char = buffer.get_rope().char(to_ch);
                // Draw other selection's data
                write!(
                    stdout,
                    "{}{}{}",
                    termion::cursor::Goto(
                        TryInto::<u16>::try_into(s.head.col.get()).unwrap() + 1u16,
                        TryInto::<u16>::try_into(s.head.line.get()).unwrap(),
                    ),
                    color::Bg(color::Blue),
                    substr
                )
                .unwrap();
                // Highlight cursor if cursor is in the selection's end

                if s.cursor_direction == CursorDirection::Backward {
                    write!(stdout, "{}", color::Bg(color::Blue)).unwrap();
                } else {
                    write!(stdout, "{}", color::Bg(color::Green)).unwrap();
                }
                write!(
                    stdout,
                    "{}{}",
                    termion::cursor::Goto(
                        s.tail.col.get().try_into().unwrap(),
                        s.tail.line.get().try_into().unwrap(),
                    ),
                    last_char
                )
                .unwrap();
            }
        }
        write!(stdout, "{}", termion::cursor::Hide).unwrap();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
