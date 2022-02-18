use handlebars::Handlebars;
use serde_derive::Deserialize;

use super::markdown::format_data;

#[derive(Deserialize)]
pub struct ActivityItem {
    pub label: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct Activity {
    pub date: String,
    pub items: Vec<ActivityItem>,
}

#[derive(Deserialize)]
pub struct Company {
    pub label: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct Service {
    pub label: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct SNS {
    pub label: String,
    pub id: String,
    pub link: String,
}

#[derive(Deserialize)]
pub struct CareerHistory {
    pub name: String,
    pub date: String,
}

#[derive(Deserialize)]
pub struct Data {
    pub name: String,
    pub description: String,
    pub specialty: Vec<String>,
    pub sns: Vec<SNS>,
    pub company: Company,
    pub service: Service,
    pub career_history: Vec<CareerHistory>,
    pub activities: Vec<Activity>,
}

pub fn render(md: &str, data: &Data) -> String {
    let handlebars = Handlebars::new();

    return handlebars
        .render_template(&md, &format_data(&data))
        .expect("could not render template");
}
