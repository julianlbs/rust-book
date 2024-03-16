use std::io;

fn main() {
    println!("Select the convertion you want to perform");
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => {
            println!("Enter the temperature in Fahrenheit:");
            let mut temperature = String::new();
            io::stdin().read_line(&mut temperature).expect("Failed to read line");
            let temperature: f64 = temperature.trim().parse().expect("Invalid input");
            let celsius = fahrenheit_to_celsius(temperature);
            println!("The temperature in Celsius is: {}", celsius);
        }
        "2" => {
            println!("Enter the temperature in Celsius:");
            let mut temperature = String::new();
            io::stdin().read_line(&mut temperature).expect("Failed to read line");
            let temperature: f64 = temperature.trim().parse().expect("Invalid input");
            let fahrenheit = celsius_to_fahrenheit(temperature);
            println!("The temperature in Fahrenheit is: {}", fahrenheit);
        }
        _ => {
            println!("Invalid choice");
        }
    };
}


fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}