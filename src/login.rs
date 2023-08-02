//! Module to implement login functionality for already signed up users

use super::{user_input, ExistingData, User};

use std::path::Path;
use std::{sync::mpsc, thread};

// meant to store data as a cache for threading

/// Reads data from data file and tells if user is present or not
///
/// Format of input file :-
///
/// {{username}}, {{password}}
///
/// {{username}}, {{password}}
///
/// ...
///
/// # Examples: -
///
/// ```
/// use std::path::Path;
/// use my_app::{User, login::attempt_login};
/// fn main() {
///     let path = Path::new("user_data.json");
///     if attempt_login(&path) {
///         println!("Login successful");
///     } else {
///         println!("Could not login");
///     }
/// }
pub fn attempt_login(file_path: &'static Path) -> Result<User, String> {
    let mut data = ExistingData::new();
    let (tx_data, rx_data) = mpsc::channel();
    let load_data = thread::spawn(move || {
        data.update(file_path);
        tx_data.send(data).unwrap();
    });

    let user = user_input();
    let username = user.username();
    let password = user.password();

    load_data.join().unwrap();
    for existing_user in rx_data.recv().unwrap().data() {
        if username == existing_user.username && password == existing_user.password {
            return Ok(user);
        }
    }
    Err(String::from("Invalid username or password"))
}
