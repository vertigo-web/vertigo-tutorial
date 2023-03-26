use vertigo::{bind, component, css, dom, transaction, Value};

#[component]
pub fn List(items: Value<Vec<String>>) {
    let new_item = Value::<String>::default();

    let add = bind!(items, new_item, || {
        transaction(|ctx| {
            items.change(|items| items.push(new_item.get(ctx)));
            new_item.set("".to_string());
        });
    });

    let change = bind!(new_item, |new_value| {
        new_item.set(new_value);
    });

    let alternate_rows = |excl: bool| {
        let bg_color = if excl { "yellow" } else { "inherit" };

        css!("
            color: black;
            background: { bg_color };

            :nth-child(odd) {
                color: blue;
            };
        ")
    };

    let elements = items.render_list(
        |item| item.clone(),
        move |item| {
            let excl = item.ends_with('!');
            dom! {
                <li css={alternate_rows(excl)}>{item}</li>
            }
        },
    );

    let count = items.map(|items| items.len());

    dom! {
        <div>
        <p>"My list (" {count} ")"</p>
            <ol>
                {elements}
            </ol>
            <input value={new_item.to_computed()} on_input={change} />
            <button on_click={add}>"Add"</button>
        </div>
    }
}
