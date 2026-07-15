use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::sync::{LazyLock, Mutex};
use rand::Rng;
use crate::*;

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
    pub dir: String,
    pub hand_money: i64,
    pub card: Vec<usize>,
    pub item: Vec<Item>,
}

pub static TURN: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));
pub static PLAYERS: LazyLock<Mutex<Vec<Player>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static DISTANCE: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
pub static HISTORY: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

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

#[wasm_bindgen]
pub fn add_player(name: &str) {
    let mut players = PLAYERS.lock().unwrap();
    let mut player = Player::default();
    player.name = name.to_string();
    player.dir = "w".to_string();
    players.push(player);
}

#[wasm_bindgen]
pub fn get_cards() -> Vec<String> {
    let players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let mut cards: Vec<String> = Vec::new();
    for card_id in players[*turn].card.iter() {
        cards.push(CARD_LIST[*card_id].name.clone());
    }
    cards
}

#[wasm_bindgen]
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

#[wasm_bindgen]
pub fn roll() -> u32 {
    let turn = TURN.lock().unwrap();
    let players = PLAYERS.lock().unwrap();
    let mut distance = DISTANCE.lock().unwrap();
    let mut history = HISTORY.lock().unwrap();
    
    loop {
        let dice0 = rand::thread_rng().gen_range(1..7);
        let dice1 = rand::thread_rng().gen_range(1..7);
        let dice2 = rand::thread_rng().gen_range(1..7);
        if dice0 == 1 && dice1 == 1 && dice2 == 1 {
            *distance = 30;
        } else if dice0 == dice1 && dice1 == dice2 {
            *distance = dice0 * 3;
        } else if dice0 == 4 && dice1 == 5 && dice2 == 6 {
            *distance = 10;
        } else if dice0 == dice1 {
            *distance = dice2;
        } else if dice1 == dice2 {
            *distance = dice0;
        } else if dice2 == dice0 {
            *distance = dice1;
        } else {
            continue;
        };
        break;
    }

    history.clear();

    *distance
}

pub fn get_color(id: usize) -> Result<String, JsValue> {
    match id {
        0 => Ok("blue".to_string()),
        1 => Ok("red".to_string()),
        2 => Ok("yellow".to_string()),
        3 => Ok("green".to_string()),
        _ => Err(JsValue::from_str(&format!("color: {}: 異常な値です", id))),
    }
}
