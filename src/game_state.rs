pub struct GameState {
    pub player: Player
}

impl GameState {
    pub fn new(rot: f64) -> GameState {
        GameState {
            player: Player::new(rot)
        }
    }
}

pub struct Player {
    pub rotation: f64
}

impl Player {
    fn new (rotation: f64) -> Player {
        Player {
            rotation
        }
    }
}
