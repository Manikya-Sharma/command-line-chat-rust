use super::{login::ExistingData, user_input, User};
use regex::Regex;
use serde_json::json;
use std::fs;
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
    let re = Regex::new("^[^\\s]+$").unwrap();
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
        return Err(String::from("Username not unique"));
    } else if !check_valid_username(username) {
        return Err(String::from("Invalid username"));
    } else if !check_valid_password(password) {
        return Err(String::from("Invalid password"));
    } else {
        updated_data.append_custom_data(User {
            username: username.to_string(),
            password: password.to_string(),
        });
        let mut upstream = String::from("[");
        for user in updated_data.data() {
            let user_json = json!(user).to_string();

            upstream.push_str(&format!("{},\n", user_json))
        }
        let mut upstream = String::from(upstream.trim());
        upstream.pop();
        upstream.push_str("]");
        fs::write("user_data.json", upstream).expect("Unable to write");
        Ok(user)
    }
}
