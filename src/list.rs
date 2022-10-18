use vertigo::{DomElement, css, Css, dom, Value, bind, bind2};

pub struct List {
    pub items: Value<Vec<String>>,
}

impl List {
    pub fn mount(self) -> DomElement {
        let new_item = Value::<String>::default();
        let count = self.items.map(|items| items.len());

        let add = bind2(&self.items, &new_item).call(|ctx, items, new_item| {
            let mut items_vec = items.get(ctx).to_vec();
            items_vec.push(new_item.get(ctx));
            items.set(items_vec);
            new_item.set("".to_string());
        });

        let change = bind(&new_item).call_param(|_ctx, new_item, new_value| {
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
