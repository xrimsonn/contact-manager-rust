use crate::utils::read_string_input;

pub struct Contact {
  pub name: String,
  pub phone_number: String,
  pub address: String,
}

pub fn print_contact(contact: &Contact) {
  let contact_info = format!(
      "------------\n{}\n{}\n{}",
      contact.name, contact.phone_number, contact.address
  );
  println!("{}", contact_info);
}

pub fn print_contact_list(contacts: &mut Vec<Contact>) {
  for contact in contacts.iter() {
      print_contact(contact);
  }
  print!("\n");
}

pub fn _add_contact(contacts: &mut Vec<Contact>) {
  let new_contact = Contact {
      name: read_string_input("Enter the contact's name"),
      phone_number: read_string_input("Enter the contact's phone number"),
      address: read_string_input("Enter the contact's address"),
  };
  contacts.push(new_contact);
  println!("Contact added!\n");
}

pub fn _search_contact(contacts: &mut Vec<Contact>) {
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

pub fn delete_contact(contacts: &mut Vec<Contact>) {
  let search_input = read_string_input("Enter the contact's name to delete (case sensitive)");
  let mut not_found = true;
  for i in (0..contacts.len()).rev() {
      if contacts[i].name == search_input {
          contacts.remove(i);
          not_found = false;
      }
  }
  if not_found {
      println!("No results for {}.", search_input);
  } else {
      println!("{} contact deleted.", search_input);
  }
  print!("\n");
}
