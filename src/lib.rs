use wasm_bindgen::prelude::*;
mod components;
use components::Calculator;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();
    log::info!("Calculator app OK");
    
    yew::Renderer::<Calculator>::new().render();
    Ok(())
}
