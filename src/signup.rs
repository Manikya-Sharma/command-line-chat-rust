use super::{user_input, ExistingData, User};
use regex::Regex;
use std::path::Path;
use std::{sync::mpsc, thread};

fn check_uniqueness(username: &str, data: &ExistingData) -> bool {
    for existing_user in data.data() {
        if existing_user.username() == username {
            return false;
        }
    }
    true
}

fn check_valid_username(username: &str) -> bool {
    // Minimum eight characters, at least one uppercase letter, one lowercase letter and one number:
    let re = Regex::new("^[a-zA-Z0-9]+$").unwrap();
    re.is_match(username)
}

fn check_valid_password(password: &str) -> bool {
    // Minimum eight characters, at least one uppercase letter, one lowercase letter and one number:
    let re = Regex::new("^[^\\s]+$").unwrap(); // will never fail because hard-coded
    re.is_match(password)
}

pub fn user_signup() -> Result<User, String> {
    // caching existing user data
    let mut data = ExistingData::new();
    let (tx_data, rx_data) = mpsc::channel();
    let load_data = thread::spawn(move || {
        data.update(Path::new("user_data.json"));
        tx_data.send(data).unwrap();
    });
    // take user input
    let user = user_input();
    let username = user.username();
    let password = user.password();
    // checking validity
    load_data.join().unwrap();
    let mut updated_data = rx_data.recv().unwrap();
    if !check_uniqueness(username, &updated_data) {
        Err(String::from("Username not unique"))
    } else if !check_valid_username(username) {
        Err(String::from("Invalid username"))
    } else if !check_valid_password(password) {
        Err(String::from("Invalid password"))
    } else {
        updated_data
            .append_custom_data(User {
                username: username.to_string(),
                password: password.to_string(),
            })
            .expect("Could not signup");
        Ok(user)
    }
}
