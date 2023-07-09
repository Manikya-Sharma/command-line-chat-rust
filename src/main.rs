use my_app::{login::attempt_login, signup::user_signup, user_interface::ui_implement, User};
use std::io::{self, Write};
use std::path::Path;

fn main() {
    let path = Path::new("user_data.csv");
    let login_attempts: u8 = 3;
    let current_user: Option<User>;

    println!("Welcome to the Messaging App!");
    println!("Please enter the option");

    'login_signup: loop {
        show_menu();
        let current_option = take_menu_input();

        if current_option == 1 {
            loop {
                match signup() {
                    Some(user) => {
                        current_user = Some(user);
                        break 'login_signup;
                    }
                    None => (),
                }
            }
        } else if current_option == 2 {
            for attempt in 1..=login_attempts {
                match login(&path) {
                    Some(user) => {
                        current_user = Some(user);
                        break 'login_signup;
                    }
                    None => println!(
                        "Wrong username/password, attempts left: {}",
                        login_attempts - attempt
                    ),
                }
            }
        } else if current_option == 3 {
            println!("Good Bye!");
            return;
        } else {
            println!("Please enter a valid option");
        }
    }
    // None case should never occur
    if let Some(user) = current_user {
        let mut run = true;
        println!("Welcome {}", user.username());
        while run {
            run = ui_implement(&user);
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

fn signup() -> Option<User> {
    match user_signup() {
        Ok(user) => {
            return Some(user);
        }
        Err(st) => {
            println!("Error: {}", st);
            return None;
        }
    }
}

fn login(path: &'static Path) -> Option<User> {
    match attempt_login(&path) {
        Ok(user) => {
            return Some(user);
        }
        Err(e) => {
            println!("{}", e);
            return None;
        }
    }
}
