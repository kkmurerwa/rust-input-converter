pub mod temperature_converter;
pub mod base_converter;

use TempConversionType::{CelsiusToFahrenheit, FahrenheitToCelsius};
use crate::converters::temperature_converter::TempConversionType;
use crate::io_manager::{input_getter, outputter};

pub fn start_converter(conversion_type: ConversionType) {
    match conversion_type {
      ConversionType::Temperature => start_temperature_converter(),
    };
}

fn start_temperature_converter() {
    println!("What temperature converter do you want to use?");
    let temp_converters = vec!["Celsius to Fahrenheit", "Fahrenheit to Celsius"];
    outputter::print_menu_options(&temp_converters);

    let input = input_getter::get_user_input();
    let processed_input = input.as_str().trim();

    match processed_input {
        "1" => {
            temperature_converter::convert(CelsiusToFahrenheit);
        },
        "2" => {
            temperature_converter::convert(FahrenheitToCelsius);
        },
        _ => {
            println!("Please enter a number between 1 and {}", temp_converters.len());
            start_temperature_converter();
        },
    };
}

pub enum ConversionType {
    Temperature,
}