use wasm_bindgen::prelude::JsValue;

use walk_the_dog as lib;

use lib::engine::GameLoop;
use lib::game::WalkTheDog;
use lib::browser;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());

    // Your code goes here!
    log::debug!("Hello world!");

    let _context = browser::context().expect("Could not get browser context");

    browser::spawn_local(async move {
        let game = WalkTheDog::new();

        GameLoop::start(game)
            .await
            .expect("Could not start game loop");
    });

    Ok(())
}
