use my_app::login::{attempt_login, user_login};
use std::path::Path;

fn main() {
    println!("Welcome to the Messaging App!\nPlease login to continue");
    let path = Path::new("user_data.txt");

    let login_attempts = 3;

    for attempt in 1..=login_attempts {
        let (username, password) = user_login();
        if attempt_login(&username, &password, &path) {
            println!("Logged in successfully");
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
