use vertigo::{start_app, DomElement, Value, dom, css_fn};

css_fn! { main_div, "
    color: darkblue;
" }

fn app() -> DomElement {
    let message = Value::new("Hello world!".to_string());

    dom! {
        <div css={main_div()}>
            "Message to the world: "
            { message }
        </div>
    }
}

#[no_mangle]
pub fn start_application() {
    start_app(app);
}
