use handlebars::Handlebars;
use serde_derive::Serialize;

#[derive(Debug, Serialize)]
pub struct Data {
    pub company: String,
    pub service: String,
}

pub fn render(md: &str, data: &Data) -> String {
    let handlebars = Handlebars::new();

    return handlebars
        .render_template(&md, &data)
        .expect("could not render template");
}
