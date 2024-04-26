mod contact;
mod json;
mod utils;
use contact::*;
use utils::run_menu;

fn main() {
    let mut contacts: Vec<Contact> = Vec::new();
    run_menu(&mut contacts);
}
