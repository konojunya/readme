use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
struct GitHubActivity {
    pub id: String,
    pub r#type: String,
    pub payload: serde_json::Value,
}

pub async fn fetch() -> Result<(), ()> {
    let client = reqwest::Client::builder().user_agent("konojunya/readme").build().expect("could not build reqwest client");

    let url = "https://api.github.com/users/konojunya/events/public";
    let content = client.get(format!("{}{}", url, "?per_page=100")).send().await.expect("could not fetch from github api").json::<Vec<GitHubActivity>>().await.expect("could not parse GitHubActivity");
    let pull_request_activities = content.iter().filter(|activity| activity.r#type == "PullRequestEvent" && activity.payload.get("action").expect("colud not find action field") == "opened" && activity.payload.get("pull_request").expect("could not find pull_request field").get("base").expect("could not find base field").get("repo").expect("could not find repo field").get("stargazers_count").expect("could not find stargazers_count field").as_i64().expect("could not to i64") > 100).collect::<Vec<&GitHubActivity>>();

    println!("{}", pull_request_activities.len());

    for item in pull_request_activities {
        println!("{:#?}", item);
    }

    Ok(())
}
