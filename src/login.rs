//! Module to implement login functionality for already signed up users
use super::User;
use rpassword;
use std::fs::read_to_string;
use std::io::{self, Write};
use std::path::Path;

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
pub fn attempt_login(user: &User, file_path: &Path) -> bool {
    let username = user.username();
    let password = user.password();
    let data = match read_to_string(file_path) {
        Ok(data) => data,
        Err(e) => {
            println!("Could not access file: {}", e);
            "invalid".to_string()
        }
    };
    for line in data.lines() {
        let line = line.to_string();
        let (existing_username, existing_password) = match line.split_once(",") {
            Some((username, password)) => (username.trim(), password.trim()),
            None => ("@", "@"), // Demo data, will not be acceptable username, password
        };
        if username == existing_username && password == existing_password {
            return true;
        }
    }
    false
}
