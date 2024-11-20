use crate::io_manager::{input_getter, outputter};

const METERS_FEET_FACTOR: f64 = 3.28084;

pub struct DistanceConverter;

impl DistanceConverter {
    pub(crate) fn convert(distance_conversion_type: DistanceConversionType) {
        outputter::show("Enter the distance you want converted");
        let input = input_getter::get_user_input();

        match distance_conversion_type {
            DistanceConversionType::MetersToFeet => {
                let output = DistanceConverter::meters_to_feet(&input).to_string();
                outputter::show(format!("{input} meter(s) is {output} feet"));
            },
            DistanceConversionType::FeetToMeters => {
                let output = DistanceConverter::feet_to_meters(&input).to_string();
                outputter::show(format!("{input} feet is {output} meter(s)"));
            }
        }
    }

    fn meters_to_feet(distance_str: &str) -> f64 {
        let distance: f64 = distance_str.parse().unwrap();
        distance * METERS_FEET_FACTOR
    }

    fn feet_to_meters(distance_str: &str) -> f64 {
        let distance: f64 = distance_str.parse().unwrap();
        distance / METERS_FEET_FACTOR
    }
}

pub(crate) enum DistanceConversionType {
    MetersToFeet,
    FeetToMeters,
}