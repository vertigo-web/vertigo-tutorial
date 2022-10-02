use vertigo::{Computed, Value, DomElement, dom, bind, bind2, css, Css};

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

#[derive(Clone)]
pub struct State {
    items: Value<Vec<String>>,
    new_item: Value<String>,
    count: Computed<usize>,
}

impl State {
    pub fn new() -> Self {
        let items = Value::new(vec![
            "Item 1".to_string(),
            "Item 2".to_string(),
        ]);

        let count = {
            let items = items.clone();
            Computed::from(move |ctx| items.get(ctx).len())
        };

        State {
            items,
            new_item: Value::new(String::default()),
            count,
        }
    }

    pub fn render(&self) -> DomElement {
        let add = bind2(&self.items, &self.new_item).call(|ctx, items, new_item| {
            let mut items_vec = items.get(ctx).to_vec();
            items_vec.push(new_item.get(ctx));
            items.set(items_vec);
            new_item.set("".to_string());
        });

        let change = bind(&self.new_item).call_param(|_ctx, new_item, new_value| {
            new_item.set(new_value);
        });

        let elements = self.items.render_list(
            |item| item.clone(),
            |item| {
                let excl = item.ends_with('!');
                dom! {
                    <li css={alternate_rows(excl)}>{item}</li>
                }
            },
        );

        dom! {
            <div>
                <p>"My list (" { &self.count } ")"</p>
                <ol>
                    { elements }
                </ol>
                <input value={self.new_item.to_computed()} on_input={change} />
                <button on_click={add}>"Add"</button>
            </div>
        }
    }
}
