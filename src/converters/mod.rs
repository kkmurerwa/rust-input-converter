pub mod temperature_converter;
mod distance_converter;

use crate::converters::temperature_converter::{TempConversionType, TemperatureConverter};
use crate::io_manager::{input_getter, outputter};
use TempConversionType::{CelsiusToFahrenheit, FahrenheitToCelsius};
use crate::converters::distance_converter::DistanceConversionType::{FeetToMeters, MetersToFeet};
use crate::converters::distance_converter::DistanceConverter;

pub fn start_converter(conversion_type: ConversionType) {
    match conversion_type {
        ConversionType::Temperature => start_temperature_converter(),
        ConversionType::Distance => start_distance_converter(),
    };
}

fn start_temperature_converter() {
    outputter::show("What temperature converter do you want to use?");
    let temp_converters = vec!["Celsius to Fahrenheit", "Fahrenheit to Celsius"];
    outputter::print_menu_options(&temp_converters);

    let input = input_getter::get_user_input();

    match input.as_str() {
        "1" => {
            TemperatureConverter::convert(CelsiusToFahrenheit);
        },
        "2" => {
            TemperatureConverter::convert(FahrenheitToCelsius);
        },
        _ => {
            outputter::show(format!("Please enter a number between 1 and {}", temp_converters.len()));
            start_temperature_converter();
        },
    };
}

fn start_distance_converter() {
    outputter::show("What distance converter do you want to use?");
    let distance_converters = vec!["Meters to feet", "Feet to meters"];
    outputter::print_menu_options(&distance_converters);

    let input = input_getter::get_user_input();

    match input.as_str() {
        "1" => {
            DistanceConverter::convert(MetersToFeet);
        },
        "2" => {
            DistanceConverter::convert(FeetToMeters);
        },
        _ => {
            outputter::show(format!("Please enter a number between 1 and {}", distance_converters.len()));
            start_temperature_converter();
        },
    };
}

pub enum ConversionType {
    Temperature,
    Distance,
}