use std::io::{self, Write};

pub fn read_string_input(msg: &str) -> String {
  let mut var = String::new();
  print!("{}: ", msg);
  io::stdout().flush().unwrap();
  io::stdin()
      .read_line(&mut var)
      .expect("Failed to read line.");
  var.trim().to_string()
}
