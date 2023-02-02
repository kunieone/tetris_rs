use std::{
    io::{stdin, stdout, Write},
    thread,
    time::Duration,
};

use crossterm::cursor;

// use crossterm::{
//     cursor,
//     event::{read, Event, KeyCode, KeyEvent, MediaKeyCode, ModifierKeyCode},
//     execute,
//     terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
// };
use game::{GameStatus, Tetris};
use termion::{input::TermRead, raw::IntoRawMode};

pub mod bricks;
pub mod display;
pub mod game;
pub mod record;

#[derive(Debug)]
pub enum Signal {
    Quit,

    Rotate,

    Left,

    Right,

    Accelerate,

    Sink,
}

pub struct RawTerminalScreen {}

impl RawTerminalScreen {
    fn raw_write_fix(&self, input_string: String) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        for line in input_string.lines() {
            write!(
                stdout,
                "{}\n{}",
                line,
                termion::cursor::Left(line.len() as u16)
            )
            .unwrap();
        }
    }

    pub fn display(&self, t: &Tetris) {
        self.raw_write_fix(t.to_string())
    }
}

fn main() {
    let screen = RawTerminalScreen {};

    let (tx, rx) = std::sync::mpsc::channel();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    thread::spawn(move || {
        for c in stdin.keys() {
            match c.unwrap() {
                termion::event::Key::Up => tx.send(Some(Signal::Rotate)).unwrap(),
                termion::event::Key::Down => tx.send(Some(Signal::Accelerate)).unwrap(),
                termion::event::Key::Left => tx.send(Some(Signal::Left)).unwrap(),
                termion::event::Key::Right => tx.send(Some(Signal::Right)).unwrap(),
                termion::event::Key::Char(' ') => tx.send(Some(Signal::Sink)).unwrap(),
                termion::event::Key::Ctrl('c') | termion::event::Key::Char('q') => {
                    tx.send(Some(Signal::Quit)).unwrap()
                }
                _ => tx.send(None).unwrap(),
            };
        }
    });

    //
    let mut t = Tetris::new(13, 20);
    t.start();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    //
    let mut counter = 0;

    loop {

        if counter % (100) == 0 {
            t.update()
        }
        if t.status == GameStatus::Exit {
            screen.raw_write_fix(format!("{}", t.record));
            break;
        }

        // 接收管道内容
        match rx.try_recv() {
            Ok(Some(signal)) => match signal {
                Signal::Quit => {
                    t.status = GameStatus::Exit;
                }
                Signal::Rotate => t.event_rotate(),
                Signal::Left => t.event_left(),
                Signal::Right => t.event_right(),
                Signal::Accelerate => t.accelerate(),
                Signal::Sink => t.event_sink(),
            },
            Ok(None) | Err(_) => {}
        }

        clear_screen();
        screen.display(&t);
        counter += 1;
        // write!(
        //     stdout,
        //     "{}",
        //     crossterm::cursor::MoveRight(t.board.width as u16 + 4),
        // )
        // .unwrap();
        // let mut list = "".to_string();
        // for e in t.following_bricks.iter() {
        //     list.push('\n');
        //     list += &e.display();
        // }
        // write!(stdout, "{}", crossterm::cursor::MoveTo(0, 0)).unwrap();
        thread::sleep(Duration::from_millis(10));
    }
    write!(
        stdout,
        "{}{}",
        crossterm::cursor::MoveToColumn(0),
        termion::cursor::Show
    )
    .unwrap();
}

fn clear_screen() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(0, 0),
        termion::clear::All,
        // cursor::MoveToColumn(0)
    )
    .unwrap();
}



















// //
// fn add_strings(str1: &str, str2: &str, gap: usize) -> String {
//     let lines1 = str1.split("\n");
//     let lines2 = str2.split("\n");
//     let mut result = Vec::new();

//     let max_len = std::cmp::max(lines1.clone().count(), lines2.clone().count());
//     for i in 0..max_len {
//         let line1 = match lines1.clone().nth(i) {
//             Some(l) => l,
//             None => "",
//         };
//         let line2 = match lines2.clone().nth(i) {
//             Some(l) => l,
//             None => "",
//         };
//         let spacing = if gap > line1.len() {
//             " ".repeat(gap - line1.len())
//         } else {
//             "".to_string()
//         };
//         result.push(format!("{}{}  {}", line1, spacing, line2));
//     }

//     result.join("\n")
// }
// #[test]
// fn add_string_test() {
//     let a = "Good Job\nYou are smart!";
//     let b = "Bad Job\nYou are stupid!";
//     println!("{}", add_strings(a, b, 2));
// }
