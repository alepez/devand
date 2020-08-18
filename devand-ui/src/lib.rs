#![recursion_limit = "512"]
#![feature(exact_size_is_empty)]

mod app;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::initialize();

    let document = yew::utils::document();
    let mount_point = document
        .query_selector(".yew-mount-point")
        .unwrap()
        .unwrap();

    yew::App::<app::App>::new().mount(mount_point);
    yew::run_loop();
    Ok(())
}
