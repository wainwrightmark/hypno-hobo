use crate::web::prelude::*;
pub mod data;
pub mod web;
pub mod components;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
