#[derive(PartialEq, Copy, Clone)]
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

    pub fn to_string(&self) -> String {
        match self {
            Tile::Player1 => "X",
            Tile::Player2 => "O",
            Tile::Empty => "",
        }.to_string()
    }
}