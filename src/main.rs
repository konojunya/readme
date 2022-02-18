use std::io::BufReader;

use utils::renderer;

mod constants;
mod utils;
mod thirdparty;

async fn write_readme(data_file: &str, file_name: &str) {
    let base_markdown = utils::file::read_file(constants::BASE_TEMPLATE);
    let ja_data = utils::file::read_file(data_file);
    let reader = BufReader::new(ja_data.as_bytes());
    let data = serde_json::from_reader(reader).expect("Failed to parse json");

    let injected_data = renderer::inject_third_party(&data).await;
    let activity_sorted_data = renderer::sort_activities(&injected_data);
    let markdown = utils::renderer::render(&base_markdown, activity_sorted_data);

    utils::file::write_file(file_name, &markdown);
}

#[tokio::main]
async fn main() {
    write_readme("data/ja.json", "README.md").await;
}
