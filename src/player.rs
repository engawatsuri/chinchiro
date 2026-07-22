use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::sync::{LazyLock, Mutex};
use rand::Rng;
use crate::*;
use crate::draw::*;

#[derive(Clone)]
pub struct Card {
    pub name: String,
    pub explain: String,
    pub process: fn() -> String,
}

#[derive(Default, Clone)]
pub struct Item {
    pub name: String,
}

#[derive(Default, Clone)]
pub struct Player {
    pub name: String,
    pub pubx: i32,
    pub x: i32,
    pub y: i32,
    pub dir: Dir,
    pub hand_money: i64,
    pub card: Vec<usize>,
    pub item: Vec<Item>,
    pub color: String,
}

impl Player {
    pub fn go(&mut self, dir: Dir) -> Result<Option<Station>, JsValue> {
        let mut distance = DISTANCE.lock().unwrap();
        let mut history = HISTORY.lock().unwrap();

        let station = MAP.get(&(self.x, self.y)).ok_or(format!("no found station at ({}, {})", self.x, self.y)).unwrap();
        if dir == Dir::North && station.north != 0 {
            self.y -= station.north as i32;
            if history.len() != 0 && history[history.len() - 1] == Dir::South {
                *distance += 1;
                history.pop();
            } else {
                *distance -= 1;
                history.push(dir.clone());
            }
        } else if dir == Dir::East && station.east != 0 {
            self.x += station.east as i32;
            if history.len() != 0 && history[history.len() - 1] == Dir::West {
                *distance += 1;
                history.pop();
            } else {
                *distance -= 1;
                history.push(dir.clone());
            }
        } else if dir == Dir::South && station.south != 0 {
            self.y += station.south as i32;
            if history.len() != 0 && history[history.len() - 1] == Dir::North {
                *distance += 1;
                history.pop();
            } else {
                *distance -= 1;
                history.push(dir.clone());
            }
        } else if dir == Dir::West && station.west != 0 {
            self.x -= station.west as i32;
            if history.len() != 0 && history[history.len() - 1] == Dir::East {
                *distance += 1;
                history.pop();
            } else {
                *distance -= 1;
                history.push(dir.clone());
            }
        }
        self.dir = dir.clone();

        if *distance == 0 {
            let station = MAP.get(&(self.x, self.y)).ok_or(format!("no found station at ({}, {})", self.x, self.y)).unwrap();
            Ok(Some(station.clone()))
        } else {
            Ok(None)
        }
    }

    pub fn roll(&self) -> [u32; 3] {
        let mut distance = DISTANCE.lock().unwrap();
        let mut history = HISTORY.lock().unwrap();

        history.clear();
        loop {
            let dices: [u32; 3] = [
                rand::thread_rng().gen_range(1..7),
                rand::thread_rng().gen_range(1..7),
                rand::thread_rng().gen_range(1..7),
            ];
            if dices[0] == 1 && dices[1] == 1 && dices[2] == 1 {
                *distance = 30;
            } else if dices[0] == dices[1] && dices[1] == dices[2] {
                *distance = dices[0] * 3;
            } else if dices[0] == 4 && dices[1] == 5 && dices[2] == 6 {
                *distance = 10;
            } else if dices[0] == dices[1] {
                *distance = dices[2];
            } else if dices[1] == dices[2] {
                *distance = dices[0];
            } else if dices[2] == dices[0] {
                *distance = dices[1];
            } else {
                continue;
            };
            return dices;
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum Dir {
    North,
    East,
    South,
    #[default] West,
}

impl Dir {
    pub fn from_key(key: &str) -> Option<Dir> {
        match key {
            "h" => Some(Dir::West),
            "j" => Some(Dir::South),
            "k" => Some(Dir::North),
            "l" => Some(Dir::East),
            _ => None,
        }
    }
}

pub static TURN: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));
pub static PLAYERS: LazyLock<Mutex<Vec<Player>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static DISTANCE: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
pub static HISTORY: LazyLock<Mutex<Vec<Dir>>> = LazyLock::new(|| Mutex::new(Vec::new()));

impl Card {
    pub fn new(name: &str, explain: &str, process: fn() -> String) -> Self {
        Self {
            name: name.to_string(),
            explain: explain.to_string(),
            process: process,
        }
    }

    fn billion() -> String {
        let mut players = PLAYERS.lock().unwrap();
        let turn = TURN.lock().unwrap();
        players[*turn].hand_money += 100000;
        "10億円を手に入れた".to_string()
    }

    fn debt_cancellation_order() -> String {
        let mut players = PLAYERS.lock().unwrap();
        for player in players.iter_mut() {
            if player.hand_money < 0 {
                player.hand_money = 0;
            }
        }
        "皆さんの借金は帳消しになりました".to_string()
    }
}

pub static CARD_LIST: LazyLock<[Card; 2]> = LazyLock::new(|| {[
    Card::new("10億円", "使用すると10億円手に入れることができます", Card::billion),
    Card::new("徳政令", "全員の借金を無くします", Card::debt_cancellation_order),
]});

pub fn add_player(name: &str) {
    let mut players = PLAYERS.lock().unwrap();
    let mut player = Player::default();
    player.name = name.to_string();
    player.color = match players.len() {
        0 => "blue".to_string(),
        1 => "red".to_string(),
        2 => "yellow".to_string(),
        3 => "green".to_string(),
        _ => panic!("too many players"),
    };
    players.push(player);
}

pub fn next_turn() -> usize {
    let mut turn = TURN.lock().unwrap();
    let mut month = MONTH.lock().unwrap();
    let mut year = YEAR.lock().unwrap();
    let players = PLAYERS.lock().unwrap();
    *turn = (*turn + 1) % players.len();
    if *turn == 0 {
        *month = (*month + 1) % 12 + 1;
        if *month == 1 {
            *year += 1;
        }
    }
    *turn
}
