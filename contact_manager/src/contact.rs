use std::io;

#[derive(Clone)]
pub struct Contact {
    pub name : String,
    pub phone : String,
}

pub fn create_contact() -> Contact
{
    let mut name_ : String = String::new();
    println!("Write the contact name");
    let _ = io::stdin().read_line(&mut name_);
    
    let mut phone_ : String = String::new();
    println!("Write the contact phone");
    let _ = io::stdin().read_line(&mut phone_);
    
    let contact : Contact = Contact {
        name : name_.clone(),
        phone : phone_,
    };

    return contact;
}

pub fn get_name() -> String
{
    let mut name_ : String = String::new();
    println!("Write the contact name");
    let _ = io::stdin().read_line(&mut name_);
    return name_;
}

pub fn print_contact(contact : Contact) {
    println!("{} : {}", contact.name, contact.phone);
}