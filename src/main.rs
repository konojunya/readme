extern crate handlebars;

mod constants;
mod utils;

// use serde_json::json;

fn main() {
    let base_markdown = utils::file::read_file(constants::BASE_TEMPLATE);
    let data = utils::renderer::Data {
        company: constants::CURRENT_COMPANY.to_owned(),
        service: constants::CURRENT_SERVICE.to_owned(),
    };
    let markdown = utils::renderer::render(&base_markdown, &data);

    utils::file::write_file(constants::README_PATH, &markdown);
}
