//! Module to implement login functionality for already signed up users
use super::User;
use std::fs::read_to_string;
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
        for line in read_to_string(path).unwrap().lines() {
            match line.split_once(",") {
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
///     let path = Path::new("user_data.txt");
///     let user = User::new("username", "password");
///     if attempt_login(&user, &path) {
///         println!("Login successful");
///     } else {
///         println!("Could not login");
///     }
/// }
pub fn attempt_login(user: &User, file_path: &'static Path) -> bool {
    let mut data = ExistingData::new();
    let (tx_data, rx_data) = mpsc::channel();
    let load_data = thread::spawn(move || {
        data.update(file_path);
        tx_data.send(data).unwrap();
    });
    let username = user.username();
    let password = user.password();
    load_data.join().unwrap();
    for (existing_username, existing_password) in rx_data.recv().unwrap().data() {
        if username == existing_username && password == existing_password {
            return true;
        }
    }
    false
}
