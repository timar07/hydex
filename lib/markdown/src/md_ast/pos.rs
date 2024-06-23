/// Represents position in the source file/string
#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub line: usize,
    pub col: usize,
    pub index: usize,
}

impl Pos {
    pub fn default() -> Pos {
        Pos {
            line: 0,
            col: 0,
            index: 0
        }
    }
}
