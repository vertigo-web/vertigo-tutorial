use vertigo::{DomElement, css, Css, dom, Value, bind, transaction};

pub struct List {
    pub items: Value<Vec<String>>,
}

impl List {
    pub fn mount(self) -> DomElement {
        let new_item = Value::<String>::default();
        let count = self.items.map(|items| items.len());

        let items = self.items.clone();
        let add = bind!(items, new_item, || {
            transaction(|ctx| {
                items.change(|items| items.push(new_item.get(ctx)));
                new_item.set("".to_string());
            });
        });

        let change = bind!(new_item, |new_value| {
            new_item.set(new_value);
        });

        let elements = self.items.render_list(
            |item| item.clone(),
            |item| {
                let excl = item.ends_with('!');
                dom! { <li css={alternate_rows(excl)}>{item}</li> }
            },
        );

        dom! {
            <div>
                <p>"My list (" {count} ")"</p>
                <ol>
                    { elements }
                </ol>
                <input value={new_item.to_computed()} on_input={change} />
                <button on_click={add}>"Add"</button>
            </div>
        }
    }
}

fn alternate_rows(excl: bool) -> Css {
    let bg_color = if excl { "yellow" } else { "inherit" };

    css!("
        color: black;
        background: { bg_color };

        :nth-child(odd) {
            color: blue;
        };
    ")
}
