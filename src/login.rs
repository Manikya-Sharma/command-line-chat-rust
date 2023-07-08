//! Module to implement login functionality for already signed up users
use super::User;
use rpassword;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;
use std::{sync::mpsc, thread};

// meant to store data as a cache for threading
struct ExistingData {
    data: Vec<(String, String)>,
}

impl ExistingData {
    fn update(&mut self, path: &Path) {
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
    fn data(&self) -> &Vec<(String, String)> {
        &self.data
    }
}

/// It is a wrapper function for taking input
///
/// # Examples: -
///
/// ```
/// use my_app::login::user_login;
/// use my_app::User;
/// fn main() {
///     let my_user = user_login();
///     println!("Username: {}, Password: {}", my_user.username(), my_user.password());
/// }
/// ```
pub fn user_login() -> User {
    let mut username = String::new();
    print!("Username: ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut username)
        .expect("Could not take input, please try again later");
    let username = username.trim();
    let password = rpassword::prompt_password("Password: ").unwrap();
    User {
        username: username.to_string(),
        password: password.to_string(),
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
    let mut data = ExistingData { data: Vec::new() };
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
