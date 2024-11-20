use std::process;
use crate::io_manager::input_getter;

const CONVERSION_FACTOR: f64 = 5f64/9f64;
const ADDITION_FACTOR: f64 = 32f64;

pub fn convert_temperature(temp_conversion_type: TempConversionType) {
    println!("Enter the temperature you want converted");

    let mut input = match input_getter::get_user_input() {
        Ok(value) => value,
        Err(err) => {
            println!("Invalid input entered. Exiting program");
            process::exit(1);
        },
    };

    let processed_input = input.as_str().trim();

    match temp_conversion_type {
        TempConversionType::CelsiusToFahrenheit => {
            let output = celsius_to_fahrenheit(processed_input).to_string();
            println!("{processed_input} celsius is {output} fahrenheit");
        },
        TempConversionType::FahrenheitToCelsius => {
            let output = fahrenheit_to_celsius(processed_input).to_string();
            println!("{processed_input} fahrenheit is {output} celsius");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_from_celsius_to_fahrenheit() {
        let temp = "37";

        assert_eq!(98.6, celsius_to_fahrenheit(&temp));
    }

    #[test]
    fn converts_from_fahrenheit_to_celsius() {
        let temp = "98.6";
        
        assert_eq!(37, fahrenheit_to_celsius(&temp));
    }
}

pub enum TempConversionType {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,
}