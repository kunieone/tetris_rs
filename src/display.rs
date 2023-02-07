use colored::{Color, Colorize};
use std::io::{stdout, Write};
use termion::raw::IntoRawMode;

use crate::{bricks::Brick, game::Tetris};

struct PaintBoard(pub Vec<Vec<String>>, pub String);

impl std::fmt::Display for PaintBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row in &self.0 {
            for c in row {
                result.push_str(c);
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

impl PaintBoard {
    fn new(width: usize, height: usize, empty: char) -> Self {
        return Self(
            vec![vec![empty.to_string(); width + 10]; height],
            empty.to_string(),
        );
    }
    fn paint_pixel(&mut self, x: usize, y: usize, pixel: char, color: Option<Color>) {
        let width = self.0[0].len();
        let height = self.0.len();
        if y >= height {
            for _ in height..=y {
                self.0.push(vec![self.1.clone(); width]);
            }
        }
        if x >= width {
            for i in 0..self.0.len() {
                for _ in width..=x {
                    self.0[i].push(self.1.clone());
                }
            }
        }
        match color {
            Some(c) => self.0[y][x] = format!("{}", pixel.to_string().color(c)),
            None => self.0[y][x] = format!("{}", pixel.to_string()),
        }
    }
    fn paint_string(&mut self, x: usize, y: usize, s: &str, color: Option<Color>) {
        let mut x_var = x;
        let mut y_var = y;
        for c in s.chars() {
            if c == '\n' {
                y_var += 1;
                x_var = x;
                continue;
            }
            self.paint_pixel(x_var, y_var, c, color);
            x_var += 1;
        }
    }
}

#[test]
fn paint_board_test() {
    let mut b = PaintBoard::new(5, 5, '-');
    b.paint_pixel(40, 2, 'H', Some(Color::Magenta));
    b.paint_string(4, 4, "Hello\nworld", Some(Color::Blue));
    println!("{}", b);
}

#[derive(Debug, Clone, Copy)]
pub struct TerminalPainter {}

impl TerminalPainter {
    fn bricks_display(brick: &Brick, texture: [char; 4]) -> String {
        let (min_x, max_x, min_y, max_y) = brick.limits();
        let mut result = String::new();
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if (x, y) == (0, 0) || brick.pixels.contains(&(x, y)) {
                    result.push(texture[0]);
                } else {
                    result.push(texture[2]);
                }
            }
            result.push('\n');
        }
        // Self::colored_string(, brick.color)
        result
    }

    pub fn colored_string(text: String, color: Color) -> String {
        // termion::color::AnsiValue
        format!("{}", text.color(color))
    }

    pub fn raw_write_fix(input_string: String) {
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

    pub fn draw(game: &Tetris) -> String {
        let poss = game.get_absolute();
        let w = game.board.width;
        let h = game.board.height;
        let full = game.cfg.texture[0];
        let wall = game.cfg.texture[1];
        let empty = game.cfg.texture[2];
        let shadow = game.cfg.texture[3];
        let mut painter: PaintBoard = PaintBoard::new(w, h, ' ');

        // 绘制墙
        // 横墙
        for i in 0..w + 2 {
            painter.paint_pixel(i, 0, '+', None);
            painter.paint_pixel(i, h + 1, wall, None);
        }
        //竖墙
        for j in 0..h + 2 {
            painter.paint_pixel(0, j, wall, None);
            painter.paint_pixel(w + 1, j, wall, None);
        }

        // 绘制元素
        for y in 0..h {
            for x in 0..w {
                match game.board.datas[y][x].0 {
                    Some(color) => painter.paint_pixel(x + 1, y + 1, full, Some(color)),
                    None => painter.paint_pixel(x + 1, y + 1, empty, None),
                };
            }
        }
        // 绘制影子

        for &(x, y) in &game.get_shadow() {
            if y >= 0 {
                painter.paint_pixel(x as usize + 1, y as usize + 1, shadow, None);
            }
        }
        // 绘制本体
        for &(x, y) in &poss {
            if y >= 0 {
                painter.paint_pixel(
                    x as usize + 1,
                    y as usize + 1,
                    full,
                    Some(game.now_brick.clone().unwrap().color),
                );
            }
        }
        // 绘制介绍
        painter.paint_string(
            0,
            h as usize + 3,
            "press arrow key to move, press space to drop.",
            Some(Color::BrightRed),
        );
        // 绘制分数
        painter.paint_string(
            w + 5,
            1,
            &format!(
                "score: {}\nhighest combo: {}\ncombout rows: {}",
                game.record.score.to_string().color(Color::Red),
                game.record.high_combo.to_string().color(Color::BrightBlue),
                game.record.eliminate_rows.to_string().color(Color::Yellow)
            ),
            None,
        );
        // 绘制next_bricks
        let mut start_y = 5;
        painter.paint_string(w + 5, start_y, "nexts:", None);
        start_y += 2;
        for e in game.following_bricks.iter() {
            painter.paint_string(
                w + 7,
                start_y,
                &(Self::bricks_display(e, game.cfg.texture)),
                Some(e.color),
            );
            start_y += e.get_size().1 + 1;
        }

        painter.to_string()
    }

    pub fn draw_record(game: &Tetris) {
        Self::raw_write_fix(format!("{}", game.record));
    }

    pub fn draw_game(t: &Tetris) {
        Self::raw_write_fix(Self::draw(t));
    }
}

#[test]
fn game_print_test() {
    let mut t = Tetris::new(crate::env::EnvConfig {
        accelerate: false,
        width: 10,
        height: 15,
        feature_brick: true,
        texture: ['#', '0', ' ', '+'],
    });

    t.start();
    t.update();
    t.update();
    t.update();
    t.update();
    t.update();
    t.update();
    TerminalPainter::draw_game(&t);
}
