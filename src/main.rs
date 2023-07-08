use my_app::{login::attempt_login, signup::user_signup};
use my_app::{user_input, User};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let path = Path::new("user_data.txt");
    let login_attempts = 3;
    let mut current_option = String::new();
    let mut current_user: User;
    println!("Welcome to the Messaging App!");
    println!("Please enter the option");

    loop {
        println!("1. Signup (1)");
        println!("2. Login (2)");
        println!("3. Quit (3)");

        current_option.clear();
        print!(">");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut current_option)
            .expect("Could not read your input");
        current_option = current_option.trim().to_string();

        if current_option == String::from("1") {
            loop {
                current_user = user_input();
                match user_signup(&current_user) {
                    Err(st) => {
                        println!("Error: {}", st);
                        continue;
                    }
                    _ => (),
                }
                break;
            }
        } else if current_option == String::from("2") {
            for attempt in 1..=login_attempts {
                current_user = user_input();
                if attempt_login(&current_user, &path) {
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
        } else if current_option == String::from("3") {
            println!("Good Bye!");
            break;
        } else {
            println!("Please enter a valid option");
        }
    }
}
