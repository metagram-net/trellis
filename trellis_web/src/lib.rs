use wasm_bindgen::prelude::*;

mod app;

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/version"));
const SOURCE_URL: &'static str = "https://github.com/metagram-net/trellis";

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    yew::start_app_with_props::<app::App>(app::Props {
        version: VERSION,
        source_url: SOURCE_URL,
    });
    Ok(())
}
