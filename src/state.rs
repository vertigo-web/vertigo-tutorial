use vertigo::{Value, DomElement};

use crate::render::render;
use crate::list;

#[derive(Clone)]
pub struct State {
    pub message: Value<String>,
    pub strong: Value<bool>,
    pub list: list::State,
}

impl State {
    pub fn component() -> DomElement {
        let state = State {
            message: Value::new("Hello world".to_string()),
            strong: Value::new(true),
            list: list::State::new(),
        };

        render(&state)
    }
}
