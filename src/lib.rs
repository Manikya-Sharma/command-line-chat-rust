//! A chat system which asks user to login/signup before messaging

use rpassword;
use std::io::{self, Write};
pub mod login;
pub mod signup;
pub mod user_interface;

/// struct storing all the information about current user
#[derive(Debug)]
pub struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.username == other.username
    }
}

/// It is a wrapper function for taking input
///
/// # Examples: -
///
/// ```
/// use my_app::user_input;
/// use my_app::User;
/// fn main() {
///     let my_user = user_input();
///     println!("Username: {}, Password: {}", my_user.username(), my_user.password());
/// }
/// ```
pub fn user_input() -> User {
    let mut username = String::new();
    print!("Username: ");
    io::stdout().flush().unwrap();
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

#[cfg(test)]
mod tests {}
