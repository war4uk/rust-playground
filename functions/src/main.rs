use std::io;

fn main() {
    let is_celsius_to_fahrenheit: bool = read_conversion();
    println!(":{}", is_celsius_to_fahrenheit);
    let temperature = read_value();

    if (is_celsius_to_fahrenheit) {
        println!("result: {}", cel_to_fahr(temperature));
    } else {
        println!("result: {}", fahr_to_cel(temperature));     
    }
}

fn cel_to_fahr(temp: f64) -> f64 {
    temp * 1.8 + 32.0
}

fn fahr_to_cel(temp: f64) -> f64 {
    (temp - 32.0) / 1.8
}

fn read_conversion() -> bool {
    loop {
        println!("Press 1 for Celsius to Fahrenheit. \r\nPress 2 for Fahrenheit to Celsius");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let parsed_input = user_input.trim();
        if parsed_input == "1" {
            return true;
        }

        if parsed_input == "2" {
            return false;      
        }
    } 
}

fn read_value() -> f64 {
    loop { 
        println!("Enter temperature");
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let parsed_input: f64 = match user_input.trim().parse() {
            Ok(res) => res,
            Err(_) => continue
        };

        println!("yay {}", parsed_input);
        return parsed_input;
    }
}

