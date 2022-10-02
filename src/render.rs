use vertigo::{DomElement, dom, css_fn, bind};

use crate::state::State;

css_fn! { main_div, "
    color: darkblue;
" }

pub fn render(state: &State) -> DomElement {
    let message = state.message.clone();

    let message_element = state.strong.render_value(move |strong|
        if strong {
            dom! { <strong><text computed={message.to_computed()}/></strong> }
        } else {
            dom! { <span><text computed={message.to_computed()}/></span> }
        }
    );

    let switch = bind(&state.strong).call(|ctx, strong|
        strong.set(
            !strong.get(ctx)
        )
    );

    dom! {
        <div css={main_div()}>
            "Message to the world: "
            {message_element}
            <button on_click={switch}>"Switch"</button>
            {state.list.render()}
        </div>
    }
}
