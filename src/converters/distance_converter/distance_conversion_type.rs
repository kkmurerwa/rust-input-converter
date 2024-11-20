use std::fmt::{Display, Formatter};

pub enum DistanceConversionType {
    MetersToFeet,
    FeetToMeters,
}

impl Display for DistanceConversionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DistanceConversionType::MetersToFeet => "Meters to feet".to_string(),
            DistanceConversionType::FeetToMeters => "Feet to meters".to_string(),
        };
        write!(f, "{}", str)
    }
}