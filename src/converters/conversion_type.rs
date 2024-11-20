use std::fmt::Display;

pub enum ConversionType {
    Temperature,
    Distance,
}

impl Display for ConversionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ConversionType::Temperature => "Temperature converter".to_string(),
            ConversionType::Distance => "Distance converter".to_string(),
        };
        write!(f, "{}", str)
    }
}