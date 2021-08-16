use wasm_bindgen::prelude::*;

mod app;

const VERSION_INFO: &'static str = include_str!(concat!(env!("OUT_DIR"), "/version"));

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app_with_props::<app::App>(app::Props {
        version_info: VERSION_INFO,
    });
    Ok(())
}
