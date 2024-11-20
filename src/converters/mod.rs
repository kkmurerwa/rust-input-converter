pub mod temperature_converter;

use std::process;
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
    outputter::print_menu_options(temp_converters);

    let mut input = match input_getter::get_user_input() {
        Ok(value) => value,
        Err(err) => {
            println!("Invalid input entered. Exiting program");
            process::exit(1);
        },
    };

    let processed_input = input.as_str().trim();

    match processed_input {
        "1" => {
            temperature_converter::convert_temperature(TempConversionType::CelsiusToFahrenheit);
        },
        "2" => {
            temperature_converter::convert_temperature(TempConversionType::FahrenheitToCelsius);
        },
        _ => {
            println!("Unable to convert your value");
            process::exit(1);
        },
    };
}

pub enum ConversionType {
    Temperature,
}