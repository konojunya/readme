mod constants;
mod utils;

fn main() {
    let base_markdown = utils::file::read_file(constants::BASE_TEMPLATE);
    utils::file::write_file(constants::README_PATH, &base_markdown);
}
