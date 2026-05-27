#[derive(PartialEq)]
pub enum Tile {
    Player1,
    Player2,
    Empty
}

impl Tile {
    pub fn is_empty(&self) -> bool {
        match self {
            Tile::Empty => true,
            _ => false
        }
    }
}