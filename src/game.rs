use crate::{game_state::GameState, tile::Tile};

pub struct Game {
    pub grid: [[Tile; 3]; 3],
    pub is_player1_turn: bool
}


impl Game {
    pub fn get_game_state(&self) -> GameState {
        let rows_game_state = self.check_rows();
        if !rows_game_state.is_running() { 
            return rows_game_state;
        }

        let cols_game_state = self.check_cols();
        if !cols_game_state.is_running() {
            return cols_game_state;
        }

        let diags_game_state = self.check_diagonals();
        if !diags_game_state.is_running() {
            return diags_game_state;
        }

        if self.has_empty_tile() {
            GameState::Running
        } else {
            GameState::Draw
        }
    }

    pub fn set_tile(self, pos: (usize, usize), tile: Tile) -> Option<Self> {
        let (row, col) = pos;
        
        if row >= 3 || col >= 3 || self.grid[row][col] != Tile::Empty {
            return None;
        }

        let mut new_game =  Self {
            is_player1_turn: !self.is_player1_turn, 
            ..self 
        };
        new_game.grid[row][col] = tile;
        Some(new_game)
    }

    fn has_empty_tile(&self) -> bool {
        for row in self.grid.iter() {
            for tile in row {
                if tile.is_empty() {
                    return false;
                }
            }
        }
        true
    }

    fn check_rows(&self) -> GameState {
        if self.grid[0][0] == self.grid[0][1] && self.grid[0][0] == self.grid[0][2] && self.grid[0][0] != Tile::Empty {
            match self.grid[0][0] {
                Tile::Player1 => GameState::Player1Win,
                _ => GameState::Player2Win
            }
        } else if self.grid[1][0] == self.grid[1][1] && self.grid[1][0] == self.grid[1][2] && self.grid[1][0] != Tile::Empty {
            match self.grid[1][0] {
                Tile::Player1 => GameState::Player1Win,
                _ => GameState::Player2Win
            }
        } else if self.grid[2][0] == self.grid[2][1] && self.grid[2][0] == self.grid[2][2] && self.grid[2][0] != Tile::Empty {
            match self.grid[2][0] {
                Tile::Player1 => GameState::Player1Win,
                _ => GameState::Player2Win
            }
        } else {
            GameState::Running
        }
    }

    fn check_cols(&self) -> GameState {
        if self.grid[0][0] == self.grid[1][0] && self.grid[0][0] == self.grid[2][0] && self.grid[0][0] != Tile::Empty {
            match self.grid[0][0] {
                Tile::Player1 => GameState::Player1Win,
                _ => GameState::Player2Win
            }
        } else if self.grid[0][1] == self.grid[1][1] && self.grid[0][1] == self.grid[2][1] && self.grid[0][1] != Tile::Empty {
            match self.grid[0][1] {
                Tile::Player1 => GameState::Player1Win,
                _ => GameState::Player2Win
            }
        } else if self.grid[0][2] == self.grid[1][2] && self.grid[0][2] == self.grid[2][2] && self.grid[0][2] != Tile::Empty {
            match self.grid[0][2] {
                Tile::Player1 => GameState::Player1Win,
                _ => GameState::Player2Win
            }
        } else {
            GameState::Running
        }
    }

    fn check_diagonals(&self) -> GameState {
        if self.grid[0][0] == self.grid[1][1] && self.grid[0][0] == self.grid[2][2] && self.grid[0][0] != Tile::Empty {
            match self.grid[0][0] {
                Tile::Player1 => GameState::Player1Win,
                _ => GameState::Player2Win
            } 
        } else if self.grid[0][2] == self.grid[1][1] && self.grid[0][0] == self.grid[2][0] && self.grid[0][2] != Tile::Empty {
            match self.grid[0][2] {
                Tile::Player1 => GameState::Player1Win,
                _ => GameState::Player2Win
            }
        } else {
            GameState::Running
        }
    }
}