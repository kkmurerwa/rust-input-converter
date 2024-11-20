use std::process;
use io_manager::input_getter;
use io_manager::outputter;
use crate::converters::ConversionType;

mod converters;
mod io_manager;

pub fn run() {
    start_program();
}

fn start_program() {
    println!("Welcome to the Rust unit converter. Enter a number to select a converter.");
    let converters = vec!["Temperature converter"];
    outputter::print_menu_options(converters);

    let mut input = match input_getter::get_user_input() {
        Ok(value) => value,
        Err(err) => {
            println!("Invalid input entered. Exiting program");
            process::exit(1);
        },
    };

    match input.as_str().trim() {
        "1" => converters::start_converter(ConversionType::Temperature),
        _ => {
            println!("Please enter a valid input");
        }
    };
}