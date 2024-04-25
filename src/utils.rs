use crate::contact::*;
use crate::Contact;
use std::io::{self, Write};
use std::process::Command;

fn clear_terminal() {
    let _ = Command::new("clear").status();
}

fn read_num_input(msg: &str) -> i8 {
    let mut var = String::new();
    print!("{}: ", msg);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line.");
    var.trim()
        .parse::<i8>()
        .expect("Failed to parse to number.")
}

pub fn read_string_input(msg: &str) -> String {
    let mut var = String::new();
    print!("{}: ", msg);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line.");
    var.trim().to_string()
}

pub fn run_menu(contacts: &mut Vec<Contact>) {
    loop {
        println!("-------Menu-------");
        println!("1. Contact list");
        println!("2. Add contact");
        println!("3. Search contact");
        println!("4. Delete contact");
        println!("0. Exit");
        let opt = read_num_input("Enter an option");
        clear_terminal();

        match opt {
            1 => print_contact_list(contacts),
            2 => add_contact(contacts),
            3 => search_contact(contacts),
            4 => delete_contact(contacts),
            0 | _ => std::process::exit(0),
        };
    }
}
