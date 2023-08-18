//use wasm_bindgen::prelude::*;
//use wasm_bindgen::prelude::wasm_bindgen;
//use wasm_bindgen::prelude::JsValue;
//use web_sys::console;
//
//use wasm_bindgen::JsCast;
//
//use rand::prelude::{thread_rng, Rng};

pub mod browser;
pub mod engine;
pub mod game;

// // When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// // allocator.
// //
// // If you don't want to use `wee_alloc`, you can safely delete this.
// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is like the `main` function, except for JavaScript.
//#[wasm_bindgen(start)]
//pub fn main_js() -> Result<(), JsValue> {
//    console_error_panic_hook::set_once();
//    wasm_logger::init(wasm_logger::Config::default());
//
//    // Your code goes here!
//    //console::log_1(&JsValue::from_str("Hello world!"));
//    log::debug!("Hello world!");
//
//    Ok(())
//}
