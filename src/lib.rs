mod render;
mod state;

use vertigo::start_app;

#[no_mangle]
pub fn start_application() {
    start_app(state::State::component);
}
