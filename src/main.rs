use clap::Parser;
use eval::eval;

/// Performs mathematical calculations
#[derive(Parser, Debug)]
#[command(version = "1.0", about, long_about = None)]
struct Args {
    /// Activate verbose mode
    #[arg(short, long)]
    verbose: bool,

    /// Mathematical expression to evaluate
    #[arg(required = true)]
    expression: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.expression.is_empty() {
        return;
    }

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
