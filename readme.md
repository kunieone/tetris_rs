# tetris-rs

 <h2 style="text-align: center;">Command line based Tetris in rust</h2>

![tetris-rs](https://pic1.zhimg.com/80/v2-16932d31f423f75ee0c69083c8e101ea_1440w.png)

## Get Started

```sh
cargo install tetris-rs
```

excute:

```sh
tetris
```

## Config

you can add to environment variable

```toml
FEATURE_BRICK = true #default: true

ACCELERATE_MODE = true #default: true

WIDTH=13 # default: 13

HEIGHT=20 # default: 20


TEXTURE_FULL= '#'
TEXTURE_WALL= 'O'
TEXTURE_EMPTY= ' '
TEXTURE_SHADOW= '+'

```

## Bricks

```rust
pub enum BrickType {
    // 7 classic bricks
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
    // feature

    // #
    Dot,
    // # #
    // ###
    Desk,
    // #
    // ##
    Angle,
    // #
    // ##
    //  ##
    W,
    // ##
    Bean,
}
```

## Score Computation

- Eliminating one row, you get 200 scores.

- 60 more points per combo.

- You get one point for every time you accelerate.

```rust
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

    fn combo_once(&mut self) {
        self.score += 200 + (self.combo * 60) as i64;
        self.eliminate_rows += 1;
        self.combo += 1;
        // 计算历史最高连击
        if self.combo > self.high_combo {
            self.high_combo = self.combo
        }
    }
}
```
