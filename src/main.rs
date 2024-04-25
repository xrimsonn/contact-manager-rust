use std::io::{self, Write};

struct Contact {
    name: String,
    phone_number: String,
    address: String,
}

fn main() {
    let antonio = Contact {
        name: String::from("Antonio Rosales"),
        phone_number: String::from("614-607-5690"),
        address: String::from("Mina el Potosi 125"),
    };

    let esau = Contact {
        name: String::from("Esau Melendez"),
        phone_number: String::from("614-255-2356"),
        address: String::from("Mina Portillo 1830"),
    };

    let mut contacts: Vec<Contact> = Vec::new();
    contacts.push(antonio);
    contacts.push(esau);

    add_contact(&mut contacts);
    search_contact(&mut contacts);
    print_contact_list(&mut contacts); }

fn read_string_input(msg: &str) -> String {
    let mut var = String::new();
    print!("{}: ", msg);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line.");
    var.trim().to_string()
}

fn print_contact(contact: &Contact) {
    let contact_info = format!(
        "------------\n{}\n{}\n{}",
        contact.name, contact.phone_number, contact.address
    );
    println!("{}", contact_info);
}

fn print_contact_list(contacts: &mut Vec<Contact>) {
    for contact in contacts.iter() {
        print_contact(contact);
    }
}

fn add_contact(contacts: &mut Vec<Contact>) {
    let new_contact = Contact {
        name: read_string_input("Enter the contact's name"),
        phone_number: read_string_input("Enter the contact's phone number"),
        address: read_string_input("Enter the contact's address"),
    };
    contacts.push(new_contact);
}

fn search_contact(contacts: &mut Vec<Contact>) {
    let search_input = read_string_input("Enter the contact's name to search").to_lowercase();
    let mut not_found = true;
    for contact in contacts.iter() {
        if contact.name.to_lowercase().contains(&search_input) {
            print_contact(contact);
            not_found = false;
        }
    }
    if not_found {
        println!("No results for {}.", search_input);
    }
    print!("\n");
}
