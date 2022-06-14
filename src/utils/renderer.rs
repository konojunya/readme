use crate::thirdparty;
use handlebars::Handlebars;
use serde_derive::Deserialize;

use super::markdown::format_data;

#[derive(Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Activity {
    pub date: u32,
    pub title: String,
    pub tag: String,
    pub link: String,
}

#[derive(Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Company {
    pub label: String,
    pub link: String,
}

#[derive(Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Service {
    pub label: String,
    pub link: String,
}

#[derive(Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct SNS {
    pub label: String,
    pub id: String,
    pub link: String,
}

#[derive(Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct CareerHistory {
    pub name: String,
    pub date: String,
}

#[derive(Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Data {
    pub name: String,
    pub description: String,
    pub specialty: Vec<String>,
    pub sns: Vec<SNS>,
    pub career_history: Vec<CareerHistory>,
    pub activities: Vec<Activity>,
}

pub async fn inject_third_party(data: &Data) -> Data {
    let mut activities = data.activities.clone();

    // zenn
    let mut zenn = thirdparty::zenn::get_activities().await;
    activities.append(&mut zenn);

    activities.sort();
    activities.reverse();

    let data = Data{
        name: data.name.clone(),
        description: data.description.clone(),
        specialty: data.specialty.clone(),
        sns: data.sns.clone(),
        career_history: data.career_history.clone(),
        activities,
    };

    data
}

pub fn render(md: &str, data: &Data) -> String {
    let handlebars = Handlebars::new();

    return handlebars
        .render_template(&md, &format_data(&data))
        .expect("could not render template");
}
