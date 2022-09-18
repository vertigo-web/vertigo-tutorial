use vertigo::{DomElement, dom, css_fn};

use super::state::State;

css_fn! { main_div, "
    color: darkblue;
" }

pub fn render(state: &State) -> DomElement {
    dom! {
        <div css={main_div()}>
            "Message to the world: "
            <text computed={state.message.to_computed()} />
        </div>
    }
}
