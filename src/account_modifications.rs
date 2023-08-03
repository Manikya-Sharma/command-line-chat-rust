use super::{ExistingData, User};
use colored::*;
use std::io;

pub fn change_username(user: &User, data: &mut ExistingData) {
    println!("Please confirm your identity");
    loop {
        let mut pwd = String::new();
        println!("Enter your password :-");
        io::stdin()
            .read_line(&mut pwd)
            .expect("Could not read line");
        pwd = String::from(pwd.trim());
        if pwd == user.password() {
            break;
        } else {
            println!("{}", "Invalid password, permission denied.".red());
            return;
        }
    }
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
    loop {
        let mut us_name = String::new();
        println!("Enter your username to confirm :-");
        io::stdin()
            .read_line(&mut us_name)
            .expect("Could not read line");
        us_name = String::from(us_name.trim());
        if us_name == user.username() {
            break;
        } else {
            println!("{}", "Invalid username, permission denied.".red());
            return;
        }
    }
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
    println!("Please confirm your identity");
    loop {
        let mut pwd = String::new();
        println!("Enter your password :-");
        io::stdin()
            .read_line(&mut pwd)
            .expect("Could not read line");
        pwd = String::from(pwd.trim());
        if pwd == user.password() {
            break;
        } else {
            println!("{}", "Invalid password, permission denied.".red());
            return;
        }
    }
    data.remove_data(String::from(user.username()))
        .expect("Could not delete account");
    println!("{}", "Account deleted successfully".green());
}
