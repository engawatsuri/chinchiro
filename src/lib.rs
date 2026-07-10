#![allow(warnings)]

pub mod player;
pub mod map;

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use std::sync::{LazyLock, Mutex};

pub static MONTH: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(4));
pub static YEAR: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(1));
