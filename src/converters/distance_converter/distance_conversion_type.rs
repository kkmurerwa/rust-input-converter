use std::fmt::{Display, Formatter};

pub enum DistanceConversionType {
    MetersToFeet,
    CentimetersToInches,
    KilometersToMiles,
}

impl Display for DistanceConversionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DistanceConversionType::MetersToFeet => "Meters ↔ Feet".to_string(),
            DistanceConversionType::CentimetersToInches => "Centimeters ↔ Inches".to_string(),
            DistanceConversionType::KilometersToMiles => "Kilometers ↔ Miles".to_string(),
        };
        write!(f, "{}", str)
    }
}