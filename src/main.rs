use std::env;

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("{} v{} by {}", name, version, authors);
        println!("Usage: {} <temperature> <input_scale> <output_scale>", name);
        println!(
            "       {} 0 c f  // will convert 0 Celcius to Fahrenheit",
            name
        );
        println!("Supported: c/f/k");

        std::process::exit(1);
    }

    let temperature: f64 = args[1].parse().expect("Invalid temperature argument");
    let input_scale = &args[2];
    let output_scale = &args[3];

    let converted_temperature = match (input_scale.as_str(), output_scale.as_str()) {
        ("c", "f") => celsius_to_fahrenheit(temperature),
        ("f", "c") => fahrenheit_to_celsius(temperature),
        ("c", "k") => celsius_to_kelvin(temperature),
        ("f", "k") => fahrenheit_to_kelvin(temperature),
        ("k", "c") => kelvin_to_celsius(temperature),
        ("k", "f") => kelvin_to_fahrenheit(temperature),
        _ => panic!("Invalid input/output scale combination"),
    };

    println!(
        "{} {} = {} {}",
        temperature,
        input_scale,
        two_decimal(converted_temperature),
        output_scale
    );
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 1.8) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit + 459.67) * 5.0 / 9.0
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    let celsius = kelvin_to_celsius(kelvin);
    celsius_to_fahrenheit(celsius)
}

fn two_decimal(value: f64) -> String {
    let formatted = format!("{:.2}", value);
    if formatted.ends_with(".00") {
        formatted[..formatted.len() - 3].to_string()
    } else {
        let mut trimmed = formatted.trim_end_matches('0').to_string();
        if trimmed.ends_with('.') {
            trimmed.pop();
        }
        trimmed
    }
}
