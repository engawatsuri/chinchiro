use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::collections::HashMap;
use std::sync::LazyLock;
use rand::Rng;
use std::f64::consts::PI;
use crate::player::*;
use crate::*;

#[derive(Clone)]
pub struct Station {
    pub x: i32,
    pub y: i32,
    pub north: u8,
    pub north_kind: LineKind,
    pub east: u8,
    pub east_kind: LineKind,
    pub south: u8,
    pub south_kind: LineKind,
    pub west: u8,
    pub west_kind: LineKind,
    pub kind: StationKind,
}

#[derive(Clone, PartialEq)]
pub enum StationKind {
    Airport {
        name: String,
    },
    Card,
    Dubbing {
        name: String,
    },
    Heliport {
        name: String,
    },
    Katanagari {
        name: String,
    },
    Lottery {
        name: String,
    },
    Minus,
    Nice {
        name: String,
    },
    Plus,
    Property {
        name: String,
    },
    Shop {
        name: String,
    },
    Tour {
        name: String,
    },
    Whirling {
        name: String,
    },
}

impl StationKind {
    pub fn as_initial(&self) -> &str {
        match self {
            Self::Airport {..} => "ai",
            Self::Card => "ca",
            Self::Dubbing {..} => "du",
            Self::Heliport {..} => "he",
            Self::Katanagari {..} => "ka",
            Self::Lottery {..} => "lo",
            Self::Minus => "mi",
            Self::Nice {..} => "ni",
            Self::Plus => "pl",
            Self::Property {..} => "pr",
            Self::Shop {..} => "sh",
            Self::Tour {..} => "to",
            Self::Whirling {..} => "wh",
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Airport {..} => "空港",
            Self::Card => "カード",
            Self::Dubbing {..} => "ダビング",
            Self::Heliport {..} => "ヘリポート",
            Self::Katanagari {..} => "刀がり",
            Self::Lottery {..} => "宝くじ",
            Self::Minus => "マイナス",
            Self::Nice {..} => "ナイスカード",
            Self::Plus => "プラス",
            Self::Property {..} => "物件",
            Self::Shop {..} => "カード売買",
            Self::Tour {..} => "周遊",
            Self::Whirling {..} => "渦潮",
        }
    }

    pub fn get_name(&self) -> Option<&str> {
        match self {
            Self::Airport {name} => Some(name),
            Self::Card => None,
            Self::Dubbing {name} => Some(name),
            Self::Heliport {name} => Some(name),
            Self::Katanagari {name} => Some(name),
            Self::Lottery {name} => Some(name),
            Self::Minus => None,
            Self::Nice {name} => Some(name),
            Self::Plus => None,
            Self::Property {name} => Some(name),
            Self::Shop {name} => Some(name),
            Self::Tour {name} => Some(name),
            Self::Whirling {name} => Some(name),
        }
    }
}

#[derive(Clone)]
pub enum LineKind {
    Air,
    Sea,
    Track,
}

impl LineKind {
    fn from(id: &str) -> Option<LineKind> {
        match id {
            "ar" => Some(LineKind::Air),
            "sr" => Some(LineKind::Sea),
            "rt" => Some(LineKind::Track),
            _ => None,
        }
    }
}

pub static MAP: LazyLock<HashMap<(i32, i32), Station>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    let data = include_str!("../map.csv");
    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.records() {
        let record = result.unwrap();
        let x = record[0].parse::<i32>().expect(&format!("{}, {}: x is blocken", &record[0], &record[1]));
        let y = record[1].parse::<i32>().expect(&format!("{}, {}: y is blocken", &record[0], &record[1]));
        let station = Station {
            x: x,
            y: y,
            north: record[2].parse().expect(&format!("{}, {}: north is blocken", x, y)),
            north_kind: LineKind::from(&record[3]).ok_or(&format!("{}, {}: north_kind is blocken", x, y)).unwrap(),
            east: record[4].parse().expect(&format!("{}, {}: east is blocken", x, y)),
            east_kind: LineKind::from(&record[5]).ok_or(&format!("{}, {}: east_kind is blocken", x, y)).unwrap(),
            south: record[6].parse().expect(&format!("{}, {}: south is blocken", x, y)),
            south_kind: LineKind::from(&record[7]).ok_or(&format!("{}, {}: south_kind is blocken", x, y)).unwrap(),
            west: record[8].parse().expect(&format!("{}, {}: west is blocken", x, y)),
            west_kind: LineKind::from(&record[9]).ok_or(&format!("{}, {}: west_kind is blocken", x, y)).unwrap(),
            kind: match &record[10] {
                "ai" => {
                    StationKind::Airport {
                        name: record[11].to_string(),
                    }
                }
                "ca" => StationKind::Card,
                "du" => {
                    StationKind::Dubbing {
                        name: record[11].to_string(),
                    }
                }
                "he" => {
                    StationKind::Heliport {
                        name: record[11].to_string(),
                    }
                }
                "ka" => {
                    StationKind::Katanagari {
                        name: record[11].to_string(),
                    }
                }
                "lo" => {
                    StationKind::Lottery {
                        name: record[11].to_string(),
                    }
                }
                "mi" => StationKind::Minus,
                "ni" => {
                    StationKind::Nice {
                        name: record[11].to_string(),
                    }
                }
                "pl" => StationKind::Plus,
                "pr" => {
                    StationKind::Property {
                        name: record[11].to_string(),
                    }
                }
                "sh" => {
                    StationKind::Shop {
                        name: record[11].to_string(),
                    }
                }
                "to" => {
                    StationKind::Tour {
                        name: record[11].to_string(),
                    }
                }
                "wh" => {
                    StationKind::Whirling {
                        name: record[11].to_string(),
                    }
                }
                _ => panic!("{}, {}: kind is blocken", x, y),
            },
        };
        map.insert((x, y), station);
    }
    map
});

pub fn draw_map() -> Result<(), JsValue> {
    let width = WIN.width() as f64; // aspect ratio is 3:4
    let station_size = width / 30.0;
    let mut left = station_size * 0.5;
    let mut top = station_size * 0.25;
    let players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();
    let year = YEAR.lock().unwrap();
    let month = MONTH.lock().unwrap();
    let goal = GOAL.lock().unwrap();
    let center_x = players[*turn].x;
    let center_y = players[*turn].y;

    // draw station and line
    for y in -3i32..=3 {
        left = station_size * 0.5;
        for x in -4i32..=4 {
            if let Some(station) = MAP.get(&(center_x + x, center_y + y)) {
                CTX.set_fill_style_str("white");
                CTX.fill_rect(left, top, station_size, station_size);
                match &station.kind {
                    StationKind::Airport {..} => {
                    }
                    StationKind::Card => {
                        CTX.set_fill_style_str("yellow");
                        CTX.begin_path();
                        CTX.move_to(left + station_size / 2.0, top);
                        CTX.line_to(left, top + station_size / 2.0);
                        CTX.line_to(left + station_size / 2.0, top + station_size);
                        CTX.line_to(left + station_size, top + station_size / 2.0);
                        CTX.close_path();
                        CTX.fill();
                    }
                    StationKind::Dubbing {..} => {
                        CTX.set_fill_style_str("black");
                        CTX.fill_rect(left, top, station_size / 2.0, station_size / 2.0);
                        CTX.fill_rect(left + station_size / 2.0, top + station_size / 2.0, station_size / 2.0, station_size / 2.0);
                    }
                    StationKind::Heliport {..} => {
                        CTX.set_fill_style_str("green");
                        CTX.begin_path();
                        CTX.arc(left + station_size / 2.0, top + station_size / 2.0, station_size / 2.0, 0.0, PI * 2.0)?;
                        CTX.fill();
                    }
                    StationKind::Katanagari {..} => {
                    }
                    StationKind::Lottery {..} => {
                        CTX.set_fill_style_str("black");
                        CTX.fill_rect(left + station_size / 4.0, top + station_size / 4.0, station_size / 2.0, station_size / 2.0);
                    }
                    StationKind::Minus => {
                        CTX.set_fill_style_str("red");
                        CTX.fill_rect(left, top + station_size / 2.0, station_size, station_size / 2.0);
                    }
                    StationKind::Nice {..} => {
                        CTX.set_fill_style_str("orange");
                        CTX.begin_path();
                        CTX.move_to(left + station_size / 2.0, top);
                        CTX.line_to(left, top + station_size / 2.0);
                        CTX.line_to(left + station_size / 2.0, top + station_size);
                        CTX.line_to(left + station_size, top + station_size / 2.0);
                        CTX.close_path();
                        CTX.fill();
                    }
                    StationKind::Plus => {
                        CTX.set_fill_style_str("blue");
                        CTX.fill_rect(left, top, station_size / 2.0, station_size);
                    }
                    StationKind::Property {..} => {
                        CTX.set_fill_style_str("purple");
                        CTX.begin_path();
                        CTX.move_to(left + station_size / 2.0, top);
                        CTX.line_to(left, top + station_size / 2.0);
                        CTX.line_to(left + station_size, top + station_size / 2.0);
                        CTX.close_path();
                        CTX.fill();
                    }
                    StationKind::Shop {..} => {
                        CTX.set_fill_style_str("purple");
                        CTX.begin_path();
                        CTX.move_to(left + station_size / 2.0, top);
                        CTX.line_to(left, top + station_size / 2.0);
                        CTX.line_to(left + station_size, top + station_size / 2.0);
                        CTX.close_path();
                        CTX.fill();
                        CTX.set_fill_style_str("yellow");
                        CTX.begin_path();
                        CTX.move_to(left + station_size / 2.0, top + station_size / 2.0);
                        CTX.line_to(left + station_size / 4.0, top + station_size * 3.0/4.0);
                        CTX.line_to(left + station_size / 2.0, top + station_size);
                        CTX.line_to(left + station_size * 3.0/4.0, top + station_size * 3.0/4.0);
                        CTX.close_path();
                        CTX.fill();
                    }
                    StationKind::Tour {..} => {
                    }
                    StationKind::Whirling {..} => {
                    }
                }
                if station.x == goal.0 && station.y == goal.1 {
                    CTX.set_line_width(station_size / 30.0);
                    CTX.set_stroke_style_str("brown");
                    CTX.stroke_rect(left, top, station_size, station_size);
                } else {
                    CTX.set_line_width(station_size / 100.0);
                    CTX.set_stroke_style_str("black");
                    CTX.stroke_rect(left, top, station_size, station_size);
                }
                let n = station.north as f64;
                let e = station.east as f64;
                let s = station.south as f64;
                let w = station.west as f64;
                CTX.set_fill_style_str("black");
                if station.west != 0 && x == -5 + station.west as i32 {
                    CTX.fill_rect(0.0, top + station_size * 0.25, station_size * -3.0 + station_size * 3.5 * w, station_size * 0.5);
                }
                if station.north != 0 && y == -4 + station.north as i32 {
                    CTX.fill_rect(left + station_size * 0.25, 0.0, station_size * 0.25, station_size * -3.0 + station_size * 3.5 * n);
                }
                if station.east != 0 && x == 4 {
                    CTX.fill_rect(left + station_size, top + station_size * 0.25, station_size * 0.5, station_size * 0.5);
                } else if station.east != 0 {
                    CTX.fill_rect(left + station_size, top + station_size * 0.25, station_size * -1.0 + station_size * 3.5 * e, station_size * 0.5);
                }
                if station.south != 0 && y == 3 {
                    CTX.fill_rect(left + station_size * 0.25, top + station_size, station_size * 0.25, station_size * 0.5);
                } else if station.south != 0 {
                    CTX.fill_rect(left + station_size * 0.25, top + station_size, station_size * 0.5, station_size * -1.0 + station_size * 3.5 * s);
                }
            }
            left += station_size * 3.5;
        }
        top += station_size * 3.5;
    }

    // draw player
    for i in 0..players.len() {
        let index = (*turn + i) % players.len();
        let player = players[index].clone();
        CTX.set_fill_style_str(&player.color);
        if center_x - 4 <= player.x && player.x <= center_x + 4 && center_y - 3 <= player.y && player.y <= center_y + 3 {
            match player.dir {
                Dir::North => {
                    if player.x == center_y + 4 {
                        CTX.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 10.75 + station_size * 3.5 * (player.y - center_y) as f64 + station_size * 0.125, station_size * 0.75, station_size * 1.125);
                    } else {
                        CTX.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 10.75 + station_size * 3.5 * (player.y - center_y) as f64 + station_size * 0.125, station_size * 0.75, station_size * 1.5);
                    }
                }
                Dir::East => {
                    if player.x == center_x - 4 {
                        CTX.fill_rect(0.0, station_size * 0.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 1.5, station_size * 0.75);
                    } else {
                        CTX.fill_rect(station_size * -1.0 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 0.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 1.5, station_size * 0.75);
                    }
                }
                Dir::South => {
                    if player.x == center_x - 4 {
                        CTX.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, 0.0, station_size * 0.75, station_size * 1.5);
                    } else {
                        CTX.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * -1.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 0.75, station_size * 1.5);
                    }
                }
                Dir::West => {
                    if player.y == center_y + 3 {
                        CTX.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 0.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 1.5, station_size * 0.75);
                    } else {
                        CTX.fill_rect(station_size * 0.5 + station_size * 3.5 * (player.x - center_x + 4) as f64 + station_size * 0.125, station_size * 0.25 + station_size * 3.5 * (player.y - center_y + 3) as f64 + station_size * 0.125, station_size * 1.375, station_size * 0.75);
                    }
                }
                _ => {
                }
            }
        }
    }

    // draw info bar
    CTX.set_fill_style_str(&players[*turn].color);
    CTX.fill_rect(width * 3.0/56.0, width * 3.0/112.0, width * 25.0/28.0, width * 9.0/56.0);
    CTX.fill_rect(width * 3.0/14.0, width * 1.0/28.0, width * 9.0/16.0, width * 1.0/7.0);
    CTX.set_fill_style_str("black");
    CTX.set_text_baseline("top");
    CTX.set_font(&format!("{}px 'Meiryo UI'", width * 5.0/56.0));
    CTX.fill_text(&format!("{}社長", players[*turn].name), width * 25.0/112.0, width * 1.0/28.0);
    CTX.fill_text(&format!("{}㌽", players[*turn].item.len()), width * 87.0/112.0, width * 1.0/14.0);
    CTX.set_font(&format!("{}px 'Meiryo UI'", width * 3.0/56.0));
    if players[*turn].hand_money < 0 {
        CTX.set_fill_style_str("red");
    }
    CTX.fill_text(&format!("{}円", players[*turn].hand_money), width * 25.0/112.0, width * 0.125);
    if players[*turn].hand_money < 0 {
        CTX.set_fill_style_str("black");
    }
    CTX.fill_text(&format!("{}年目 {}月", year, month), width * 0.5, width * 0.125);

    Ok(())
}

pub fn draw() -> Result<(), JsValue> {
    let page = PAGE.lock().unwrap();
    CTX.clear_rect(0.0, 0.0, WIN.width() as f64, WIN.height() as f64);

    match page.as_str() {
        "arrival" => {
            draw_map()?;
        }
        "create0" | "create1" => {
        }
        "dice" => {
            draw_map()?;
        }
        "home" => {
        }
        "movement" => {
            draw_map()?;
        }
        "start" => {
            draw_map()?;
        }
        "station_ai" | "station_ca" | "station_du" | "station_he" |
        "station_ka" | "station_lo" | "station_mi" | "station_ni" |
        "station_pl" | "station_pr" | "station_sh" | "station_to" | "station_wh" => {
            draw_map()?;
        }
        _ => {
            panic!("no found page");
        }
    }
    Ok(())
}
