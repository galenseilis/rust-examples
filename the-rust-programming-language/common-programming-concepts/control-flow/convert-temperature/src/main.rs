fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
	(fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    let celsius_temperature = 20.0; // Example Celsius temperature
    let fahrenheit_temperature = celsius_to_fahrenheit(celsius_temperature);
    println!("{}°C is equal to {}°F", celsius_temperature, fahrenheit_temperature);
	let back_to_celcius = fahrenheit_to_celcius(fahrenheit_temperature);
	assert_eq!(celsius_temperature, back_to_celcius);
}

