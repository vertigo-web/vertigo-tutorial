use vertigo::{Value, DomElement};

use crate::render::render;

#[derive(Clone)]
pub struct State {
    pub message: Value<String>,
}

impl State {
    pub fn component() -> DomElement {
        let state = State {
            message: Value::new("Hello world".to_string()),
        };

        render(&state)
    }
}
