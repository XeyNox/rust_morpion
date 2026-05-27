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

    pub fn to_string(&self) -> String {
        match self {
            GameState::Player1Win => "Victoire joueur 1",
            GameState::Player2Win => "Victoire joueur 2",
            GameState::Draw => "Partie nulle",
            GameState::Running => "En cours",
        }.to_string()
    }
}