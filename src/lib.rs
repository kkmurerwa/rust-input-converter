use io_manager::input_getter;
use io_manager::outputter;
use converters::conversion_type::ConversionType;

mod converters;
mod io_manager;

pub fn run() {
    start_program();
}

fn start_program() {
    outputter::show("Welcome to the Rust unit converter. Enter a number to select a converter.");
    let converters = vec![ConversionType::Temperature.to_string(), ConversionType::Distance.to_string()];
    outputter::print_menu_options(&converters);

    let input = input_getter::get_user_input();

    match input.as_str() {
        "1" => converters::start_converter(ConversionType::Temperature),
        "2" => converters::start_converter(ConversionType::Distance),
        _ => {
            outputter::show(format!("Please enter a number between 1 and {}", converters.len()));
            start_program();
        }
    };
}