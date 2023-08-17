//use wasm_bindgen::prelude::*;
//use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::prelude::JsValue;
//use web_sys::console;

use wasm_bindgen::JsCast;

use serde::Deserialize;
use std::collections::HashMap;

use walk_the_dog as lib;

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

    //let window = web_sys::window().unwrap();
    let window = lib::browser::window().expect("No Window Found");

    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    wasm_bindgen_futures::spawn_local(async move {
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = std::rc::Rc::new(std::sync::Mutex::new(Some(success_tx)));
        let error_tx = std::rc::Rc::clone(&success_tx);

        let image = web_sys::HtmlImageElement::new().unwrap();

        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                success_tx.send(Ok(())).unwrap();
                //  log::debug!("loaded");
            }
        });
        let error_callback = Closure::once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                error_tx.send(Err(err)).unwrap();
            }
        });
        // as_ref -> JSValue , JSValue - unchecked_ref -> &Function
        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
        //callback.forget();

        image.set_src("/assets/Idle (1).png");
        //image.set_src("/assets/Idle (2).png");
        let _ = success_rx.await;

        context
            .draw_image_with_html_image_element(&image, 0.0, 0.0)
            .unwrap();

        // -------------------------

        let json: JsValue = fetch_json("/assets/rhb.json")
            .await
            .expect("Could not fetch rhb.json");
        //let sheet: Sheet = json
        //    .into_serde()
        //    .expect("Could not convert rhb.json into a Sheet structure");
        let sheet: Sheet = serde_wasm_bindgen::from_value(json)
            .expect("Could not convert rhb.json into a Sheet structure");

        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let success_tx = std::rc::Rc::new(std::sync::Mutex::new(Some(success_tx)));
        let error_tx = std::rc::Rc::clone(&success_tx);

        let image = web_sys::HtmlImageElement::new().unwrap();

        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
                success_tx.send(Ok(())).unwrap();
                //  log::debug!("loaded");
            }
        });
        let error_callback = Closure::once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
                error_tx.send(Err(err)).unwrap();
            }
        });
        // as_ref -> JSValue , JSValue - unchecked_ref -> &Function
        image.set_onload(Some(callback.as_ref().unchecked_ref()));
        image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
        //callback.forget();

        image.set_src("/assets/rhb.png");
        let _ = success_rx.await;

        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);
            context.clear_rect(0.0, 0.0, 600.0, 600.0);
            let sprite = sheet.frames.get(&frame_name).expect("Cell not found");
            context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    &image,
                    sprite.frame.x.into(),
                    sprite.frame.y.into(),
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                    300.0,
                    300.0,
                    sprite.frame.w.into(),
                    sprite.frame.h.into(),
                )
                .unwrap();
        }) as Box<dyn FnMut()>);

        window
            .set_interval_with_callback_and_timeout_and_arguments_0(
                interval_callback.as_ref().unchecked_ref(),
                50,
            )
            .unwrap();
        interval_callback.forget();
    });

    Ok(())
}

async fn fetch_json(json_path: &str) -> Result<JsValue, JsValue> {
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_str(json_path)).await?;
    let resp: web_sys::Response = resp_value.dyn_into()?;
    wasm_bindgen_futures::JsFuture::from(resp.json()?).await
}

#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}
