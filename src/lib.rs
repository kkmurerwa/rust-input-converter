use crate::converters::ConversionType;
use io_manager::input_getter;
use io_manager::outputter;

mod converters;
mod io_manager;

pub fn run() {
    start_program();
}

fn start_program() {
    println!("Welcome to the Rust unit converter. Enter a number to select a converter.");
    let converters = vec!["Temperature converter"];
    outputter::print_menu_options(&converters);

    let input = input_getter::get_user_input();

    match input.as_str().trim() {
        "1" => converters::start_converter(ConversionType::Temperature),
        _ => {
            println!("Please enter a number between 1 and {}", converters.len());
            start_program();
        }
    };
}