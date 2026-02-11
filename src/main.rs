use crate::cli::handle_menu;
mod cli;
mod parser;
mod theory;

// const CHORD_FORMAT: &str = "[Root note] [quality (blank for major)]";

fn main() {
    handle_menu();
}
