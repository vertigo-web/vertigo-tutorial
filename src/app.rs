use vertigo::{
    computed::Computed,
    VDomElement,
};
use vertigo::{html, css_fn};

use super::state::State;

css_fn! { main_div, "
    color: darkblue;
" }

pub fn render(app_state: &Computed<State>) -> VDomElement {
    let state = app_state.get_value();

    html! {
        <div css={main_div()}>
            "Message to the world: "
            {state.message.get_value()}
        </div>
    }
}
