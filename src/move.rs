#[derive(Clone, Default)]
pub struct Face(usize);

#[derive(Clone, Default)]
pub struct Row(usize);

#[derive(Clone, Default)]
pub struct Column(usize);

impl Face {
    pub fn get(&self) -> usize {
        self.0
    }
}

impl Row {
    pub fn get(&self) -> usize {
        self.0
    }
}

impl Column {
    pub fn get(&self) -> usize {
        self.0
    }
}

#[derive(Clone)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    Pos((Face, Row, Column, Box<Move>)),
}

// TODO: switch to TryFrom and check max indices/faces
impl From<usize> for Column {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl From<usize> for Row {
    fn from(val: usize) -> Self {
        Self(val)
    }
}

impl From<usize> for Face {
    fn from(val: usize) -> Self {
        Self(val)
    }
}
