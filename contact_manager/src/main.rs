mod command;
mod contact;
use std::io;
use std::collections::HashMap;

fn main() {
    let mut contact_manager : HashMap<String, contact::Contact> = HashMap::new();

    println!("Welcome to the contact app :");
    let mut comm : command::Command = command::Command::Error;
    while comm != command::Command::Quit
    {
        command::explain_command();
        let mut input : String = String::new();
        let _ = io::stdin().read_line(&mut input);
        comm = command::string_to_command(input);
        match comm {
            command::Command::Add => {
                let contact = contact::create_contact();
                let name = contact.name.clone();
                if (&contact_manager).contains_key(&name) {
                    println!("The contact already exists !");
                } else {
                    contact_manager.insert(name, contact);
                }
            },
            command::Command::Remove => {
                let name = contact::get_name();
                if !contact_manager.contains_key(&name) {
                    println!("This contact does not exists !");
                } else {
                    contact_manager.remove(&name);
                }
            },
            command::Command::List => {
                for (_, contact) in contact_manager.iter() {
                    contact::print_contact(contact.clone());
                }
            },
            command::Command::Quit => {

            }
            _ => {
                println!("Choose a real function");
            },
        }
    }
}


