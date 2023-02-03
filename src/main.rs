use std::{
    io::{stdin, stdout, Write},
    process,
    sync::mpsc::{Receiver, Sender},
    thread,
    time::Duration,
};

use colored::{Color, Colorize};
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
                Signal::Quit => t.event_quit(),
                Signal::Rotate => t.event_rotate(),
                Signal::Left => t.event_left(),
                Signal::Right => t.event_right(),
                Signal::Accelerate => t.event_accelerate(),
                Signal::Sink => t.event_sink(),
            },
            Ok(None) | Err(_) => {}
        }

        TerminalPainter::draw_game(&t);

        if let GameStatus::Exit(ref e) = t.status {
            TerminalPainter::draw_record(&t);
            write!(stdout, "{}", termion::cursor::Show).unwrap();
            TerminalPainter::raw_write_fix(format!("{} {}", "[exit]".color(Color::Blue), e));
            process::exit(0);
        }
        counter += 1;
        thread::sleep(Duration::from_millis(10));
    }
}

fn main() {
    // 两个线程 A监听键盘事件 B游戏主线程 数据流向: A ===管道===> B
    let (tx, rx) = std::sync::mpsc::channel();
    let _cfg = match env::load() {
        Ok(v) => v,
        Err(e) => {
            println!("{} {}", "[config error]".color(Color::Red), e);
            process::exit(1);
        }
    };

    let mut t = Tetris::new(_cfg);

    t.start();

    // 发送者线程A
    thread::spawn(move || listen_key_event(tx));

    // 接受者线程B
    launch(t, rx)
}
