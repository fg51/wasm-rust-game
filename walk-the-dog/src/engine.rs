use anyhow::anyhow;
use anyhow::Result;

use wasm_bindgen::prelude::{Closure, JsValue};
use wasm_bindgen::JsCast;

use web_sys::HtmlImageElement;

use futures::channel::oneshot::channel;

use crate::browser;

pub async fn load_image(source: &str) -> Result<HtmlImageElement> {
    let image = browser::new_image()?;

    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = std::rc::Rc::new(std::sync::Mutex::new(Some(complete_tx)));
    let error_tx = std::rc::Rc::clone(&success_tx);

    let success_callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            let _ = success_tx.send(Ok(()));
            //  log::debug!("loaded");
        }
    });
    let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            let _ = error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", err)));
        }
    });
    // as_ref -> JSValue , JSValue - unchecked_ref -> &Function
    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    //callback.forget();

    image.set_src(source);

    complete_rx.await??;
    Ok(image)
}
