use std::error::Error;
use rss::Channel;
use crate::renderer::Activity;
use chrono::DateTime;

async fn fetch() -> Result<Channel, Box<dyn Error>> {
    let url = "https://zenn.dev/jj/feed?include_scraps=1&all=1";
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

async fn format_activities(channel: Channel) -> Vec<Activity> {
    let mut activities: Vec<Activity> = Vec::new();
    for item in channel.items {
        let date_str = DateTime::parse_from_rfc2822(&item.pub_date.expect("could not find pub_date")).expect("colud not parse pub_date by rfc2822").format("%Y%m%d").to_string();
        let date: u32 = date_str.parse().expect("could not parse string to u32");

        activities.push(Activity {
            title: item.title.expect("colud not find title"),
            tag: "Zenn".to_owned(),
            link: item.link.expect("could not find link"),
            date,
        });
    };

    activities
}

pub async fn get_activities() -> Vec<Activity> {
    let channel = fetch().await.expect("colud not fetch zenn contents");
    let activities = format_activities(channel).await;

    activities
}
