use std::collections::BTreeMap;

use crate::utils::renderer::{Activity, Data, SNS};
use handlebars::Handlebars;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Markdown {
    pub name: String,
    pub description: String,
    pub specialty: String,
    pub sns: String,
    pub activities: String,
}

fn format_activities(activities: &Vec<Activity>) -> String {
    let mut result = String::new();
    for activity in activities {
        result.push_str(&format!("\n### {}\n", activity.date));
        for item in &activity.items {
            result.push_str(&format!("- [{}]({})\n", item.label, item.link));
        }
    }
    result
}

fn format_label_and_link(label: &str, link: &str) -> String {
    format!("[{}]({})", label, link)
}

fn format_string_vec(vec: &Vec<String>) -> String {
    let mut result = String::new();
    for item in vec {
        result.push_str(&format!("- {}\n", item));
    }
    result
}

fn format_sns(sns: &Vec<SNS>) -> String {
    let mut result = String::new();
    for item in sns {
        result.push_str(&format!(
            "- {}: {}\n",
            item.label,
            format_label_and_link(&format!("@{}", item.id), &format!("{}", item.link))
        ));
    }
    result
}

pub fn format_data(data: &Data) -> Markdown {
    let handlebars = Handlebars::new();
    let mut description = BTreeMap::new();

    description.insert(
        "company",
        format_label_and_link(&data.company.label, &data.company.link),
    );
    description.insert(
        "service",
        format_label_and_link(&data.service.label, &data.service.link),
    );

    return Markdown {
        name: data.name.clone(),
        description: handlebars
            .render_template(&data.description.clone(), &description)
            .expect("cloud not render description"),
        specialty: format_string_vec(&data.specialty),
        sns: format_sns(&data.sns),
        activities: format_activities(&data.activities),
    };
}
