use std::i8::MIN;
use conversion_type::ConversionType;
use crate::converters::distance_converter::distance_conversion_type::DistanceConversionType::{CentimetersToInches, KilometersToMiles, MetersToFeet};
use crate::converters::distance_converter::DistanceConverter;
use crate::converters::temperature_converter::temp_conversion_type::TempConversionType::CelsiusToFahrenheit;
use crate::converters::temperature_converter::temp_conversion_type::TempConversionType::FahrenheitToCelsius;
use crate::converters::temperature_converter::TemperatureConverter;
use crate::io_manager::{input_getter, outputter};

pub mod temperature_converter;
mod distance_converter;
pub mod conversion_type;

pub fn start_converter(conversion_type: ConversionType) {
    match conversion_type {
        ConversionType::Temperature => start_temperature_converter(),
        ConversionType::Distance => start_distance_converter(),
    };
}

fn start_temperature_converter() {
    outputter::show("What temperature converter do you want to use?");
    let temp_converters = vec![CelsiusToFahrenheit.to_string(), FahrenheitToCelsius.to_string()];
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
    let distance_converters = vec![
        MetersToFeet.to_string(), CentimetersToInches.to_string(), KilometersToMiles.to_string(),
    ];
    outputter::print_menu_options(&distance_converters);

    let input = input_getter::get_user_input();

    match input.as_str() {
        "1" => DistanceConverter::convert(MetersToFeet),
        "2" => DistanceConverter::convert(CentimetersToInches),
        "3" => DistanceConverter::convert(KilometersToMiles),
        _ => {
            outputter::show(format!("Please enter a number between 1 and {}", distance_converters.len()));
            start_distance_converter();
        },
    };
}