use colored::Color;

// 我的真理用旧了
use rand::seq::IteratorRandom;

use strum::IntoEnumIterator;

use std::{collections::VecDeque, vec};

use crate::display;

use display::colored_string;
pub type BrickInfo<'a> = (&'a [Pixel], Color);

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
static SHAPE_S: BrickInfo = (&[(1, 0), (0, 1), (1, -1)], Color::Red);
// featured
static SHAPE_Z: BrickInfo = (
    &[(0, 1), (-1, 0), (-1, -1)],
    Color::TrueColor {
        r: 0xec,
        g: 0xc5,
        b: 0x44,
    },
);
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
#[derive(strum_macros::EnumIter, Debug, PartialEq, Clone, Copy)]
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
pub enum GameStatus {
    Running,
    Pause,
    Accelerative,
    Exit,
}

#[derive(Debug, Clone, Copy)]
pub struct Record {
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
    GameJustOver,
    KeepDroping,
    FinishDropping,
}

pub enum ControlLimit {
    CantLeft,
    CantRight,
    CantLeftAndRight,
}

#[derive(Debug)]
pub struct Tetris {
    pub board: Board,
    pub status: GameStatus,
    pub now_brick: Option<Brick>,
    pub now_brick_position: (usize, usize),
    pub following_bricks: VecDeque<Brick>,
    pub record: Record,
}

impl std::fmt::Display for Tetris {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.board.display(
                self.record.score,
                self.now_brick.as_ref().unwrap().color,
                self.get_absolute(),
                self.following_bricks.clone()
            )
        )
    }
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
    pub fn try_rotate(&mut self) -> bool {
        if let Some(brick) = &mut self.now_brick {
            let old = brick.clone();
            brick.rotate();
            if !self.is_legal_positions() || self.is_overlapped() {
                self.now_brick = Some(old); //还原
                return false;
            }
            return true;
        }
        return false;
    }
    // is_legal_positions 是否越界
    pub fn is_legal_positions(&self) -> bool {
        let mut is_legal = true;
        for (x, y) in self.get_absolute() {
            if x < 0 || (x > self.board.width as isize - 1) {
                // x横向越界!
                is_legal = false
            }
            if y > self.board.height as isize - 1 {
                // y 出现不可能的更高值
                is_legal = false
            }
        }
        is_legal
    }
    pub fn is_overlapped(&self) -> bool {
        // 只有旋转需要重叠检验。左右移动使用limits检验，下落使用collapse检验.
        for (x, y) in self.get_absolute() {
            // 不考虑负y
            if y >= 0 {
                if self.board.datas[y as usize][x as usize].0.is_some() {
                    return true;
                }
            }
        }
        false
    }
    pub fn event_rotate(&mut self) {
        self.try_rotate();
    }

    pub fn event_left(&mut self) {
        match self.limited() {
            Some(limit) => match limit {
                ControlLimit::CantLeft => return,
                ControlLimit::CantLeftAndRight => return,
                ControlLimit::CantRight => {}
            },
            None => {}
        }

        self.now_brick_position.0 -= 1;
    }

    pub fn event_right(&mut self) {
        match self.limited() {
            Some(limit) => match limit {
                ControlLimit::CantRight => return,
                ControlLimit::CantLeftAndRight => return,
                ControlLimit::CantLeft => {}
            },
            None => {}
        }
        self.now_brick_position.0 += 1;
    }
    pub fn event_quit(&mut self) {
        self.status = GameStatus::Exit;
    }

    pub fn event_sink(&mut self) {
        // 持续掉掉落
        // 这里不需要担心内部的游戏结束触发。机制。如果结束，则游戏Status成为Exit，游戏循环内通过判断则结束游戏。
        while self.down_settle() == 2 {}
    }

    pub fn accelerate(&mut self) {
        self.down_settle();
    }
    pub fn limited(&self) -> Option<ControlLimit> {
        //是否贴着左右的Unit 用于限制左右移动碰撞箱
        let absolute_positions = self.get_absolute();
        // 尝试探测
        let mut cant_l = false;
        let mut cant_r = false;
        for e in &absolute_positions {
            let &(x, y) = e;
            if y >= 0 {
                //防止越界
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

    fn collapse(&mut self, poss: Vec<(isize, isize)>) {
        let color = self.now_brick.as_ref().unwrap().color;
        for e in &poss {
            let y = e.1;
            if y >= 0 {
                self.board.datas[e.1 as usize][e.0 as usize] = Unit(Some(color))
            }
        }
    }
    fn try_collapse(&mut self) -> Option<Vec<(isize, isize)>> {
        // 绝对位置
        let poss = self.get_absolute();
        // 尝试碰撞
        let mut can_collapse = false;
        for e in &poss {
            let &(x, y) = e;
            if y >= 0 {
                if y == self.board.height as isize - 1 {
                    // 碰到地板
                    can_collapse = true;
                    break;
                }

                if self.board.datas[y as usize + 1][x as usize].0.is_some() {
                    // 碰到了实体方块
                    can_collapse = true;
                    break;
                }
            }
        }
        if can_collapse {
            return Some(poss);
        } else {
            return None;
        }
    }

    fn try_down(&mut self) -> InGameStatus {
        if let Some(poss) = self.try_collapse() {
            self.collapse(poss.clone());
            let new_poss: Vec<(isize, isize)> = self.get_absolute();
            // 判断游戏是否结束
            for (_, y) in new_poss {
                if y < 0 {
                    // 确定结束了
                    self.status = GameStatus::Exit;
                    return InGameStatus::GameJustOver;
                }
            }
            // 这里是完成💥（碰撞）同时还没有游戏结束。
            return InGameStatus::FinishDropping;
        }
        return InGameStatus::KeepDroping;
    }

    // 结算
    // 0 游戏结束 1 完成掉落，刚刚落地。 2 继续掉落中
    fn down_settle(&mut self) -> usize {
        match self.try_down() {
            InGameStatus::FinishDropping => {
                let times = self.combout(); //计算消除的行数
                self.record.compute(times); //记录对应的分数
                self.new_small_run(); //召唤新的砖块.
                1
            }
            InGameStatus::KeepDroping => {
                self.now_brick_position.1 += 1;
                2
            }
            InGameStatus::GameJustOver => {
                return 0;
            }
        }
    }

    fn new_small_run(&mut self) {
        let new_brick = self.following_bricks.pop_front().unwrap();
        self.now_brick = Some(new_brick);
        self.add_next_brick();
        //开始第二个
        self.now_brick_position = (self.board.center, 0);
        // 计算是否重叠，否则直接结束游戏.
        if self.is_overlapped() {
            self.status = GameStatus::Exit;
        }
    }

    pub fn start(&mut self) {
        self.status = GameStatus::Running;
        self.new_small_run();
    }
    pub fn update(&mut self) -> Option<Record> {
        match self.down_settle() {
            0 => {
                return Some(self.record);
            }
            1 | 2 => None,
            _ => unreachable!(),
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

    // poss: positions
    pub fn display(
        &self,
        score: i64,
        c: Color,
        poss: Vec<(isize, isize)>,
        following_bricks: VecDeque<Brick>,
    ) -> String {
        let next_infos: Vec<String> = following_bricks
            .clone()
            .iter()
            .map(|x| colored_string(format!("{:?}", x.brick_type), x.color))
            .collect();

        let separation = WALL.to_string();
        let mut drawing_board = "".to_string();
        for i in 0..=self.height {
            drawing_board += &separation;
            for j in 0..self.width {
                if poss.contains(&(j as isize, i as isize)) {
                    drawing_board += &colored_string(FULL.to_string(), c);
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
                drawing_board += &format!("          score {{ {} }}", score);
            }
            if i == 7 {
                drawing_board += &format!("          nexts");
            }
            if i == 9 {
                drawing_board += &format!("          {}", next_infos[0]);
            }
            if i == 10 {
                drawing_board += &format!("          {}", next_infos[1]);
            }
            if i == 11 {
                drawing_board += &format!("          {}", next_infos[2]);
            }

            drawing_board += "\n";
        }

        drawing_board
    }
}
