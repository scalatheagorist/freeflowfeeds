use minijinja::Environment;

use crate::frontend::view::IndexHtml;

pub struct WebEnv {
    pub value: Environment<'static>
}

impl WebEnv {
    pub fn new() -> Self {
        let mut value: Environment = Environment::new();

        value
            .add_template("index", &*IndexHtml::HTML)
            .expect("could not load page initially");

        Self {
            value
        }
    }
}
