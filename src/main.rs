use colored::*;
use my_app::{login::attempt_login, signup::user_signup, user_interface::ui_implement, User};
use std::{
    io::{self, Write},
    path::Path,
};

fn main() {
    app();
}

fn app() {
    if let Some((w, _)) = term_size::dimensions() {
        println!(
            "\n{:=^width$}",
            "Welcome to the Messaging App!".cyan(),
            width = w
        );
    } else {
        println!("\n{:=^50}", "Welcome to the Messaging App!".cyan());
    }

    println!("{}", "Please enter the option".cyan());
    let user = implement_login_signup_loop();
    if let Some(user) = user {
        // None case should never occur
        let mut run = true;
        println!("{} {}!", "\nWelcome".cyan(), user.username().cyan().bold());

        while run {
            let (now_run, repeat) = ui_implement(&user);
            run = now_run;
            if repeat {
                app();
            }
        }
    }
}

fn show_menu() {
    println!("\n1. Signup (1)");
    println!("2. Login (2)");
    println!("3. Quit (3)\n");
}

fn take_menu_input() -> u8 {
    let mut current_option = String::new();
    print!(">");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut current_option)
        .expect("Could not read your input");
    current_option.trim().parse().unwrap_or(0)
}

fn signup() -> Option<User> {
    match user_signup() {
        Ok(user) => Some(user),
        Err(st) => {
            println!("{} {}", "Error:".red(), st);
            None
        }
    }
}

fn login(path: &'static Path) -> Option<User> {
    match attempt_login(path) {
        Ok(user) => Some(user),
        Err(e) => {
            println!("{}", e.red());
            None
        }
    }
}

fn implement_login_signup_loop() -> Option<User> {
    show_menu();
    let path = Path::new("user_data.json");
    let login_attempts: u8 = 3;

    loop {
        let current_option = take_menu_input();

        if current_option == 1 {
            loop {
                if let Some(user) = signup() {
                    return Some(user);
                }
            }
        } else if current_option == 2 {
            for attempt in 1..=login_attempts {
                match login(path) {
                    Some(user) => {
                        return Some(user);
                    }
                    None => println!(
                        "{} {}",
                        "Wrong username/password, attempts left:".red(),
                        login_attempts - attempt
                    ),
                }
                if attempt == login_attempts {
                    println!("{}", "\nCould not login\n".red());
                }
            }
        } else if current_option == 3 {
            println!("{}", "Good Bye!".green());
            return None;
        } else {
            println!("{}", "Please enter a valid option".red());
        }
    }
}
