use super::{login::ExistingData, User};
use std::io::{self};
use std::{fs, io::ErrorKind, path::Path};
use std::{sync::mpsc, thread};

struct Messages {
    message_data: Vec<(String, String, String)>,
}

impl Messages {
    fn new() -> Messages {
        Messages {
            message_data: Vec::new(),
        }
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
        for line in file.lines() {
            let data: Vec<&str> = line.split("~").collect();
            if data.len() < 3 {
                // Must be an empty line
                continue;
            }
            let from = data[0];
            let to = data[1];
            let message = data[2];
            self.message_data.push((
                from.trim().to_string(),
                to.trim().to_string(),
                message.trim().to_string(),
            ));
        }
    }

    fn show_received_messages(&self, username: &str) {
        for (from, to, message) in &self.message_data {
            if username == to {
                println!("Received from `{from}`: -");
                println!("{message}\n");
            }
        }
    }

    fn append_data(&mut self, from: String, to: String, message: String) {
        self.message_data.push((from, to, message));
    }

    fn upload_data(&self) {
        let mut upstream = String::new();
        for (from, to, message) in &self.message_data {
            upstream.push_str(&format!("{}~{}~{}\n", from, to, message))
        }
        fs::write("database.csv", upstream).expect("Unable to write");
    }
}

pub fn ui_implement(user: &User) -> (bool, bool) {
    let (tx_messages, rx_messages) = mpsc::channel();
    let message_handler = thread::spawn(move || {
        let mut messages_store = Messages::new();
        messages_store.fetch_messages(&Path::new("database.csv"));
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
        return (false, false);
    } else {
        println!("Please enter a valid option");
    }
    (true, false)
}

fn send_message(username: &str, messages_store: &mut Messages) {
    let (tx_user_data, rx_user_data) = mpsc::channel();

    let user_data_handler = thread::spawn(move || {
        let mut complete_user_data = ExistingData::new();
        complete_user_data.update(&Path::new("user_data.csv"));
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
    for (username, _) in data.data() {
        if &to_username == username {
            flag = true;
            break;
        }
    }
    if !flag {
        println!("No such user found")
    } else {
        let mut message = String::new();
        println!("Please enter the message(press enter to stop typing):-");
        io::stdin()
            .read_line(&mut message)
            .expect("Could not read line");
        let message = message.trim();
        messages_store.append_data(
            username.to_string(),
            to_username.to_string(),
            message.to_string(),
        );
        messages_store.upload_data();
    }
}

fn menu() -> u8 {
    let mut input = String::new();
    println!("1. Show received messages (1)");
    println!("2. New message (2)");
    println!("3. Show password (3)");
    println!("4. Log out (4)");
    println!("5. Quit (5)");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
