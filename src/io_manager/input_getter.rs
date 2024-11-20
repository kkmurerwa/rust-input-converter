use std::io;
use std::io::Error;

pub fn get_user_input() -> Result<String, Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;

    Ok(input)
}