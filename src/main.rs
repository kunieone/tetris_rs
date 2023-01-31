use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode},
    execute,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};
use game::Tetris;

pub mod display;
pub mod game;

#[derive(Debug)]
pub enum Signal {
    Quit,

    Rotate,

    Left,

    Right,

    Accelerate,

    Sink,
}

fn clear_screen() -> crossterm::Result<()> {
    execute!(stdout(), terminal::Clear(ClearType::All))?;
    execute!(stdout(), cursor::MoveTo(0, 0))
}

pub struct RawTerminalScreen {}

impl RawTerminalScreen {
    fn raw_write_fix(&self, input_string: String) {
        // Enable raw mode
        let mut stdout = std::io::stdout();

        for line in input_string.lines() {
            // Move the cursor up and back to the start of the line
            write!(stdout, "{}", cursor::MoveToColumn(0)).unwrap();
            // Write the line
            writeln!(stdout, "{}", line).unwrap();
        }
    }
    pub fn display(&self, t: &Tetris) {
        self.raw_write_fix(t.to_string())
    }
}

fn main() {
    let mut t = Tetris::new(15, 20);
    t.start();
    let screen = RawTerminalScreen {};

    let (tx, rx) = std::sync::mpsc::channel();
    thread::spawn(move || loop {
        if let Event::Key(crossterm::event::KeyEvent { code, .. }) = read().unwrap() {
            match code {
                KeyCode::Char('w') => tx.send(Some(Signal::Rotate)).unwrap(),
                KeyCode::Char('s') => tx.send(Some(Signal::Accelerate)).unwrap(),
                KeyCode::Char('a') => tx.send(Some(Signal::Left)).unwrap(),
                KeyCode::Char('d') => tx.send(Some(Signal::Right)).unwrap(),
                KeyCode::Char('x') => tx.send(Some(Signal::Sink)).unwrap(),
                KeyCode::Char('q') => tx.send(Some(Signal::Quit)).unwrap(),
                _ => tx.send(None).unwrap(),
            };
        }
        std::thread::sleep(Duration::from_millis(1));
    });
    let mut count = 0;
    enable_raw_mode().unwrap();
    loop {
        execute!(stdout(), cursor::Hide).unwrap();
        // 接收管道内容
        match rx.try_recv() {
            Ok(Some(signal)) => match signal {
                Signal::Quit => break,
                Signal::Rotate => t.event_rotate(),
                Signal::Left => t.event_left(),
                Signal::Right => t.event_right(),
                Signal::Accelerate => t.accelerate(),
                Signal::Sink => t.event_sink(),
            },
            Ok(None) | Err(_) => {}
        }

        if count % (100) == 0 {
            if let Some(record) = t.update() {
                screen.raw_write_fix(format!("{:?}", record)); //结束，结算.
                break;
            };
        }
        clear_screen().unwrap();
        screen.display(&t);
        execute!(stdout(), cursor::MoveToColumn(0)).unwrap();

        thread::sleep(Duration::from_millis(10));
        count += 1;
    }
    execute!(stdout(), cursor::MoveToColumn(0)).unwrap();
    disable_raw_mode().unwrap();
    execute!(stdout(), cursor::Show).unwrap();
    execute!(stdout(), cursor::EnableBlinking).unwrap();
    execute!(stdout(), cursor::SetCursorStyle::DefaultUserShape).unwrap();
}
