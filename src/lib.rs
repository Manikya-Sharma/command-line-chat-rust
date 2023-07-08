//! A chat system which asks user to login/signup before messaging

use std::io::{self, Write};
pub mod login;
pub mod signup;
use rpassword;

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
mod tests {
    use super::*;
    use login::attempt_login;
    use signup::user_signup;
    use std::path::Path;
    #[test]
    fn could_login() {
        assert_eq!(
            true,
            attempt_login(&User::new("a", "b"), &Path::new("user_data.txt"))
        );
    }
    #[test]
    fn non_unique_username_signup() {
        assert_eq!(
            Err(String::from("Username not unique")),
            user_signup(&User::new("a", "b"))
        );
    }
    #[test]
    fn invalid_password_signup() {
        assert_eq!(
            Err(String::from("Invalid password")),
            user_signup(&User::new("jon_doe", "@@b"))
        );
    }
    #[test]
    fn invalid_username_signup() {
        assert_eq!(
            Err(String::from("Invalid username")),
            user_signup(&User::new("jon doe", "b"))
        );
    }
}
