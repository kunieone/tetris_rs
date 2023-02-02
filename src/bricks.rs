use colored::Color;

type Pixel = (isize, isize);

pub type BrickInfo<'a> = (&'a [Pixel], Color);

// classic
pub static SHAPE_I: BrickInfo = (&[(0, 2), (0, 1), (0, -1)], Color::Cyan);
pub static SHAPE_O: BrickInfo = (&[(0, 1), (1, 1), (1, 0)], Color::Yellow);
pub static SHAPE_T: BrickInfo = (
    &[(-1, 0), (0, 1), (1, 0)],
    Color::TrueColor {
        r: 0x64,
        g: 0x95,
        b: 0xed,
    },
);
pub static SHAPE_S: BrickInfo = (&[(1, 0), (0, 1), (1, -1)], Color::Red);
// featured
pub static SHAPE_Z: BrickInfo = (
    &[(0, 1), (-1, 0), (-1, -1)],
    Color::TrueColor {
        r: 0xec,
        g: 0xc5,
        b: 0x44,
    },
);
pub static SHAPE_J: BrickInfo = (&[(0, 1), (0, -1), (-1, -1)], Color::Green);
pub static SHAPE_L: BrickInfo = (
    &[(0, 1), (0, -1), (1, -1)],
    Color::TrueColor {
        r: 0xef,
        g: 0x6b,
        b: 0x81,
    },
);

// feature
// pub static SHAPE_CROSS: BrickInfo = (
//     &[(-1, 0), (1, 0), (0, -1), (0, 1)],
//     Color::TrueColor {
//         r: 0xea,
//         g: 0x53,
//         b: 0xea,
//     },
// );
pub static SHAPE_DOT: BrickInfo = (
    &[],
    Color::TrueColor {
        r: 0x80,
        g: 0x00,
        b: 0x80,
    },
);

pub static SHAPE_ANGLE: BrickInfo = (
    &[(0, 1), (1, 0)],
    Color::TrueColor {
        r: 0x00,
        g: 0x60,
        b: 0x40,
    },
);
/*
  W
 WW
WW
*/
pub static SHAPE_W: BrickInfo = (
    &[(0, -1), (1, 0), (-1, -1), (1, 1)],
    Color::TrueColor {
        r: 0x2b,
        g: 0xdd,
        b: 0x14,
    },
);
pub static SHAPE_BEAN: BrickInfo = (
    &[(0, 1)],
    Color::TrueColor {
        r: 0xe8,
        g: 0x7d,
        b: 0x0a,
    },
);

pub static SHAPE_DESK: BrickInfo = (
    &[(-1, 1), (1, 1), (1, 0), (-1, 0)],
    Color::TrueColor {
        r: 0x20,
        g: 0x60,
        b: 0xee,
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
    // custom
    Dot,
    Desk,
    Angle,
    // Cross,
    W,
    Bean,
}

#[derive(Debug, Clone)]
pub struct Brick {
    pub brick_type: BrickType,
    pub pixels: Vec<Pixel>,
    pub color: Color,
}

pub static FULL: char = '#';
pub static WALL: char = 'O';
pub static EMPTY: char = ' ';
pub static SHADOW: char = '+';

impl Brick {
    pub fn limits(&self) -> (isize, isize, isize, isize) {
        if self.pixels.len() == 0 {
            return (0, 0, 0, 0);
        }
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
    pub fn get_size(&self) -> (usize, usize) {
        let (min_x, max_x, min_y, max_y) = self.limits();
        ((max_x - min_x) as usize + 1, (max_y - min_y) as usize + 1)
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
            // BrickType::Cross => SHAPE_CROSS,
            BrickType::Angle => SHAPE_ANGLE,
            BrickType::Desk => SHAPE_DESK,
            BrickType::W => SHAPE_W,
            BrickType::Bean => SHAPE_BEAN,
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
    pub fn pixels_info(&self, offset_x: isize, offset_y: isize) -> Vec<(isize, isize)> {
        let mut absolute_positions: Vec<(isize, isize)> = vec![(offset_x, offset_y)];
        for e in &self.pixels {
            absolute_positions.push((offset_x + e.0, (offset_y - e.1)))
        }
        absolute_positions
    }
}
