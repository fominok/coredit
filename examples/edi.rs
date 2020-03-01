use coredit::Buffer;
use itertools::Itertools;
use std::convert::TryInto;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let sample_file = File::open("test_data/sample_text.txt").unwrap();
    let mut buffer = Buffer::from_reader(sample_file).unwrap();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "Help:\r\nShift-C: place new cursors under existing ones\r\nESC: quit\r\nPress any key to continue...\r\n").unwrap();

    for c in stdin.keys() {
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All,
        )
        .unwrap();

        for line in buffer.get_rope().lines_at(1) {
            write!(stdout, "{}\r", line);
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
            Key::Char('q') => break,
            Key::Char(c) => println!("{}", c),
            Key::Alt(c) => println!("^{}", c),
            Key::Ctrl(c) => println!("*{}", c),
            Key::Esc => println!("ESC"),
            Key::Left => println!("←"),
            Key::Right => println!("→"),
            Key::Up => println!("↑"),
            Key::Down => println!("↓"),
            Key::Backspace => println!("×"),
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
