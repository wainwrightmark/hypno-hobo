use crate::web::*;
pub mod components;
pub mod data;
pub mod web;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    wasm_bindgen_futures::spawn_local(CreationState::setup());
    yew::Renderer::<App>::new().render();
}
