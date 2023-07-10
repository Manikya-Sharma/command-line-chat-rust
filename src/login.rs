//! Module to implement login functionality for already signed up users

use super::{user_input, User};
use std::fs::read_to_string;
use std::io::ErrorKind;
use std::path::Path;
use std::{sync::mpsc, thread};

// meant to store data as a cache for threading
pub struct ExistingData {
    data: Vec<(String, String)>,
}

impl ExistingData {
    pub fn new() -> ExistingData {
        ExistingData { data: Vec::new() }
    }
    pub fn update(&mut self, path: &Path) {
        let file = match read_to_string(path) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    // no need to cache
                    return;
                } else {
                    panic!("Could not read file");
                }
            }
        };
        for line in file.lines() {
            match line.split_once("~") {
                Some((username, password)) => {
                    self.data
                        .push((username.trim().to_string(), password.trim().to_string()));
                }
                None => (),
            }
        }
    }
    pub fn data(&self) -> &Vec<(String, String)> {
        &self.data
    }
    pub fn append_custom_data(&mut self, data: (String, String)) {
        self.data.push(data);
    }
}

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
///     let path = Path::new("user_data.csv");
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
    for (existing_username, existing_password) in rx_data.recv().unwrap().data() {
        if username == existing_username && password == existing_password {
            return Ok(user);
        }
    }
    Err(String::from("Invalid username or password"))
}
