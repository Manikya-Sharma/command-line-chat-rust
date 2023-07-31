use std::{fs, io::ErrorKind, path::Path};

use super::User;

fn fetch_users(path: &Path) {
    let file = match fs::read_to_string(path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return;
            } else {
                panic!("Could not read file")
            }
        }
    };

    for line in file.lines() {}
}

pub fn change_username(user: &User) {}
pub fn change_password(user: &User) {}
pub fn delete_account(user: &User) {}
