use std::fmt::{Display, Formatter};

pub enum TempConversionType {
    CelsiusToFahrenheit,
    FahrenheitToCelsius,
}

impl Display for TempConversionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TempConversionType::CelsiusToFahrenheit => "Celsius to fahrenheit".to_string(),
            TempConversionType::FahrenheitToCelsius => "Fahrenheit to celsius".to_string(),
        };
        write!(f, "{}", str)
    }
}