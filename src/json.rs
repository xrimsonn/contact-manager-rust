use crate::Contact;
use std::fs::{self, File};

pub fn load_contacts_json(contacts: &mut Vec<Contact>) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("contacts.json")?;
    let mut new_contacts: Vec<Contact> = serde_json::from_reader(file)?;
    contacts.append(&mut new_contacts);
    Ok(())
}

pub fn save_contacts_json(contacts: &mut Vec<Contact>) -> Result<(), Box<dyn std::error::Error>> {
    let serialized_contacts = serde_json::to_string_pretty(&contacts)?;
    fs::write("contacts.json", serialized_contacts)?;
    Ok(())
}
