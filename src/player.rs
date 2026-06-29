use std::sync::{LazyLock, Mutex};

pub struct Player {
    name: String,
    x: i32,
    y: i32,
    hand_monay: i64,
}

impl Player {
    pub fn get_name(&self) -> &str {
        &(self.name)
    }
    pub fn get_x(&self) -> f64 {
        self.x as f64
    }
    pub fn get_y(&self) -> f64 {
        self.y as f64
    }
}

static PLAYERS: LazyLock<Mutex<Vec<Player>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn add_player(name: &str) {
    let mut players = PLAYERS.lock().unwrap();
    players.push(Player {
        name: name.to_string();
        x: 0,
        y: 0,
        hand_monay: 0,
    });
}

pub fn get_player(id: f64) -> Player {
    let mut players = PLAYERS.lock().unwrap();
    players[id as usize]
}
