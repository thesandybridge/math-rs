use clap::Parser;
use eval::eval;

/// Performs mathematical calculations and unit conversions
#[derive(Parser, Debug)]
#[command(version = "1.0", about, long_about = None)]
struct Args {
    /// Activate verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Mathematical expression to evaluate or value for unit conversion
    #[arg(required = true)]
    expression: Vec<String>,

    /// Unit conversion, specified as 'input_unit:output_unit' (e.g., 'c:f' for Celsius to Fahrenheit)
    #[arg(short, long)]
    unit_conversion: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(unit_conversion) = args.unit_conversion {
        // Perform unit conversion
        if let Err(e) = perform_unit_conversion(&args.expression.join(" "), &unit_conversion) {
            eprintln!("Error performing unit conversion: {:?}", e);
        }
    } else {
        let expression_str = args.expression.join(" ");

        let modified_expression = expression_str.replace("x", "*").replace(".", "*");

        match eval(&modified_expression) {
            Ok(result) => {
                if args.verbose {
                    println!("{} = {} : 0.5ns", modified_expression, result);
                } else {
                    println!("{} = {}", modified_expression, result);
                }
            }
            Err(e) => eprintln!("Error evaluating expression: {:?}", e),
        }
    }
}

fn perform_unit_conversion(expression: &str, unit_conversion: &str) -> Result<(), &'static str> {
    let parts: Vec<&str> = unit_conversion.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid unit conversion format. Use 'input_unit:output_unit'.");
    }
    let value: f64 = expression
        .parse()
        .map_err(|_| "Invalid value for conversion")?;

    match (parts[0], parts[1]) {
        ("c", "f") => println!("{}°C is {}°F", value, celsius_to_fahrenheit(value)),
        ("f", "c") => println!("{}°F is {}°C", value, fahrenheit_to_celsius(value)),
        ("km", "mi") => println!("{} km is {} mi", value, kilometers_to_miles(value)),
        ("mi", "km") => println!("{} mi is {} km", value, miles_to_kilometers(value)),
        _ => return Err("Unsupported unit conversion"),
    }

    Ok(())
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn kilometers_to_miles(kilometers: f64) -> f64 {
    kilometers * 0.621371
}

fn miles_to_kilometers(miles: f64) -> f64 {
    miles / 0.621371
}
