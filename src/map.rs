use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
struct Station {
    x: i32,
    y: i32,
    n: u8,
    e: u8,
    s: u8,
    w: u8,
    kind: StationKind,
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

static MAP: Lazy<HashMap<(i32, i32), Station>> = Lazy::new(|| load_map());

fn load_map() -> HashMap<(i32, i32), Station> {
    let mut map = HashMap::new();
    let data = include_str!("map.csv");
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
}

#[wasm_bindgen]
pub fn draw_map(center_x: f64, center_y: f64) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no found window")?;
    let document = window.document().ok_or("no found document")?;
    let win = document.get_element_by_id("win").ok_or("no found canvas")?.dyn_into::<HtmlCanvasElement>().map_err(|_| "HtmlCanvasElementへの変換に失敗しました")?;
    let width = win.width() as f64; // aspect ratio is 3:4
    let ctx = win.get_context("2d")?.ok_or("2dコンテキストの取得に失敗しました")?.dyn_into::<CanvasRenderingContext2d>()?;
    let station_size = width / 30;
    let mut left = station_size / 2;
    let mut top = station_size / 4;
    for y in -3..=3 {
        left = station_size / 2;
        for x in -4..=4 {
            if let Some(station) = MAP.get(&(center_x as i32 + x, center_y as i32 + y)) {
                match &station.kind {
                    StationKind::Property { name } => {
                        ctx.set_fill_style(&"purple".into());
                        ctx.fill_rect(left, top, station_size, station_size);
                    }
                    StationKind::Card => {
                        ctx.set_fill_style(&"yellow".into());
                        ctx.fill_rect(left, top, station_size, station_size);
                    }
                    StationKind::Red => {
                        ctx.set_fill_style(&"red".into());
                        ctx.fill_rect(left, top, station_size, station_size);
                    }
                    StationKind::Blue => {
                        ctx.set_fill_style(&"blue".into());
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
                ctx.set_fill_style(&"black".into());
                if station.w != 0 && x == -5 + station.w {
                    ctx.fill_rect(0, top + station_size / 4, station_size / 2 + (w - 1) * station_size * 3.5, station_size / 2);
                }
                if station.n != 0 && y == -4 + station.n {
                    ctx.fill_rect(left + station_size / 4, 0, station_size / 4, station_size / 2 + (n - 1) * station_size * 3.5);
                }
                if station.e != 0 && x == 4 {
                    ctx.fill_rect(left + station_size, top + station_size / 4, station_size / 2, station_size / 2);
                } else if station.e != 0 {
                    ctx.fill_rect(left + station_size, top + station_size / 4, station_size / 2, station_size * 2.5 + station_size * 3.5 * (e - 1));
                }
                if station.s != 0 && y == 3 {
                    ctx.fill_rect(left + station_size / 4, top + station_size, station_size / 4, station_size / 2);
                } else if station.s != 0 {
                    ctx.fill_rect(left + station_size / 4, top + station_size, station_size * 2.5 + station_size * 3.5 * (s - 1), station_size / 2);
                }
            }
            left += station_size * 3.5;
        }
        top += station_size * 3.5;
    }
    Ok(())
}
