//! A chat system which asks user to login/signup before messaging

pub mod login;

/// struct storing all the information about current user
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

#[cfg(test)]
mod tests {
    use super::*;
    use login::attempt_login;
    use std::path::Path;
    #[test]
    fn could_login() {
        assert_eq!(
            true,
            attempt_login(&User::new("a", "b"), &Path::new("user_data.txt"))
        );
    }
}
