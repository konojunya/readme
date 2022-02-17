use std::io::BufReader;

extern crate handlebars;
extern crate serde;

mod constants;
mod utils;

fn write_readme(data_file: &str, file_name: &str) {
    let base_markdown = utils::file::read_file(constants::BASE_TEMPLATE);
    let ja_data = utils::file::read_file(data_file);
    let reader = BufReader::new(ja_data.as_bytes());
    let data = serde_json::from_reader(reader).expect("Failed to parse json");
    let markdown = utils::renderer::render(&base_markdown, &data);

    utils::file::write_file(file_name, &markdown);
}

fn main() {
    write_readme("data/ja.json", "README.md");
    write_readme("data/en.json", "docs/README.en.md");
}
