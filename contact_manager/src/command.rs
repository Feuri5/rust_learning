#[derive(PartialEq, Eq)]
pub enum Command {
    Add,
    Remove,
    List,
    Quit,
    Error,
}

pub fn string_to_command(input : String) -> Command {
    match input.as_str().trim() {
        "add" => return Command::Add,
        "remove" => return Command::Remove,
        "list" => return Command::List,
        "quit" => return Command::Quit,
        _ => return Command::Error,   
    };
}

pub fn explain_command() {
    println!("What do you want to do ?");
    println!("Add a contact : add");
    println!("Remove a contact : remove");
    println!("List all contacts : list");
    println!("Quit the app : quit");
}