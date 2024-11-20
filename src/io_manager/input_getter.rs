use std::{io, process};

pub fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap_or_else(|_| {
        println!("Invalid input entered. Exiting program");
        process::exit(1);
    });
    input
}