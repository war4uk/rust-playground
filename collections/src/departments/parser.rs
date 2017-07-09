 use std::io;

#[derive(Debug)]
pub struct Command {
  pub name: String,
  pub department: String
}


 pub fn read_command() -> Option<Command> {
    println!("Enter a command.");
    
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let tokens = get_tokens(guess.trim());
    parse_command(&tokens) 
}

fn parse_command(tokens: &Vec<&str>) -> Option<Command> {
  let length = tokens.len();
  if length < 4 {
    return None;
  }

  if tokens[0].to_lowercase() != "add" {
    return None;
  }

  let to_keyword_index: usize;
  match find_to_keyword_index(&tokens) {
    Some(index) => to_keyword_index = index,
    None => return None
  } 

  if to_keyword_index < 2 {
    return None
  }

  if to_keyword_index == length - 1 {
    return None
  }

  let name_segments = &tokens[1..to_keyword_index];
  let department_segments = &tokens[to_keyword_index + 1..];

  Some(Command {
    name: name_segments.join(" "), 
    department: department_segments.join(" ").to_lowercase()
  })
}

fn find_to_keyword_index(tokens: &Vec<&str>) -> Option<usize> {
  for (index, token) in tokens.iter().rev().enumerate() {
    if token.trim() == "to" {
      return Some(tokens.len() - index - 1);
    }
  }
  None
}

fn get_tokens(guess: &str) -> Vec<&str> {
  guess.split(" ").collect::<Vec<&str>>()
}