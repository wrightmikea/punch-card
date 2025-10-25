// IBM 1130 Punch Card Simulator - Web Application
//
// Yew-based web interface for the punch card simulator

use wasm_bindgen::prelude::*;

mod components;

use components::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<App>::new().render();
}
