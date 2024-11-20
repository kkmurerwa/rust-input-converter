pub fn print_menu_options(items: &Vec<&str>) {
    for (index, item) in items.iter().enumerate() {
        println!("{}. {item}", index + 1);
    }
}