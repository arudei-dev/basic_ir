#[derive(Clone)]
pub struct Position {
    pub idx: usize,
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub fn init(idx: usize, line: usize, col: usize) -> Self {
        Self { idx, line, col }
    }

    pub fn advance(&mut self, curr_char: char) {
        self.idx += 1;
        self.col += 1;

        if curr_char == '\n' {
            self.line += 1;
            self.col = 0;
        }
    }
}
