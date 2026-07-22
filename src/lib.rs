#![allow(warnings)]

pub mod player;
pub mod draw;
pub mod init;

use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
    HtmlButtonElement,
    Element,
    DomRect,
};
use std::sync::{LazyLock, Mutex};
use console_error_panic_hook;
use draw::*;
use init::*;

pub static WINDOW: LazyLock<web_sys::Window> = LazyLock::new(|| web_sys::window().ok_or("no found window").unwrap());
pub static DOCUMENT: LazyLock<web_sys::Document> = LazyLock::new(|| web_sys::window().ok_or("no found window").unwrap().document().ok_or("no found document").unwrap());
pub static WIN: LazyLock<HtmlCanvasElement> = LazyLock::new(|| DOCUMENT.get_element_by_id("win").ok_or("no found win").unwrap().dyn_into::<HtmlCanvasElement>().map_err(|_| "HtmlCanvasElementへの変換に失敗しました").unwrap());
pub static MSG: LazyLock<HtmlButtonElement> = LazyLock::new(|| DOCUMENT.get_element_by_id("msg").ok_or("no found win").unwrap().dyn_into::<HtmlButtonElement>().map_err(|_| "HtmlCanvasElementへの変換に失敗しました").unwrap());
pub static CTX: LazyLock<CanvasRenderingContext2d> = LazyLock::new(|| WIN.get_context("2d").unwrap().ok_or("2dコンテキストの取得に失敗しました").unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap());
pub static MONTH: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(4));
pub static YEAR: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
pub static PAGE: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));
pub static LINE: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
pub static GOAL: LazyLock<Mutex<(i32, i32)>> = LazyLock::new(|| Mutex::new((0, 0)));

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn change_page(id: &str) -> Result<(), JsValue> {
    let mut page = PAGE.lock().unwrap();
    let mut line = LINE.lock().unwrap();
    let all_page = DOCUMENT.query_selector_all(".page")?;
    for i in 0..all_page.length() {
        let el: Element = all_page.get(i).ok_or("cannot get page").unwrap().dyn_into()?;
        el.class_list().remove_1("active")?;
    }
    let active_page = DOCUMENT.get_element_by_id(id).ok_or(format!("error: {}: no found page", id)).unwrap();
    active_page.class_list().add_1("active")?;
    *page = id.to_string();
    *line = 0;
    init()?;
    draw()?;
    Ok(())
}

#[wasm_bindgen]
pub fn resize_win() -> Result<(), JsValue> {
    let dpr = WINDOW.device_pixel_ratio();
    let rect: DomRect = WIN.get_bounding_client_rect();
    WIN.set_width((rect.width() * dpr).round() as u32);
    WIN.set_height((rect.height() * dpr).round() as u32);
    let style = WIN.style();
    style.set_property("width", &format!("{}px", rect.width()))?;
    style.set_property("height", &format!("{}px", rect.height()))?;
    CTX.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)?;
    draw()?;
    Ok(())
}

