use my_app::{login::attempt_login, signup::user_signup};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let path = Path::new("user_data.txt");
    let login_attempts: u8 = 3;

    println!("Welcome to the Messaging App!");
    println!("Please enter the option");

    loop {
        show_menu();
        let current_option = take_menu_input();

        if current_option == 1 {
            signup();
        } else if current_option == 2 {
            login(login_attempts, &path);
        } else if current_option == 3 {
            println!("Good Bye!");
            break;
        } else {
            println!("Please enter a valid option");
        }
    }
}

fn show_menu() {
    println!("1. Signup (1)");
    println!("2. Login (2)");
    println!("3. Quit (3)");
}

fn take_menu_input() -> u8 {
    let mut current_option = String::new();
    print!(">");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut current_option)
        .expect("Could not read your input");
    match current_option.trim().parse() {
        Ok(num) => num,
        // avoid panic
        Err(_) => 0,
    }
}

fn signup() {
    loop {
        match user_signup() {
            Err(st) => {
                println!("Error: {}", st);
                continue;
            }
            _ => (),
        }
        break;
    }
}

fn login(login_attempts: u8, path: &'static Path) {
    for attempt in 1..=login_attempts {
        if attempt_login(&path) {
            println!("Logged in successfully");
            break;
        } else {
            println!(
                "Login unsuccessful, {} attempts left",
                login_attempts - attempt
            );
            if attempt == login_attempts {
                println!("Please try again later");
            }
        }
    }
}
