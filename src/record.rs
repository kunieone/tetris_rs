#[derive(Clone, Copy, Debug)]
pub struct Record {
    pub score: i64,
    pub combo: usize, //连击数量
    pub high_combo: usize,
    pub eliminate_rows: usize,
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "score: {}, highest combo: {}, rows eliminated: {}",
            self.score, self.high_combo, self.eliminate_rows
        )
    }
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
        // 计算历史最高连击
        if self.combo > self.high_combo {
            self.high_combo = self.combo
        }
    }
}
