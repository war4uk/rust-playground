use std::collections::HashMap;

mod parser;

pub struct DepartmentsManager {
    deparments: HashMap<String, Vec<String>>
}

impl DepartmentsManager {
  pub fn new() -> DepartmentsManager {
    DepartmentsManager {
      deparments: HashMap::new()
    }
  }

  pub fn accept_input(&mut self) {
    loop { 
      let input = parser::read_command();
      match input {
        Some(command) => self.add_user(command),
        None => println!("Cannot understand you")
      }

      println!("{:?}", self.deparments);
    }
  } 

  fn add_user(&mut self, command: parser::Command) {
    let users_in_department = self.deparments
      .entry(command.department).or_insert(vec![]);

    users_in_department.push(command.name);
  }
} 