use super::account_modifications::{change_password, change_username, delete_account};
use super::{login::ExistingData, User};
use colored::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self};
use std::{fs, io::ErrorKind, path::Path};
use std::{sync::mpsc, thread};

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
    from: String,
    to: String,
}

struct Messages {
    data: Vec<Message>,
}

impl Messages {
    fn new() -> Messages {
        Messages { data: Vec::new() }
    }

    fn fetch_messages(&mut self, path: &Path) {
        let file = match fs::read_to_string(path) {
            Ok(file) => file,
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    return;
                } else {
                    panic!("Could not read file");
                }
            }
        };

        self.data = serde_json::from_str(file.as_str()).expect("There was some problem in data");

        // for line in file.lines() {
        //     let data: Vec<&str> = line.split("~").collect();
        //     if data.len() < 3 {
        //         // Must be an empty line
        //         continue;
        //     }
        //     let from = data[0];
        //     let to = data[1];
        //     let message = data[2];
        //     self.data.push(Message {
        //         from: from.trim().to_string(),
        //         to: to.trim().to_string(),
        //         message: message.trim().to_string(),
        //     });
        // }
    }

    fn show_received_messages(&self, username: &str) {
        for message in &self.data {
            if username == message.to {
                println!("{} `{}`: -", "->Recieved from".magenta(), message.from);
                println!("{}\n", message.message.green());
            }
        }
    }

    fn append_data(&mut self, message: Message) {
        self.data.push(message);
    }

    fn upload_data(&self) {
        let mut upstream = String::from("[");
        for message in &self.data {
            let json_message = json!(message);

            upstream.push_str(&format!("{},\n", json_message.to_string()))
        }
        let mut upstream = String::from(upstream.trim());
        upstream.pop(); // remove trailing comma
        upstream.push_str("]");
        fs::write("database.json", upstream).expect("Unable to write");
    }
}

pub fn ui_implement(user: &User) -> (bool, bool) {
    // returns continue running and current signed up state
    let (tx_messages, rx_messages) = mpsc::channel();
    let message_handler = thread::spawn(move || {
        let mut messages_store = Messages::new();
        messages_store.fetch_messages(&Path::new("database.json"));
        tx_messages
            .send(messages_store)
            .expect("Could not transfer data");
    });

    let current_option = menu();
    message_handler.join().expect("Unable to finish fetching");
    let mut messages_store = rx_messages.recv().expect("Could not accept data");

    if current_option == 1 {
        messages_store.show_received_messages(&user.username());
    } else if current_option == 2 {
        send_message(&user.username(), &mut messages_store);
    } else if current_option == 3 {
        println!("Your password is {}", user.password());
    } else if current_option == 4 {
        return (false, true);
    } else if current_option == 5 {
        show_settings(&user);
    } else if current_option == 6 {
        return (false, false);
    } else {
        println!("{}", "Please enter a valid option".red());
    }
    (true, false)
}

fn send_message(username: &str, messages_store: &mut Messages) {
    let (tx_user_data, rx_user_data) = mpsc::channel();

    let user_data_handler = thread::spawn(move || {
        let mut complete_user_data = ExistingData::new();
        complete_user_data.update(&Path::new("user_data.json"));
        tx_user_data
            .send(complete_user_data)
            .expect("Could not send data");
    });

    let mut to_username = String::new();
    println!("Whom to refer?");
    io::stdin()
        .read_line(&mut to_username)
        .expect("Could not read line");
    let to_username = to_username.trim();

    user_data_handler.join().expect("Could not fetch data");
    let data = rx_user_data.recv().expect("Could not receive data");
    let mut flag = false;
    for user in data.data() {
        if &to_username == &user.username() {
            flag = true;
            break;
        }
    }
    if !flag {
        println!("{}", "No such user found".red());
    } else {
        let mut message = String::new();
        println!("Please enter the message(press enter to stop typing):-");
        io::stdin()
            .read_line(&mut message)
            .expect("Could not read line");
        let message = message.trim();
        messages_store.append_data(Message {
            from: username.to_string(),
            to: to_username.to_string(),
            message: message.to_string(),
        });
        messages_store.upload_data();
    }
}

fn show_settings(user: &User) {
    let mut input = String::new();
    println!("{}", "1. Change username(1)".red());
    println!("{}", "2. Change password(2)".red());
    println!("{}", "3. Delete Account(3)".red());
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    let input = input.trim();
    if input == String::from("1") {
        change_username(&user);
    } else if input == String::from("2") {
        change_password(&user);
    } else if input == String::from("3") {
        delete_account(&user);
    } else {
        println!("Please enter a valid option");
    }
}

fn menu() -> u8 {
    let mut input = String::new();
    println!("{}", "\n1. Show received messages (1)");
    println!("{}", "2. New message (2)");
    println!("{}", "3. Show password (3)");
    println!("{}", "4. Log out (4)");
    println!("{}", "5. Account Settings (5)");
    println!("{}", "6. Quit (6)\n");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
