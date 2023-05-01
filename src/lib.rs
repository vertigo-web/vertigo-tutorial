use vertigo::{bind, css, DomNode, dom, main, Value};

mod list;
use list::List;

#[main]
fn app() -> DomNode {
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

    let switch = bind!(strong, ||
        strong.change(|val| { *val = !*val; })
    );

    let title_style = css!("
        color: darkblue;
    ");

    dom! {
        <html>
            <head />
            <body>
                <div css={title_style}>"Hello " {message_element}</div>
                <button on_click={switch}>"Switch"</button>
                <List items={my_items} />
            </body>
        </html>
    }
}
