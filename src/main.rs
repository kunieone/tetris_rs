#![feature(exclusive_range_pattern)]
use std::{
    io::{stdin, stdout, Write},
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use display::TerminalPainter;
use game::{GameStatus, Tetris};
use termion::{input::TermRead, raw::IntoRawMode};

pub mod bricks;
pub mod display;
pub mod env;
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

fn clear_screen() {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}{}",
        termion::scroll::Up(10),
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
    .unwrap();
}

fn listen_key_event(tx: Sender<Option<Signal>>) {
    let stdin = stdin();
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
}

fn launch(mut t: Tetris, rx: Receiver<Option<Signal>>) {
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    //
    let mut counter = 0;

    loop {
        clear_screen();
        t.update_by(counter);

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

        TerminalPainter::draw_game(&t);

        if t.status == GameStatus::Exit {
            TerminalPainter::draw_record(&t);
            write!(stdout, "{}", termion::cursor::Show).unwrap();

            break;
        }
        counter += 1;
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    let _cfg = env::load().unwrap();

    let mut t = Tetris::new(_cfg);

    t.start();

    thread::spawn(move || listen_key_event(tx));
    launch(t, rx)
}
