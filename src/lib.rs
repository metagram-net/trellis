use wasm_bindgen::prelude::*;

mod app;

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app::<app::App>();
    Ok(())
}
