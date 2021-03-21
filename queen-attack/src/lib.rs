#[derive(Debug)]
pub struct ChessPosition {
    x: i32,
    y: i32,
}

#[derive(Debug)]
pub struct Queen {
    x: i32,
    y: i32,
}

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Option<Self> {
        match (x, y) {
            (0..=7, 0..=7) => Some(ChessPosition { x, y }),
            _ => None,
        }
    }
}

impl Queen {
    pub fn new(p: ChessPosition) -> Self {
        Queen { x: p.x, y: p.y }
    }

    pub fn can_attack(&self, o: &Queen) -> bool {
        o.x == self.x || o.y == self.y || (self.x - o.x).abs() == (self.y - o.y).abs()
    }
}
