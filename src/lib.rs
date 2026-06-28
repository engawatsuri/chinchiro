use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn draw_station(ctx: CanvasRenderingContext2d, x: i32, y: i32, left: f64, top: f64, station_size: f64) -> Result<(&str, &str, &str, &str), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("map.csv")?;
    for result in rdr.records() {
        let record = result?;
        if record.len() < 2 {
            continue;
        }
        if &record[0] == &(x.to_string()) && &record[1] == &(y.to_string()) {
            if &record[6] == "property" {
                ctx.fill_style("purple");
                ctx.fill_rect(top, left, station_size, station_size);
            } else if &record[6] == "card" {
                ctx.fill_style("yellow");
                ctx.fill_rect(top, left, station_size, station_size);
            } else if &record[6] == "red" {
                ctx.fill_style("red");
                ctx.fill_rect(top, left, station_size, station_size);
            } else if &record[6] == "blue" {
                ctx.fill_style("blue");
                ctx.fill_rect(top, left, station_size, station_size);
            }
            return Ok(&record[2], &record[3], &record[4], &record[5]);
        }
    }
    Ok("0", "0", "0", "0")
}

#[wasm_bindgen]
pub fn draw_map(center_x: f64, center_y: f64) -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no found window")?;
    let document = window.document().ok_or("no found document")?;
    let win = document.get_element_by_id("win").ok_or("no found canvas")?.dyn_into::<HtmlCanvasElement>().map_err(|_| "HtmlCanvasElementへの変換に失敗しました")?;
    let width = win.width() as f64; // aspect ratio is 3:4
    let ctx = canvas.get_context("2d")?.ok_or("2dコンテキストの取得に失敗しました")?.dyn_into::<CanvasRenderingContext2d>()?;
    let station_size = width / 30;
    let mut x = center_x - 4;
    let mut y = center_y - 3;
    let mut left = station_size / 2;
    let mut top = station_size / 4;
    for y in -3..=3 {
        for x in -4..=4 {
            let (n, e, s, w) = draw_station(ctx, center_x + x, center_y + y, left, top, station_size);
            let n = n.parse::<f64>();
            let e = e.parse::<f64>();
            let s = s.parse::<f64>();
            let w = w.parse::<f64>();
            ctx.fill_style("black");
            if x == -4 && w != 0 {
                ctx.fill_rect(top + station_size / 4, left - station_size / 2, station_size / 2, station_size / 2);
            }
            if y == -3 && n != 0 {
                ctx.fill_rect(top - station_size / 4, left + station_size / 4, station_size / 4, station_size / 2);
            }
            if x == 4 && e != 0 {
                ctx.fill_rect(top + station_size / 4, left + station_size, station_size / 2, station_size / 2);
            } else if e != 0 {
                ctx.fill_rect(top + station_size / 4, left + station_size, station_size / 2, station_size * (7 * e - 2)/2 +);
            }
            if y == 3 && s != 0 {
                ctx.fill_rect(top + station_size, left + station_size / 4, station_size / 4, station_size / 2);
            } else if s != 0 {
                ctx.fill_rect(top + station_size, left + station_size / 4, station_size * (7 * s - 2)/2, station_size / 2);
            }
            left += station_size * 7/2;
        }
        top += station_size * 7/2;
    }
    Ok(())
}
