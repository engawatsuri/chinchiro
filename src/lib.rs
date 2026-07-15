#![allow(warnings)]

pub mod player;
pub mod map;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::sync::{LazyLock, Mutex};
use console_error_panic_hook;

pub static MONTH: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(4));
pub static YEAR: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}
