use super::User;
use std::io;
use std::{fs::read_to_string, io::ErrorKind, path::Path};
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
        let file = match read_to_string(path) {
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
            let data: Vec<&str> = line.split(",").collect();
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
                println!("Received from {from}: -");
                println!("{message}\n");
            }
        }
    }
}

pub fn ui_implement(user: &User) -> bool {
    let (tx_messages, rx_messages) = mpsc::channel();
    let message_handler = thread::spawn(move || {
        let mut messages_store = Messages::new();
        messages_store.fetch_messages(&Path::new("database.csv"));
        tx_messages
            .send(messages_store)
            .expect("Could not transfer data");
    });

    let current_option = menu();
    if current_option == 1 {
        message_handler.join().expect("Unable to finish fetching");
        let messages_store = rx_messages.recv().expect("Could not accept data");
        messages_store.show_received_messages(&user.username());
    } else if current_option == 2 {
        send_message();
    } else if current_option == 3 {
        println!("Your password is {}", user.password());
    } else if current_option == 4 {
        return false;
    } else {
        println!("Please enter a valid option");
    }
    true
}

fn send_message() {}

fn menu() -> u8 {
    let mut input = String::new();
    println!("1. Show received messages (1)");
    println!("2. New message (2)");
    println!("3. Show password (3)");
    println!("4. Quit (4)");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    }
}
