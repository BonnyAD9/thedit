#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
}

impl Pos {
    pub const MAX: Self = Self::new(usize::MAX, usize::MAX);

    pub const fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}
