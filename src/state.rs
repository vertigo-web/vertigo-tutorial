use std::cmp::PartialEq;
use vertigo::{Driver, Computed, Value};

#[derive(PartialEq)]
pub struct State {
    driver: Driver,

    pub message: Value<String>,
}

impl State {
    pub fn new(driver: &Driver) -> Computed<State> {
        let state = State {
            driver: driver.clone(),
            message: driver.new_value("Hello world".to_string()),
        };

        driver.new_computed_from(state)
    }
}
