pub fn print_menu_options(items: &Vec<String>) {
    for (index, item) in items.iter().enumerate() {
        println!("{}. {item}", index + 1);
    }
}

pub fn show(output: impl Into<String>) {
    let output: String = output.into();
    println!("{}", output);
}