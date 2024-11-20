use std::{io, process};
use crate::io_manager::outputter;

pub fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap_or_else(|_| {
        outputter::show("Invalid input entered. Exiting program");
        process::exit(1);
    });
    input.trim().to_string()
}