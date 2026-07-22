use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d,
    HtmlCanvasElement,
    HtmlInputElement,
    KeyboardEvent,
};
use std::sync::{LazyLock, Mutex};
use rand::Rng;
use crate::draw::*;
use crate::player::*;
use crate::*;

pub static SELECTED: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
pub static SELECTED_RANGE: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));
pub static INITIALIZED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

fn btn_sys(key: &str) -> Result<(), JsValue> {
    let mut selected = SELECTED.lock().unwrap();
    let selected_range = SELECTED_RANGE.lock().unwrap();
    let mut initialized = INITIALIZED.lock().unwrap();
    let page = PAGE.lock().unwrap();

    let new_selected = match key {
        "Enter" => {
            let btn = DOCUMENT.get_element_by_id("msg").ok_or("no found win").unwrap().dyn_into::<HtmlButtonElement>()?;
            btn.class_list().remove_1("selected")?;
            btn.click();
            return Ok(());
        }
        "J" => *selected_range - 1,
        "j" => (*selected + 1) % *selected_range,
        "K" => 0,
        "k" => (*selected - 1 + *selected_range) % *selected_range,
        _ => {
            return Ok(());
        }
    };
    if *initialized {
        let btn = DOCUMENT.get_element_by_id(&format!("{}{}", *page, *selected)).ok_or(format!("error: {}: no found page", *page)).unwrap();
        btn.class_list().remove_1("selected")?;
    } else {
        *initialized = true;
    }
    let new_btn = DOCUMENT.get_element_by_id(&format!("{}{}", *page, new_selected)).ok_or(format!("error: {}: no found page", *page)).unwrap();
    new_btn.class_list().add_1("selected")?;
    *selected = new_selected;
    Ok(())
}

pub fn change_goal() -> String {
    let mut goal = GOAL.lock().unwrap();
    let stations: Vec<_> = MAP.values().collect();
    loop {
        let id = rand::thread_rng().gen_range(0..stations.len());
        let station = stations[id as usize];
        let Some(name) = station.kind.get_name() else {
            continue;
        };
        *goal = (station.x, station.y).clone();
        break name.to_string();
    }
}

pub fn init() -> Result<(), JsValue> {
    let page = PAGE.lock().unwrap();
    let mut line = LINE.lock().unwrap();
    let mut initialized = INITIALIZED.lock().unwrap();
    let mut selected_range = SELECTED_RANGE.lock().unwrap();
    let mut players = PLAYERS.lock().unwrap();
    let turn = TURN.lock().unwrap();

    match page.as_str() {
        "card" | "item" | "search" | "other" => {
            // unsetting page
            MSG.style().set_property("display", "block")?;
            MSG.set_text_content(Some("未設定のページです"));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" | "Escape" => {
                        change_page("start").unwrap();
                    }
                    &_ => {}
                }
            });
        }
        "arrival" => {
            let Some(station) = MAP.get(&(players[*turn].x, players[*turn].y)) else {
                panic!("enter a private place");
            };
            MSG.style().set_property("display", "block")?;
            MSG.set_text_content(Some(&format!("{}{}駅に到着しました", if let Some(name) = station.kind.get_name() {
                format!("｢{}｣", name)
            } else {
                "".to_string()
            }, station.kind.as_str())));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" => {
                        change_page(&format!("station_{}", station.kind.as_initial())).unwrap();
                    }
                    &_ => {}
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "arrival_goal" => {
            MSG.style().set_property("display", "block")?;
            if players[*turn].hand_money < 0 {
                MSG.set_text_content(Some("ゴールに到着しました．借金を住民が肩代わりした"));
                players[*turn].hand_money = 0;
            } else {
                MSG.set_text_content(Some("ゴールに到着しました．10億円を獲得した"));
                players[*turn].hand_money += 100000;
            }
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match *line {
                    0 => {
                        match e.key().as_str() {
                            "Enter" => {
                                if rand::thread_rng().gen_range(0..10) == 0 {
                                    for i in 0..players.len() {
                                        if i == *turn {
                                            continue;
                                        } else {
                                            let monay = (players[i].hand_money as f64 * 0.3) as i64;
                                            players[i].hand_money -= monay;
                                            players[*turn].hand_money += monay;
                                        }
                                    }
                                } else {
                                    MSG.set_text_content(Some(&format!("次のゴール地点は{}です", change_goal())));
                                    *line = 1;
                                }
                            }
                            &_ => {}
                        }
                    }
                    1 => {
                        match e.key().as_str() {
                            "Enter" => {
                                change_page("arrival");
                            }
                            &_ => {}
                        }
                    }
                    _ => {}
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "create0" => {
            MSG.style().set_property("display", "block")?;
            MSG.set_text_content(Some("社長さんの名前とあなたの汽車を設定してください"));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" => {
                        let players_el: [HtmlInputElement; 4] = [
                            DOCUMENT.get_element_by_id("player0").ok_or("no found player0").unwrap().dyn_into::<HtmlInputElement>().unwrap(),
                            DOCUMENT.get_element_by_id("player1").ok_or("no found player1").unwrap().dyn_into::<HtmlInputElement>().unwrap(),
                            DOCUMENT.get_element_by_id("player2").ok_or("no found player2").unwrap().dyn_into::<HtmlInputElement>().unwrap(),
                            DOCUMENT.get_element_by_id("player3").ok_or("no found player3").unwrap().dyn_into::<HtmlInputElement>().unwrap(),
                        ];
                        if players_el[0].value() != "" {
                            for el in players_el {
                                if el.value() == "" {
                                    return;
                                } else {
                                    add_player(&el.value());
                                }
                            }
                        }
                        change_page("create1");
                    }
                    "Escape" => {
                        change_page("home");
                    }
                    &_ => {}
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "create1" => {
            MSG.style().set_property("display", "block")?;
            MSG.set_text_content(Some("本作の目標はお金を集めることではありません。様々な事象でポイントを集めることです。その事象の中には、総資産が少なくないと発生しないものもあります。まずは100ポイントを目指して頑張ってください"));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" => {
                        change_page("setting_goal");
                    }
                    "Escape" => {
                        change_page("create0");
                    }
                    &_ => {}
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "dice" => {
            let dices = players[*turn].roll();
            MSG.style().set_property("display", "block")?;
            MSG.set_text_content(Some(&format!("サイコロの出目は({}, {}, {})です", dices[0], dices[1], dices[2])));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" => {
                        change_page("movement");
                    }
                    &_ => {}
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "home" => {
            *initialized = false;
            *selected_range = 4;
            btn_sys("K");
            MSG.style().set_property("display", "block")?;
            MSG.set_text_content(Some("チンチロ電車(えんがわ釣り・作)へようこそ"));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                btn_sys(&e.key()).unwrap();
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "movement" => {
            MSG.style().set_property("display", "none")?;
            MSG.set_text_content(Some(""));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                let Some(dir) = Dir::from_key(&e.key()) else {
                    return;
                };
                let Some(station) = players[*turn].go(dir).unwrap() else {
                    let distance = DISTANCE.lock().unwrap();
                    draw().unwrap();
                    MSG.set_text_content(Some(&format!("残り{}マス", *distance)));
                    return;
                };
                let goal = GOAL.lock().unwrap();
                if players[*turn].x == goal.0 && players[*turn].y == goal.1 {
                    change_page("arrival_goal").unwrap();
                } else {
                    change_page("arrival").unwrap();
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "setting_goal" => {
            MSG.style().set_property("display", "none")?;
            MSG.set_text_content(Some(&format!("みなさんの目的地は{}です", change_goal())));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" => {
                        change_page("start");
                    }
                    &_ => {}
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "start" => {
            *initialized = false;
            *selected_range = 5;
            btn_sys("K");
            MSG.style().set_property("display", "none")?;
            MSG.set_text_content(Some(""));
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                btn_sys(&e.key()).unwrap();
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "station_ai" => {
            next_turn();
            change_page("start")?;
        }
        "station_ca" => {
            next_turn();
            change_page("start")?;
        }
        "station_du" => {
            next_turn();
            change_page("start")?;
        }
        "station_he" => {
            next_turn();
            change_page("start")?;
        }
        "station_ka" => {
            next_turn();
            change_page("start")?;
        }
        "station_lo" => {
            next_turn();
            change_page("start")?;
        }
        "station_mi" => {
            MSG.style().set_property("display", "block")?;
            MSG.set_text_content(Some("1億円失いました"));
            players[*turn].hand_money -= 10000;
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" => {
                        next_turn();
                        change_page("start").unwrap();
                    }
                    &_ => {}
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "station_ni" => {
            next_turn();
            change_page("start")?;
        }
        "station_pl" => {
            MSG.style().set_property("display", "block")?;
            MSG.set_text_content(Some("1億円取得しました"));
            players[*turn].hand_money += 10000;
            let closure = Closure::<dyn FnMut(_)>::new(move |e: KeyboardEvent| {
                match e.key().as_str() {
                    "Enter" => {
                        next_turn();
                        change_page("start").unwrap();
                    }
                    &_ => {}
                }
            });
            WINDOW.set_onkeydown(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
        }
        "station_pr" => {
            next_turn();
            change_page("start")?;
        }
        "station_sh" => {
            next_turn();
            change_page("start")?;
        }
        "station_to" => {
            next_turn();
            change_page("start")?;
        }
        "station_wh" => {
            next_turn();
            change_page("start")?;
        }
        _ => {
            panic!("no found page");
        }
    }
    Ok(())
}
