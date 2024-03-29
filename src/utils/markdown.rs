use crate::utils::renderer::{Activity, CareerHistory, Data, SNS};
use chrono::prelude::*;
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Markdown {
    pub name: String,
    pub description: String,
    pub specialty: String,
    pub sns: String,
    pub what_kind_of_people_do_you_want_to_work_with: String,
    pub field_of_interest: String,
    pub career_history: String,
    pub activities: String,
    pub more_activities: String,
}

fn format_activities(activities: &Vec<Activity>, more: bool) -> String {
    let now = Local::now();
    let year = now.year().to_string();

    let mut result = String::new();
    let mut current_year = "".to_string();
    for activity in activities {
        let activity_date = activity.date.to_string();
        let activity_year = &activity_date[..4];
        let activity_month = &activity_date[4..6];
        let activity_day = &activity_date[6..8];

        // for this year
        if !more && activity_year == year {
            if activity_year != &current_year {
                current_year = activity_year.to_string();
                result.push_str(&format!("\n### {}\n", current_year));
            }
            result.push_str(&format!(
                "- [[{}] {}]({}) ({}/{}/{})\n",
                activity.tag,
                activity.title,
                activity.link,
                activity_year,
                activity_month,
                activity_day
            ));
        }

        // before activity
        if more && activity_year != year {
            if activity_year != &current_year {
                current_year = activity_year.to_string();
                result.push_str(&format!("\n### {}\n", current_year));
            }
            result.push_str(&format!(
                "- [[{}] {}]({}) ({}/{}/{})\n",
                activity.tag,
                activity.title,
                activity.link,
                activity_year,
                activity_month,
                activity_day
            ));
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

fn format_career_history(career_history: &Vec<CareerHistory>) -> String {
    let mut result = String::new();
    for item in career_history {
        result.push_str(&format!("- {} ({})\n", item.name, item.date,));
    }
    result
}

pub fn format_data(data: &Data) -> Markdown {
    return Markdown {
        name: data.name.clone(),
        description: data.description.clone(),
        specialty: format_string_vec(&data.specialty),
        sns: format_sns(&data.sns),
        what_kind_of_people_do_you_want_to_work_with: format_string_vec(
            &data.what_kind_of_people_do_you_want_to_work_with,
        ),
        field_of_interest: format_string_vec(&data.field_of_interest),
        career_history: format_career_history(&data.career_history),
        activities: format_activities(&data.activities, false),
        more_activities: format_activities(&data.activities, true),
    };
}
