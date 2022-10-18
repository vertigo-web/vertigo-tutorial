use vertigo::{start_app, bind, DomElement, Value, dom, css_fn};

mod list;
use list::List;

css_fn! { main_div, "
    color: darkblue;
" }

fn app() -> DomElement {
    let message = Value::new("Hello world!".to_string());
    let strong = Value::new(true);

    let my_items = Value::new(
        vec![
            "Item1".to_string(),
            "Item2".to_string(),
        ]
    );

    let message_element = strong.render_value(move |strong|
        if strong {
            dom! { <strong>{&message}</strong> }
        } else {
            dom! { <span>{&message}</span> }
        }
    );

    let switch = bind(&strong).call(|ctx, strong|
        strong.set(
            !strong.get(ctx)
        )
    );

    dom! {
        <div css={main_div()}>
            "Message to the world: "
            {message_element}
            <button on_click={switch}>"Switch"</button>
            <List items={my_items}/>
        </div>
    }
}

#[no_mangle]
pub fn start_application() {
    start_app(app);
}
