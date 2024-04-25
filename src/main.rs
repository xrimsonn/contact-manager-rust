mod contact;
mod utils;
use contact::*;
use utils::run_menu;

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

    run_menu(&mut contacts);
}
