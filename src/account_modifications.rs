use super::{ExistingData, User};
use colored::*;
use std::io;

pub fn change_username(user: &User, data: &mut ExistingData) {
    let mut new_username = String::new();
    'outer: loop {
        new_username.clear();
        println!("Enter your new username");
        io::stdin()
            .read_line(&mut new_username)
            .expect("Could not read");
        let new_username = new_username.trim();
        for existing_user in data.data() {
            if new_username == existing_user.username() {
                println!(
                    "{}",
                    "Sorry, this username is already taken, please try again".red()
                );
                continue 'outer;
            }
        }
        break;
    }
    let new_username = String::from(new_username.trim());
    let new_user = User {
        username: new_username,
        password: String::from(user.password()),
    };
    data.change_data(user.username(), new_user)
        .expect("Unable to update data");
    println!("{}", "Username has been changed successfully!".green());
}

pub fn change_password(user: &User, data: &mut ExistingData) {
    let mut new_password = String::new();
    println!("Enter your new password");
    io::stdin()
        .read_line(&mut new_password)
        .expect("Could not read");
    let new_password = String::from(new_password.trim());
    let new_user = User {
        username: String::from(user.username()),
        password: new_password,
    };
    data.change_data(user.username(), new_user)
        .expect("Unable to update data");
    println!("{}", "Password has been changed successfully!".green());
}

pub fn delete_account(user: &User, data: &mut ExistingData) {
    data.remove_data(String::from(user.username()))
        .expect("Could not delete account");
    println!("{}", "Account deleted successfully".green());
}
