//! A chat system which asks user to login/signup before messaging

pub mod login;

#[cfg(test)]
mod tests {
    use super::*;
    use login::attempt_login;
    use std::path::Path;
    #[test]
    fn could_login() {
        assert_eq!(
            true,
            attempt_login("Manikya", "Manikya@123", Path::new("user_data.txt"))
        );
    }
}
