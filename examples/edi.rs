use coredit::{Buffer, LineLength, Position};
use itertools::Itertools;
use std::convert::TryInto;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn position_to_char_idx(b: &Buffer, p: Position) -> usize {
    //let line_length = b.get_rope().line_length(p.line.get()).unwrap();
    b.get_rope().line_to_char(p.line.get() - 1) + p.col.get() - 1
}

fn main() {
    let sample_file = File::open("test_data/sample_text.txt").unwrap();
    let mut buffer = Buffer::from_reader(sample_file).unwrap();
    buffer.move_right(5, false);
    buffer.move_right(10, true);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "Help:\r\nShift-C: place new cursors under existing ones\r\nESC: quit\r\nPress any key to continue...\r\n").unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}{}",
            color::Bg(color::Reset),
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        for line in buffer.get_rope().lines_at(0) {
            write!(stdout, "{}\r", line);
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
            );
            let substr = buffer.get_rope().slice(from_ch..=to_ch);
            write!(
                stdout,
                "{}{}{}",
                termion::cursor::Goto(
                    s.head.col.get().try_into().unwrap(),
                    s.head.line.get().try_into().unwrap(),
                ),
                color::Bg(color::Blue),
                substr
            );

            //buffer.get_rope().line(k
        }

        // Overwrite with colored

        //for s in buffer.selections_iter().group_by(|x| x.head.line) {
        //    write!(
        //        stdout,
        //        "{}{}",
        //        termion::cursor::Goto((s.bounds().0).0.try_into().unwrap(), (s.bounds().0).1.try_into().unwrap()),
        //        color::Fg(color::Green)
        //    )
        //    .unwrap();
        //}

        match c.unwrap() {
            // Key::Char('q') => break,
            // Key::Char(c) => println!("{}", c),
            // Key::Alt(c) => println!("^{}", c),
            // Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => break,
            // Key::Left => println!("←"),
            // Key::Right => println!("→"),
            // Key::Up => println!("↑"),
            // Key::Down => println!("↓"),
            // Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
