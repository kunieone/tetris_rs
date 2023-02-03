# tetris-rs

 <h2 style="text-align: center;">Command line based Tetris in rust</h2>

![tetris-rs](https://pic1.zhimg.com/80/v2-16932d31f423f75ee0c69083c8e101ea_1440w.png)

## Get Started

```sh
cargo install tetris-rs
```

excute the binary command `tetris`:

```sh
tetris
```

## Config

Setting environment variables to customize

```toml
# defaults:
FEATURE_BRICK=true #bool

ACCELERATE_MODE=true #bool

WIDTH=13 #number

HEIGHT=20  #number

TEXTURE_FULL='#'  #char
TEXTURE_WALL='O' #char
TEXTURE_EMPTY=' ' #char
TEXTURE_SHADOW='+' #char

```

example:

```sh
TEXTURE_FULL='%' FEATURE_BRICK=false tetris 
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
    // FEATURE_BRICK to enable feature bricks
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

- Eliminating one row, you get `200` scores.

- `60` more points per combo.

- You get `1` point for every time you accelerate.

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

## Accelerate Mode

accelerate the update frequency based on your score

```rust
pub fn update_by(&mut self, counter: i32) {
    match self.cfg.accelerate {
        true => {
            let time = match self.record.score {
                0..=5999 => 100,
                6000..=9999 => 70,
                10000..=24999 => 60,
                25000..=39999 => 50,
                40000..=59999 => 45,
                _ => 40,
            };
            if counter % (time) == 0 {
                self.update()
            }
        }
        false => {
            if counter % (100) == 0 {
                self.update()
            }
        }
    }
}
```

