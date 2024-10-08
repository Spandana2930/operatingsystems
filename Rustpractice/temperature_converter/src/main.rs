// Constant for freezing point of water in Fahrenheit
const FREEZING_POINT_FAHRENHEIT: f64 = 32.0;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_FAHRENHEIT) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_FAHRENHEIT
}

fn main() {
    // Declare a mutable variable for temperature in Fahrenheit
    let mut temp_fahrenheit: f64 = 32.0;

    // Convert the temperature to Celsius and print the result
    let temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
    println!("{:.2}°F is {:.2}°C", temp_fahrenheit, temp_celsius);

    // Use a loop to convert and print the next 5 temperatures
    for _ in 1..=5 {  // Prefix the loop variable with an underscore
        temp_fahrenheit += 1.0;
        let temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
        println!("{:.2}°F is {:.2}°C", temp_fahrenheit, temp_celsius);
    }

    // Example usage of the celsius_to_fahrenheit function to avoid dead code warning
    let temp_celsius_example = 0.0; // Example Celsius temperature
    let converted_back_to_fahrenheit = celsius_to_fahrenheit(temp_celsius_example);
    println!("{:.2}°C is {:.2}°F", temp_celsius_example, converted_back_to_fahrenheit);
}
