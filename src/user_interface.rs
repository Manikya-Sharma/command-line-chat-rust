use super::{
    account_modifications::{change_password, change_username, delete_account},
    upload_data_as_json, ExistingData, LoopHandler, User,
};
use colored::*;
use serde_derive::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, ErrorKind, Write},
    path::Path,
};
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
    }

    fn show_received_messages(&self, username: &str) {
        for message in &self.data {
            if username == message.to {
                println!("{} `{}`: -", "->Received from".magenta(), message.from);
                println!("{}\n", message.message.green());
            }
        }
    }

    fn append_data(&mut self, message: Message) {
        self.data.push(message);
    }

    fn upload_data(&self) {
        upload_data_as_json(&self.data, "database.json".to_string()).unwrap();
    }
}

pub fn ui_implement(user: &User) -> LoopHandler {
    // returns continue running and current signed up state
    let (tx_messages, rx_messages) = mpsc::channel();
    let message_handler = thread::spawn(move || {
        let mut messages_store = Messages::new();
        messages_store.fetch_messages(Path::new("database.json"));
        tx_messages
            .send(messages_store)
            .expect("Could not transfer data");
    });

    let (tx_user_data, rx_user_data) = mpsc::channel();
    let user_data_handler = thread::spawn(move || {
        let mut complete_user_data = ExistingData::new();
        complete_user_data.update(Path::new("user_data.json"));
        tx_user_data
            .send(complete_user_data)
            .expect("Could not send data");
    });

    let current_option = menu();
    message_handler.join().expect("Unable to finish fetching");
    user_data_handler
        .join()
        .expect("Could not fetch users data");

    let mut messages_store = rx_messages.recv().expect("Could not accept data");
    let mut users_data = rx_user_data.recv().expect("Could not receive data");

    if current_option == 1 {
        messages_store.show_received_messages(user.username());
    } else if current_option == 2 {
        send_message(user.username(), &mut messages_store, &users_data);
    } else if current_option == 3 {
        return LoopHandler::ReachMain;
    } else if current_option == 4 {
        if show_settings(user, &mut users_data) {
            return LoopHandler::ReachMain;
        }
    } else if current_option == 5 {
        return LoopHandler::BreakMain;
    } else {
        println!("{}", "Please enter a valid option".red());
    }
    LoopHandler::Continue
}

fn send_message(username: &str, messages_store: &mut Messages, data: &ExistingData) {
    let mut to_username = String::new();
    println!("Whom to refer?");
    io::stdin()
        .read_line(&mut to_username)
        .expect("Could not read line");
    let to_username = to_username.trim();

    let mut flag = false;
    for user in data.data() {
        if to_username == user.username() {
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

fn show_settings(user: &User, data: &mut ExistingData) -> bool {
    // returns whether to logout or not
    let mut input = String::new();
    println!("{}", "1. Change username(1)".red());
    println!("{}", "2. Change password(2)".red());
    println!("{}", "3. Delete Account(3)".red());
    println!("{}", "4. Cancel".green());
    print!("=>");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    let input = input.trim();
    if input == "1" {
        change_username(user, data);
    } else if input == "2" {
        change_password(user, data);
    } else if input == "3" {
        delete_account(user, data);
    } else if input == "4" {
        return false;
    } else {
        println!("Please enter a valid option");
        return false;
    }
    true
}

fn menu() -> u8 {
    let mut input = String::new();
    println!("\n1. Show received messages (1)");
    println!("2. New message (2)");
    println!("3. Log out (3)");
    println!("4. Account Settings (4)");
    println!("5. Quit (5)\n");
    print!("=>");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    input.trim().parse().unwrap_or(0)
}
