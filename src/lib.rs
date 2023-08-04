//! A chat system which asks user to login/signup before messaging

use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::vec;
use std::{
    fs::{self, read_to_string},
    io::Write,
    io::{self, Error, ErrorKind},
    path::Path,
};

pub mod account_modifications;
pub mod login;
pub mod signup;
pub mod user_interface;

/// struct storing all the information about current user
#[derive(Debug, Serialize, Deserialize)]
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

pub struct ExistingData {
    data: Vec<User>,
}

impl Default for ExistingData {
    fn default() -> Self {
        Self::new()
    }
}

impl ExistingData {
    pub fn new() -> ExistingData {
        ExistingData { data: vec![] }
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

        self.data = serde_json::from_str(file.as_str()).expect("There was some problem in data");
    }
    pub fn data(&self) -> &Vec<User> {
        &self.data
    }

    pub fn append_custom_data(&mut self, data: User) -> Result<(), Error> {
        self.data.push(data);
        let mut upstream = String::from("[");
        for user in self.data() {
            let user_json = json!(user).to_string();

            upstream.push_str(&format!("{},\n", user_json))
        }
        let mut upstream = String::from(upstream.trim());
        upstream.pop();
        upstream.push(']');
        fs::write("user_data.json", upstream)?;
        Ok(())
    }

    pub fn change_data(&mut self, old_username: &str, new_user: User) -> Result<(), Error> {
        println!("{}", old_username);
        if let Some(index) = self.data.iter().position(|x| x.username() == old_username) {
            self.data.remove(index);
            self.data.push(new_user);
            let mut upstream = String::from("[");
            for user in self.data() {
                let user_json = json!(user).to_string();

                upstream.push_str(&format!("{},\n", user_json))
            }
            let mut upstream = String::from(upstream.trim());
            upstream.pop();
            upstream.push(']');
            fs::write("user_data.json", upstream)?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "No such user found"))
        }
    }

    pub fn remove_data(&mut self, username: String) -> Result<(), Error> {
        for (index, user) in self.data.iter().enumerate() {
            if user.username() == username {
                self.data.remove(index);
                let mut upstream = String::from("[");
                for user in self.data() {
                    let user_json = json!(user).to_string();
                    upstream.push_str(&format!("{},\n", user_json))
                }
                let mut upstream = String::from(upstream.trim());
                upstream.pop();
                upstream.push(']');
                fs::write("user_data.json", upstream)?;
                break;
            }
        }
        Ok(())
    }
}

/// It is a wrapper function for taking input

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
        password,
    }
}
