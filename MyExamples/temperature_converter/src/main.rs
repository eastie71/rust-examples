use std::io;

fn main() {
    println!("\nTemperature calculator - From Fahrenheit to Celsius (or vice versa)");

    let enter_scale_type = get_enter_scale_type();
    // println!("Scale type = {}", enter_scale_type);

    if enter_scale_type == 'F' {
        calc_celsius_from_fahrenheit();
    } else if enter_scale_type == 'C' {
        calc_fahrenheit_from_celsius();
    }
}

fn read_line_to_string() -> String {
    let mut entry_str: String = String::new();

    io::stdin()
        .read_line(&mut entry_str)
        .expect("Failed to read line");

    entry_str.trim().to_string()
}

fn get_enter_scale_type() -> char {
    loop {
        println!("\nEnter the Temperature type you want to convert FROM.\nEnter either F (Fahrenheit) or C (Celsius)");

        let mut scale_type: String = read_line_to_string();

        scale_type = scale_type.trim().to_uppercase();
        // println!("Entered value = {}", scale_type);
        if scale_type != "F" && scale_type != "C" {
            println!("Must enter either F or C");
            continue;
        }
        // let char_type: char = match scale_type.chars().next() {
        //     Some(c) => {
        //         if c.to_uppercase().to_string() != "F" && c.to_uppercase().to_string() != "C" {
        //             println!("Must enter either F or C");
        //             continue;
        //         }
        //         c
        //     },
        //     None => {
        //         println!("Must enter either F or C");
        //         continue;
        //     }
        // };

        return scale_type.chars().next().expect("Unexpected error: Try again!");
        //println!("Type entered = {}", char_type);
        //return char_type;
    }
}

fn calc_fahrenheit_from_celsius() {
    let temp_c: i32;

    loop {
        println!("Enter temperature in Celsuis (no decimals) to convert to Fahnenheit:");
        let temp: String = read_line_to_string();

        temp_c = match temp.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Must enter a valid integer number");
                continue;
            }
        };
        break;
    }

    let temp_f: f64 = (temp_c as f64 * 9.0 / 5.0) + 32.0;
    println!("{}°C converts to {:.1}°F", temp_c, temp_f);
}

fn calc_celsius_from_fahrenheit() {
    let temp_f: i32;

    loop {
        println!("Enter temperature in Fahrenheit (no decimals) to convert to Celsuis:");
        let temp: String = read_line_to_string();

        temp_f = match temp.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Must enter a valid integer number");
                continue;
            }
        };
        break;
    }
    
    let temp_c: f64 = (temp_f as f64 - 32.0)* 5.0 / 9.0;
    println!("{}°F converts to {:.1}°C", temp_f, temp_c);
}