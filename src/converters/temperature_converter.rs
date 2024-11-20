use crate::io_manager::{input_getter, outputter};

const CONVERSION_FACTOR: f64 = 5f64/9f64;
const ADDITION_FACTOR: f64 = 32f64;

pub(crate) struct TemperatureConverter;

impl TemperatureConverter {
    pub(crate) fn convert(temp_conversion_type: TempConversionType) {
        outputter::show("Enter the temperature you want converted");

        let input = input_getter::get_user_input();

        match temp_conversion_type {
            TempConversionType::CelsiusToFahrenheit => {
                let output = TemperatureConverter::celsius_to_fahrenheit(&input).to_string();
                outputter::show(format!("{input} celsius is {output} fahrenheit"));
            },
            TempConversionType::FahrenheitToCelsius => {
                let output = TemperatureConverter::fahrenheit_to_celsius(&input).to_string();
                outputter::show(format!("{input} fahrenheit is {output} celsius"));
            },
        };
    }

    fn celsius_to_fahrenheit(temp_str: &str) -> f64 {
        let temp: f64 = temp_str.parse().unwrap();
        temp / CONVERSION_FACTOR + ADDITION_FACTOR
    }

    fn fahrenheit_to_celsius(temp_str: &str) -> i32 {
        let temp: f64 = temp_str.parse().unwrap();
        (CONVERSION_FACTOR * (temp - ADDITION_FACTOR)) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_from_celsius_to_fahrenheit() {
        let temp = "37";
        assert_eq!(98.6, TemperatureConverter::celsius_to_fahrenheit(&temp));
    }

    #[test]
    fn converts_from_fahrenheit_to_celsius() {
        let temp = "98.6";
        assert_eq!(37, TemperatureConverter::fahrenheit_to_celsius(&temp));
    }
}

pub(crate) enum TempConversionType {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,
}