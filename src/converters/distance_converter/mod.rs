pub mod distance_conversion_type;

use crate::converters::distance_converter::distance_conversion_type::DistanceConversionType;
use crate::io_manager::{input_getter, outputter};

const METERS_FEET_FACTOR: f64 = 3.28084;
const INCHES_TO_CM_FACTOR: f64 = 2.54;
const KM_TO_MILES_FACTOR: f64 = 0.621371;

pub struct DistanceConverter;

impl DistanceConverter {
    pub(crate) fn convert(distance_conversion_type: DistanceConversionType) {
        outputter::show("Enter the distance you want converted");
        let input = input_getter::get_user_input();

        match distance_conversion_type {
            DistanceConversionType::MetersToFeet => {
                Self::process_meter_feet_conversion(&input);
            },
            DistanceConversionType::CentimetersToInches => {
                Self::process_cm_inches_conversion(&input);
            },
            DistanceConversionType::KilometersToMiles => {
                Self::process_kilometers_miles_conversion(&input);
            }
        }
    }

    fn process_meter_feet_conversion(input: &str) {
        if input.to_lowercase().contains("m") {
            let cleaned_input = input.replace("m", "");
            let output = DistanceConverter::meters_to_feet(&cleaned_input).to_string();
            outputter::show(format!("{cleaned_input} meter(s) is {output} feet"));
        } else if input.to_lowercase().contains("ft") {
            let cleaned_input = input.replace("ft", "");
            let output = DistanceConverter::feet_to_meters(&cleaned_input).to_string();
            outputter::show(format!("{cleaned_input} feet is {output} meter(s)"));
        } else {
            outputter::show("Please enter a valid input");
            return;
        }
    }

    fn process_cm_inches_conversion(input: &str) {
        if input.to_lowercase().contains("cm") {
            let cleaned_input = input.replace("cm", "");
            let output = DistanceConverter::centimeters_to_inches(&cleaned_input).to_string();
            outputter::show(format!("{cleaned_input} centimeter(s) is {output} inch(es)"));
        } else if input.to_lowercase().contains("in") {
            let cleaned_input = input.replace("in", "");
            let output = DistanceConverter::inches_to_centimeters(&cleaned_input).to_string();
            outputter::show(format!("{cleaned_input} inch(es) is {output} centimeter(s)"));
        } else {
            outputter::show("Please enter a valid input");
            return;
        }
    }

    fn process_kilometers_miles_conversion(input: &str) {
        if input.to_lowercase().contains("km") {
            let cleaned_input = input.replace("km", "");
            let output = DistanceConverter::centimeters_to_inches(&cleaned_input).to_string();
            outputter::show(format!("{cleaned_input} kilometer(s) is {output} mile(s)"));
        } else if input.to_lowercase().contains("miles") {
            let cleaned_input = input.replace("miles", "");
            let output = DistanceConverter::inches_to_centimeters(&cleaned_input).to_string();
            outputter::show(format!("{cleaned_input} mile(s) is {output} kilometer(s)"));
        } else {
            outputter::show("Please enter a valid input");
            return;
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

    fn centimeters_to_inches(distance_str: &str) -> f64 {
        let distance: f64 = distance_str.parse().unwrap();
        distance / INCHES_TO_CM_FACTOR
    }

    fn inches_to_centimeters(distance_str: &str) -> f64 {
        let distance: f64 = distance_str.parse().unwrap();
        distance * INCHES_TO_CM_FACTOR
    }

    fn kilometers_to_miles(distance_str: &str) -> f64 {
        let distance: f64 = distance_str.parse().unwrap();
        distance * KM_TO_MILES_FACTOR
    }

    fn miles_to_kilometers(distance_str: &str) -> f64 {
        let distance: f64 = distance_str.parse().unwrap();
        distance / KM_TO_MILES_FACTOR
    }
}
