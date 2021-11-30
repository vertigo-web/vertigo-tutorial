use wasm_bindgen::prelude::wasm_bindgen;

use vertigo::{start_app, VDomComponent};

use vertigo_browserdriver::DriverBrowser;

mod app;
mod state;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub async fn start_application() {
    // Throw panics in using console.error
    console_error_panic_hook::set_once();

    // Redirect logging into console
    wasm_logger::init(wasm_logger::Config::default());

    // Create rendering driver with dependency graph inside
    let driver = DriverBrowser::new();

    // Create application state lain on this graph
    let app_state = state::State::new(&driver);

    // Main component
    let main_component = VDomComponent::new(app_state, app::render);

    // Run component in rendering driver
    start_app(driver, main_component).await;
}
