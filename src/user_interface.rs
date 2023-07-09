use super::User;
use std::io;

pub fn ui_implement(user: &User) -> bool {
    let current_option = menu();
    if current_option == 1 {
        fetch_messages();
        show_messages();
    } else if current_option == 2 {
        send_message();
    } else if current_option == 3 {
        println!("Your password is {}", user.password());
    } else if current_option == 4 {
        return false;
    } else {
        println!("Please enter a valid option");
    }
    true
}

fn fetch_messages() {}

fn show_messages() {}

fn send_message() {}

fn menu() -> u8 {
    let mut input = String::new();
    println!("1. Show messages (1)");
    println!("2. New message (2)");
    println!("2. Show password (2)");
    println!("3. Quit (3)");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
