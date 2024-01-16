use minijinja::{Environment, Error};

use crate::frontend::view::IndexHtml;

pub struct WebEnv {
    pub value: Environment<'static>,
}

impl WebEnv {
    pub fn new() -> Result<WebEnv, Error> {
        let mut value: Environment = Environment::new();

        value
            .add_template("index", IndexHtml::HTML)
            .map(|_| Self { value })
    }
}
