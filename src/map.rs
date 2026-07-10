use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::collections::HashMap;
use std::sync::LazyLock;
use rand::Rng;
use crate::player::*;
use crate::*;

#[derive(Debug, Clone)]
struct Station {
    pub x: i32,
    pub y: i32,
    pub n: u8,
    pub e: u8,
    pub s: u8,
    pub w: u8,
    pub kind: StationKind,
}

#[derive(Debug, Clone)]
enum StationKind {
    Property {
        name: String,
    },
    Card,
    Red,
    Blue,
    Other,
}

static MAP: LazyLock<HashMap<(i32, i32), Station>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    let data = include_str!("../map.csv");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.records() {
        let record = result.unwrap();
        let x = record[0].parse().unwrap_or(0);
        let y = record[1].parse().unwrap_or(0);
        let station = Station {
            x,
            y,
            n: record[2].parse().unwrap_or(0),
            e: record[3].parse().unwrap_or(0),
            s: record[4].parse().unwrap_or(0),
            w: record[5].parse().unwrap_or(0),
            kind: match &record[6] {
                "property" => StationKind::Property {
                    name: record[7].to_string(),
                },
                "card" => StationKind::Card,
                "red" => StationKind::Red,
                "blue" => StationKind::Blue,
                _ => StationKind::Other,
            },
        };
        map.insert((x, y), station);
    }
    map
});

#[wasm_bindgen]
pub fn draw_map() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no found window")?;
    let document = window.document().ok_or("no found document")?;
    let win = document.get_element_by_id("win").ok_or("no found canvas")?.dyn_into::<HtmlCanvasElement>().map_err(|_| "HtmlCanvasElementへの変換に失敗しました")?;
    let width = win.width() as f64; // aspect ratio is 3:4
    let ctx = win.get_context("2d")?.ok_or("2dコンテキストの取得に失敗しました")?.dyn_into::<CanvasRenderingContext2d>()?;
    let station_size = width / 30.0;
    let mut left = station_size * 0.5;
    let mut top = station_size * 0.25;
    let players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let year = YEAR.lock().unwrap();
    let month = MONTH.lock().unwrap();
    let center_x = players[*turn].x;
    let center_y = players[*turn].y;

    // draw station and line
    for y in -3i32..=3 {
        left = station_size * 0.5;
        for x in -4i32..=4 {
            if let Some(station) = MAP.get(&(center_x + x, center_y + y)) {
                match &station.kind {
                    StationKind::Property {..} => {
                        ctx.set_fill_style_str("purple");
                        ctx.fill_rect(left, top, station_size, station_size);
                    }
                    StationKind::Card => {
                        ctx.set_fill_style_str("yellow");
                        ctx.fill_rect(left, top, station_size, station_size);
                    }
                    StationKind::Red => {
                        ctx.set_fill_style_str("red");
                        ctx.fill_rect(left, top, station_size, station_size);
                    }
                    StationKind::Blue => {
                        ctx.set_fill_style_str("blue");
                        ctx.fill_rect(left, top, station_size, station_size);
                    }
                    StationKind::Other => {
                        return Err(JsValue::from_str(&format!("map.csv: {}: {}: kind: no match", station.x, station.y)));
                    }
                }
                let n = station.n as f64;
                let e = station.e as f64;
                let s = station.s as f64;
                let w = station.w as f64;
                ctx.set_fill_style_str("black");
                if station.w != 0 && x == -5 + station.w as i32 {
                    ctx.fill_rect(0.0, top + station_size * 0.25, station_size * -3.0 + station_size * 3.5 * w, station_size * 0.5);
                }
                if station.n != 0 && y == -4 + station.n as i32 {
                    ctx.fill_rect(left + station_size * 0.25, 0.0, station_size * 0.25, station_size * -3.0 + station_size * 3.5 * n);
                }
                if station.e != 0 && x == 4 {
                    ctx.fill_rect(left + station_size, top + station_size * 0.25, station_size * 0.5, station_size * 0.5);
                } else if station.e != 0 {
                    ctx.fill_rect(left + station_size, top + station_size * 0.25, station_size * 0.5, station_size * -1.0 + station_size * 3.5 * e);
                }
                if station.s != 0 && y == 3 {
                    ctx.fill_rect(left + station_size * 0.25, top + station_size, station_size * 0.25, station_size * 0.5);
                } else if station.s != 0 {
                    ctx.fill_rect(left + station_size * 0.25, top + station_size, station_size * -1.0 + station_size * 3.5 * s, station_size * 0.5);
                }
            }
            left += station_size * 3.5;
        }
        top += station_size * 3.5;
    }

    // draw train
    for i in 0..players.len() {
        let index = (*turn + i) % players.len();
        let player = players[index].clone();
        ctx.set_fill_style_str(&get_color(index)?);
        if center_x - 4 <= player.x && player.x <= center_x + 4 && center_y - 3 <= player.y && player.y <= center_y + 3 {
            match player.dir.as_str() {
                "n" => {
                    if player.x == center_y + 4 {
                        ctx.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 10.75 + station_size * 3.5 * (player.y - center_y) as f64 + station_size * 0.125, station_size * 0.75, station_size * 1.125);
                    } else {
                        ctx.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 10.75 + station_size * 3.5 * (player.y - center_y) as f64 + station_size * 0.125, station_size * 0.75, station_size * 1.5);
                    }
                }
                "e" => {
                    if player.x == center_x - 4 {
                        ctx.fill_rect(0.0, station_size * 0.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 1.5, station_size * 0.75);
                    } else {
                        ctx.fill_rect(station_size * -1.0 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 0.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 1.5, station_size * 0.75);
                    }
                }
                "s" => {
                    if player.x == center_x - 4 {
                        ctx.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, 0.0, station_size * 0.75, station_size * 1.5);
                    } else {
                        ctx.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * -1.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 0.75, station_size * 1.5);
                    }
                }
                "w" => {
                    if player.y == center_y + 3 {
                        ctx.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 0.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 1.5, station_size * 0.75);
                    } else {
                        ctx.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 0.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 1.375, station_size * 0.75);
                    }
                }
                &_ => {}
            }
        }
    }

    // draw info bar
    ctx.set_fill_style_str(&get_color(*turn)?);
    ctx.fill_rect(width * 3.0/56.0, width * 3.0/112.0, width * 25.0/28.0, width * 9.0/56.0);
    ctx.fill_rect(width * 3.0/14.0, width * 1.0/28.0, width * 9.0/16.0, width * 1.0/7.0);
    ctx.set_fill_style_str("black");
    ctx.set_text_baseline("top");
    ctx.set_font(&format!("{}px 'Meiryo UI'", width * 5.0/56.0));
    ctx.fill_text(&format!("{}社長", players[*turn].name), width * 25.0/112.0, width * 1.0/28.0);
    ctx.fill_text(&format!("{}個", players[*turn].item.len()), width * 0.28125, width * 1.0/14.0);
    ctx.set_font(&format!("{}px 'Meiryo UI'", width * 3.0/56.0));
    ctx.fill_text(&format!("{}円", players[*turn].hand_money), width * 25.0/112.0, width * 0.125);
    ctx.fill_text(&format!("{}年目 {}月", year, month), width * 0.28125, width * 0.125);

    Ok(())
}

#[wasm_bindgen]
pub fn move_train(dir: &str) -> String {
    let mut players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let mut distance = DISTANCE.lock().unwrap();
    let mut history = HISTORY.lock().unwrap();

    let Some(station) = MAP.get(&(players[*turn].x, players[*turn].y)) else {
        return "".to_string();
    };
    if dir == "n" && station.n != 0 && history[history.len() - 1] == "s" {
        history.pop();
        players[*turn].y += 1;
        *distance += 1;
    } else if dir == "n" && station.n != 0 {
        players[*turn].y -= 1;
        *distance -= 1;
        history.push(dir.to_string());
    } else if dir == "e" && station.e != 0 && history[history.len() - 1] == "w" {
        history.pop();
        players[*turn].x -= 1;
        *distance += 1;
    } else if dir == "e" && station.e != 0 {
        players[*turn].x += 1;
        *distance -= 1;
        history.push(dir.to_string());
    } else if dir == "s" && station.s != 0 && history[history.len() - 1] == "n" {
        history.pop();
        players[*turn].y -= 1;
        *distance += 1;
    } else if dir == "s" && station.s != 0 {
        players[*turn].y += 1;
        *distance -= 1;
        history.push(dir.to_string());
    } else if dir == "w" && station.w != 0 && history[history.len() - 1] == "e" {
        history.pop();
        players[*turn].x += 1;
        *distance += 1;
    } else if dir == "w" && station.w != 0 {
        players[*turn].x -= 1;
        *distance -= 1;
        history.push(dir.to_string());
    }
    if *distance == 0 {
        let Some(station) = MAP.get(&(players[*turn].x, players[*turn].y)) else {
            return "".to_string();
        };
        match station.kind {
            StationKind::Property {..} => {
                return "property".to_string();
            }
            StationKind::Blue => {
                return "blue".to_string();
            }
            StationKind::Card => {
                return "card".to_string();
            }
            StationKind::Red => {
                return "red".to_string();
            }
            StationKind::Other => {
                return "".to_string();
            }
        }
    }
    players[*turn].dir = dir.to_string();
    distance.to_string()
}

#[wasm_bindgen]
pub fn stop_property_station() -> String {
    let players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let Some(station) = MAP.get(&(players[*turn].x, players[*turn].y)) else {
        return Default::default();
    };
    match &station.kind {
        StationKind::Property { name } => name.to_string(),
        _ => "error".to_string(),
    }
}

#[wasm_bindgen]
pub fn stop_blue_station() -> i64 {
    let mut players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let money = rand::thread_rng().gen_range(1000..10000);
    players[*turn].hand_money += money;
    money
}

#[wasm_bindgen]
pub fn stop_card_station() -> String {
    let mut players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let card_id = rand::thread_rng().gen_range(0..CARD_LIST.len());
    players[*turn].card.push(card_id);
    CARD_LIST[card_id as usize].name.clone()
}

#[wasm_bindgen]
pub fn stop_red_station() -> i64 {
    let mut players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let money = rand::thread_rng().gen_range(1000..10000);
    players[*turn].hand_money -= money;
    money
}
