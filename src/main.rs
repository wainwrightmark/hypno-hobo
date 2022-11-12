use crate::web::prelude::*;
pub mod components;
pub mod data;
pub mod web;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
