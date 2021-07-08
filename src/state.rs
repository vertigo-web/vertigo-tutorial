use std::cmp::PartialEq;
use vertigo::computed::{Computed, Dependencies, Value};

#[derive(PartialEq)]
pub struct State {
    root: Dependencies,

    pub message: Value<String>,
}

impl State {
    pub fn new(root: &Dependencies) -> Computed<State> {
        let state = State {
            root: root.clone(),
            message: root.new_value("Hello world".to_string()),
        };

        root.new_computed_from(state)
    }
}
