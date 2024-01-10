enum TemperatureFormat {
    Celsius,
    Fahrenheit,
}

fn main() {
    println!("## Temperature Conversion ##");

    println!("What is the source temperate format? (C or F)");

    let mut format = String::new();

    std::io::stdin()
        .read_line(&mut format)
        .expect("Unable to read line");

    let format = format.trim();

    let format = match format {
        "C" => TemperatureFormat::Celsius,
        "F" => TemperatureFormat::Fahrenheit,
        _ => panic!("Invalid format"),
    };

    println!("What is the temperature value?");

    let mut value = String::new();

    std::io::stdin()
        .read_line(&mut value)
        .expect("Unable to read line");

    let value = value
        .trim()
        .parse::<f64>()
        .expect("Please enter a valid number");

    let result = match format {
        TemperatureFormat::Celsius => c_to_f(value),
        TemperatureFormat::Fahrenheit => f_to_c(value),
    };

    println!("The result is: {}", result)
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) / 5.0 * 9.0
}

fn c_to_f(c: f64) -> f64 {
    (c * 9.0) / 5.0 + 32.0
}
