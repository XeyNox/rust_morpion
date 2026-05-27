pub enum GameState {
    Player1Win,
    Player2Win,
    Draw,
    Running
}

impl GameState {
    pub fn is_running(&self) -> bool {
        match self {
            GameState::Running => true,
            _ => false
        }
    } 
}