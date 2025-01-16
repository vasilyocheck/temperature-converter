use std::io;
fn main() {
    println!("Hi, this is a temperature converter!");
    println!("If you want to conver Fahrenheit to Celsius input 1, and if Celcius to Fahrenheit - input 2:");

    let conversion_direction = get_user_input("Enter your choice(1 or 2): ");

    let temperature = match conversion_direction.trim() {
        "1" => {
            println!("We are going to convert Fahrenheit to Celsius");
            &get_user_input("Please input the temperature in Fahrenheit to convert to Celsius:")
        },
        "2" => {
            println!("We are going to convert Celsius to Fahrenheit");
            &get_user_input("Please input the temperature in Celsius to convert to Fahrenheit:")
        },
        _ => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };

    let temperature: f64 = match temperature.trim().parse(){
        Ok(temperature) => temperature,
        Err(_) => {
            println!("Invalid temperature input");
            return;
        },
    };

    match conversion_direction.trim() {
        "1" => {
            let temperature_in_fahrenheit = round_to_decimals((temperature - 32.0) * 5.0 / 9.0);
            
            println!("The temperature is {} degrees Celsius.", temperature_in_fahrenheit);
        },
        "2" => {
            let temperature_in_celsius = round_to_decimals(temperature * 9.0 / 5.0  + 32.0);
            println!("The temperature is {} degrees Fahrenheit.", temperature_in_celsius);
        },
        _ => {
            return
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
    input
}

fn round_to_decimals (value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}