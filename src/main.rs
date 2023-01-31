use colored::Color;
use display::colored_string;
use rand::seq::IteratorRandom;
use std::time::Duration;
use std::{collections::VecDeque, vec};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tokio::time::sleep;

pub mod async_entry;
pub mod display;

type BrickInfo<'a> = (&'a [Pixel], Color);

// classic
static SHAPE_I: BrickInfo = (&[(0, 2), (0, 1), (0, -1)], Color::Cyan);
static SHAPE_O: BrickInfo = (&[(0, 1), (1, 1), (1, 0)], Color::Yellow);
static SHAPE_T: BrickInfo = (
    &[(-1, 0), (0, 1), (1, 0)],
    Color::TrueColor {
        r: 0x64,
        g: 0x95,
        b: 0xed,
    },
);
static SHAPE_S: BrickInfo = (&[(0, 1), (-1, 0), (-1, -1)], Color::Red);
// featured
static SHAPE_Z: BrickInfo = (&[(1, 0), (0, 1), (1, -1)], Color::BrightMagenta);
static SHAPE_J: BrickInfo = (&[(0, 1), (0, -1), (-1, -1)], Color::Green);
static SHAPE_L: BrickInfo = (
    &[(0, 1), (0, -1), (1, -1)],
    Color::TrueColor {
        r: 0xef,
        g: 0x6b,
        b: 0x81,
    },
);

// feature
static SHAPE_CROSS: BrickInfo = (
    &[(-1, 0), (1, 0), (0, -1), (0, 1)],
    Color::TrueColor {
        r: 0xd6,
        g: 0x36,
        b: 0xb4,
    },
);
static SHAPE_DOT: BrickInfo = (
    &[],
    Color::TrueColor {
        r: 0x80,
        g: 0x00,
        b: 0x80,
    },
);
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum BrickType {
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
    Cross,
    Dot,
}
type Pixel = (isize, isize);

#[derive(Debug, Clone)]
pub struct Brick {
    brick_type: BrickType,
    pixels: Vec<Pixel>,
    color: Color,
}

const FULL: char = 'X';
const WALL: char = 'O';
const EMPTY: char = ' ';

// ...
impl Brick {
    fn limits(&self) -> (isize, isize, isize, isize) {
        self.pixels.iter().fold(
            (
                std::isize::MAX,
                std::isize::MIN,
                std::isize::MAX,
                std::isize::MIN,
            ),
            |(min_x, max_x, min_y, max_y), &(x, y)| {
                (min_x.min(x), max_x.max(x), min_y.min(y), max_y.max(y))
            },
        )
    }
    fn display(&self) -> String {
        let (min_x, max_x, min_y, max_y) = self.limits();
        let mut result = String::new();
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if (x, y) == (0, 0) || self.pixels.contains(&(x, y)) {
                    result.push(FULL);
                } else {
                    result.push(EMPTY);
                }
            }
            result.push('\n');
        }
        result
    }

    fn display_color(&self) -> String {
        colored_string(self.display(), self.color)
    }
    pub fn new(b: BrickType) -> Self {
        let ps: BrickInfo = match b {
            BrickType::I => SHAPE_I,
            BrickType::O => SHAPE_O,
            BrickType::T => SHAPE_T,
            BrickType::S => SHAPE_S,
            BrickType::Z => SHAPE_Z,
            BrickType::L => SHAPE_L,
            BrickType::J => SHAPE_J,
            BrickType::Dot => SHAPE_DOT,
            BrickType::Cross => SHAPE_CROSS,
        };
        Self {
            brick_type: b,
            pixels: ps.0.to_vec(),
            color: ps.1,
        }
    }
    pub fn rotate(&mut self) {
        for i in 0..self.pixels.len() {
            let (x, y) = self.pixels[i];
            self.pixels[i] = (y, -x);
        }
    }
}

#[test]
fn brick_spin_test() {
    let mut b = Brick::new(BrickType::L);
    println!("{}", b.display_color());
    b.rotate();
    println!("{}", b.display_color());
    b.rotate();
    println!("{}", b.display_color());
    b.rotate();
    println!("{}", b.display_color());
}

#[test]
fn brick_color_test() {
    for e in &[
        Brick::new(BrickType::I).display_color(),
        Brick::new(BrickType::O).display_color(),
        Brick::new(BrickType::J).display_color(),
        Brick::new(BrickType::L).display_color(),
        Brick::new(BrickType::S).display_color(),
        Brick::new(BrickType::Z).display_color(),
        Brick::new(BrickType::T).display_color(),
        Brick::new(BrickType::Dot).display_color(),
        Brick::new(BrickType::Cross).display_color(),
    ] {
        println!("{e}")
    }
}

/*
O              O       SCORE: {}
O              O
O              O
O              O
O              O       NEXT:
O              O       ||||||
O              O       |  X |
O              O       |  X |
O              O       |  X |
O              O       |  X |
O              O       ||||||
O              O
OX             O
OX             O
OXX            O
OOOOOOOOOOOOOOOO
*/
#[derive(Debug, PartialEq)]
enum GameStatus {
    Running,
    Pause,
    Accelerative,
    Exit,
}

#[derive(Debug)]
struct Record {
    score: i64,
    combo: usize, //连击数量
    eliminate_rows: usize,
}

impl Record {
    pub fn compute(&mut self, rows_num: usize) {
        if rows_num == 0 {
            self.combo = 0;
            return;
        }
        for _ in 0..rows_num {
            self.combo_once()
        }
    }

    pub fn combo_once(&mut self) {
        self.score += 100 + (self.combo * 30) as i64;
        self.eliminate_rows += 1;
        self.combo += 1;
    }
}
#[derive(PartialEq, Eq)]
pub enum InGameStatus {
    KeepDroping,
    FinishDropping,
}

#[derive(Debug)]
enum Signal {
    Quit,
    Left,
    Right,
    Accelerate,
    Sink,
    Empty,
}
#[derive(Debug)]
struct Controller {
    signal: Signal,
}

enum ControlLimit {
    CantLeft,
    CantRight,
    CantLeftAndRight,
}

#[derive(Debug)]
pub struct Tetris {
    board: Board,
    status: GameStatus,
    now_brick: Option<Brick>,
    now_brick_position: (usize, usize),
    following_bricks: VecDeque<Brick>,
    record: Record,
    controller: Controller,
}
impl Tetris {
    pub fn new(w: usize, h: usize) -> Self {
        let mut q = VecDeque::new();
        for _ in 0..3 {
            q.push_back(Self::random_brick());
        }
        let board = Board::new(w, h);
        let c = board.center;
        Self {
            board,
            status: GameStatus::Pause,
            now_brick_position: (c, 0),
            following_bricks: q,
            now_brick: None,
            record: Record {
                score: 0,
                combo: 0,
                eliminate_rows: 0,
            },
            controller: Controller {
                signal: Signal::Empty,
            },
        }
    }

    fn random_brick() -> Brick {
        let mut rng = rand::thread_rng();
        Brick::new(BrickType::iter().choose(&mut rng).unwrap())
    }

    fn line_full(line: &Line) -> bool {
        line.iter().all(|x| x.0.is_some())
    }

    // instance method
    fn add_next_brick(&mut self) {
        self.following_bricks.push_back(Self::random_brick())
    }

    fn combout(&mut self) -> usize {
        // compute how many lines were combed.
        let mut combo_count = 0;
        for i in 0..self.board.height {
            if Self::line_full(&self.board.datas[i]) {
                combo_count += 1;
                //符合消除条件。
                self.board.datas.remove(i); //这一层消除
                self.board
                    .datas
                    .insert(0, vec![Unit(None); self.board.width]); //添加新层到最前面。
            }
        }
        combo_count
    }
    fn event_rotate(&mut self) {
        if let Some(brick) = &mut self.now_brick {
            brick.rotate();
        }
    }

    fn event_left(&mut self) {
        self.now_brick_position.0 - 1;
    }

    fn event_right(&mut self) {
        self.now_brick_position.0 + 1;
    }
    fn event_quit(&mut self) {
        self.status = GameStatus::Exit;
    }
    fn event_accelerate(&mut self) {
        self.status = GameStatus::Accelerative
    }

    fn limited(&self) -> Option<ControlLimit> {
        //是否贴着左右的Unit 用于限制左右移动碰撞箱
        let absolute_positions = self.get_absolute();
        // 尝试探测
        let mut cant_l = false;
        let mut cant_r = false;
        for e in &absolute_positions {
            let &(x, y) = e;
            // 左边
            if x == 0 || self.board.datas[y as usize][x as usize - 1].0.is_some() {
                cant_l = true
            }
            // 右边
            if x == (self.board.width - 1) as isize
                || self.board.datas[y as usize][x as usize + 1].0.is_some()
            {
                cant_r = true
            }
        }
        match (cant_l, cant_r) {
            (true, true) => Some(ControlLimit::CantLeftAndRight),
            (true, false) => Some(ControlLimit::CantLeft),
            (false, true) => Some(ControlLimit::CantRight),
            (false, false) => None,
        }
    }
    fn get_absolute(&self) -> Vec<(isize, isize)> {
        let r_x = self.now_brick_position.0 as isize;
        let r_y = self.now_brick_position.1 as isize;

        let mut absolute_positions: Vec<(isize, isize)> = vec![(r_x, r_y)];
        for e in &self.now_brick.as_ref().unwrap().pixels {
            absolute_positions.push((r_x + e.0, (r_y - e.1)))
        }
        absolute_positions
    }
    fn collapse(&mut self) -> bool {
        let color = self.now_brick.as_ref().unwrap().color;
        let absolute_positions: Vec<(isize, isize)> = self.get_absolute();
        // 尝试碰撞
        let mut is_collapsed = false;
        for e in &absolute_positions {
            let &(x, y) = e;
            println!("e: {:?}", &e);
            if y >= 0 {
                if y == self.board.height as isize - 1 {
                    // 到最底层了
                    println!("{:?}", "到最底层了");
                    is_collapsed = true;
                    break;
                }

                if self.board.datas[y as usize + 1][x as usize].0.is_some() {
                    // 碰到了实体
                    println!("{:?}", "碰到了实体");
                    is_collapsed = true;
                    break;
                }
            }
        }
        // 碰撞，更改面板.
        if is_collapsed {
            for e in &absolute_positions {
                self.board.datas[e.1 as usize][e.0 as usize] = Unit(Some(color))
            }
            return true;
        }
        false
    }

    fn down(&mut self) -> InGameStatus {
        if self.collapse() {
            return InGameStatus::FinishDropping;
        }
        self.now_brick_position.1 += 1;
        InGameStatus::KeepDroping
    }

    fn new_small_run(&mut self) {
        let new_brick = self.following_bricks.pop_front().unwrap();
        self.now_brick = Some(new_brick);
        self.add_next_brick();
        self.now_brick_position = (self.board.center, 0)
    }

    pub fn start(&mut self) {
        self.status = GameStatus::Running;
        self.new_small_run();
    }
    pub fn update(&mut self) {
        // 读取输入
        if self.down() == InGameStatus::FinishDropping {
            let times = self.combout(); //计算消除的行数
            self.record.compute(times); //记录对应的分数
            self.new_small_run(); //召唤新的砖块.
        }
    }
}

#[derive(Debug, Clone)]
pub struct Unit(Option<Color>);
type Line = Vec<Unit>;
#[derive(Debug, Clone)]
pub struct Board {
    pub center: usize,
    pub width: usize,
    pub height: usize,
    pub datas: Vec<Line>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let mut datas = vec![];
        for _ in 0..height {
            datas.push(vec![Unit(None); width])
        }
        Self {
            width,
            height,
            datas,
            center: width / 2,
        }
    }
    pub fn display(&self, score: i64, bricks_color: Color, bricks_positions: Vec<(isize, isize)>) {
        let separation = WALL.to_string();
        let mut drawing_board = "".to_string();
        for i in 0..=self.height {
            drawing_board += &separation;
            for j in 0..self.width {
                if bricks_positions.contains(&(j as isize, i as isize)) {
                    drawing_board += &colored_string(FULL.to_string(), bricks_color);
                    continue;
                }
                if i == self.height {
                    drawing_board += &separation;
                } else {
                    let ee = &self.datas[i][j];
                    match ee.0 {
                        None => {
                            drawing_board += " ";
                        }
                        Some(color) => {
                            drawing_board += &colored_string(FULL.to_string(), color);
                        }
                    }
                }
            }
            drawing_board += &separation;
            if i == 5 {
                drawing_board += &format!("          score {{ {score} }}");
            }

            drawing_board += "\n";
        }

        print!("{drawing_board}")
    }
}
#[test]
fn board_test() {
    let b: Board = Board::new(10, 10);
    println!("{:?}", b);
}

#[test]
fn game_test() {
    let mut game = Tetris::new(10, 11);
    println!("{:?}", Tetris::random_brick());
    game.start();
    println!("{:#?}", game);
}

#[test]
fn _test() {
    let mut c = vec![vec![Unit(None); 3]; 5];
    c[2] = vec![];
    print!("{c:?}")
}

#[test]
fn game_run_test() {
    let mut g = Tetris::new(13, 20);
    g.start();
    dbg!(&g);
    loop {
        g.update();
        g.board.display(
            g.record.score,
            g.now_brick.as_ref().unwrap().color,
            g.get_absolute(),
        );
        sleep(Duration::from_millis(300));
    }
}

// self.board.datas  先y再x [1][2] 第index=1列，第index=2行
/*
y=1;x=2
ooo
ooX
ooo
*/
#[test]
fn run_game_test() {
    let mut g = Tetris::new(10, 20);
    g.start();
    dbg!(&g);
    loop {
        g.update();
        g.board.display(
            g.record.score,
            g.now_brick.as_ref().unwrap().color,
            g.get_absolute(),
        );
        sleep(Duration::from_millis(100));
    }
}

#[test]
fn input_test() {
    use std::io::{stdin, stdout, Write};
    use termion::input::TermRead;
    use termion::raw::IntoRawMode;

    let mut g = Tetris::new(10, 20);
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Char('w') => g.event_rotate(),
            Char('s') => g.event_accelerate(),
            Char('a') => g.event_left(),
            Char('d') => g.event_right(),
            Ctrl('c') | Char('q') => g.event_quit(),
            _ => (),
        }
        stdout.flush().unwrap();
    }
}

#[test]
fn multi_thread_test() {
    use std::io::stdin;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use termion::input::TermRead;

    let stdin = stdin();
    let g = Arc::new(Mutex::new(Tetris::new(10, 20)));
    let g_input = g.clone();

    g.lock().unwrap().start();

    let handle = thread::spawn(move || {
        for c in stdin.keys() {
            match c.unwrap() {
                Char('w') => g_input.lock().unwrap().event_rotate(),
                Char('s') => g_input.lock().unwrap().event_accelerate(),
                Char('a') => g_input.lock().unwrap().event_left(),
                Char('d') => g_input.lock().unwrap().event_right(),
                Ctrl('c') | Char('q') => g_input.lock().unwrap().event_quit(),
                _ => (),
            }
        }
    });
    let mut z = 1;
    loop {
        if g.lock().unwrap().status == GameStatus::Exit {
            break;
        }
        if z % 500 == 0 {
            g.lock().unwrap().update();
            z = 1;
        }
        g.lock().unwrap().board.display(
            g.lock().unwrap().record.score,
            g.lock().unwrap().now_brick.as_ref().unwrap().color,
            g.lock().unwrap().get_absolute(),
        );
        if g.lock().unwrap().status == GameStatus::Accelerative {
            sleep(Duration::from_millis(5));
        } else {
            sleep(Duration::from_millis(10));
        }
        z += 1;
    }
    handle.join().unwrap();
}

use async_std::task;
use std::io::{stdin, stdout, Write};
use termion::event::Key::{Char, Ctrl};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

// #[async_std::main]
fn main() {
    let mut g = Tetris::new(10, 20);

    task::spawn(async move {
        loop {
            g.update();
            g.board.display(
                g.record.score,
                g.now_brick.as_ref().unwrap().color,
                g.get_absolute(),
            );
            task::sleep(Duration::from_millis(100)).await;
        }
    });

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    for c in stdin.keys() {
        match c.unwrap() {
            Char('w') => g.event_rotate(),
            Char('s') => g.event_accelerate(),
            Char('a') => g.event_left(),
            Char('d') => g.event_right(),
            // Ctrl('c') | Char('q') => g.event_quit(),
            _ => (),
        }
        stdout.flush().unwrap();
    }
}
