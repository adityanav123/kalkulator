use clap::Parser;
use kalkulator::Expression;

/// Command-line arguments accepted by `kalkulator`.
#[derive(Parser, Debug)]
#[command(
    name = "kalkulator",
    about = "kalkulator: A command line calculator using Rust",
    version = "0.1.1"
)]
pub struct Args {
    /// The mathematical expression to be processed.
    #[arg(short, long)]
    expr: Option<String>,

    /// Flag to only convert the expression to postfix notation, without evaluating it.
    #[arg(short = 'p', long = "postfix", action = clap::ArgAction::SetTrue)]
    to_postfix: bool,

    /// Flag to display the available operators.
    #[arg(short = 's', long = "show-ops", action = clap::ArgAction::SetTrue)]
    show_ops: bool,
}

fn main() {
    let args = Args::parse();

    if args.show_ops {
        // Display available operators and exit
        println!("Supported operators:");
        println!("+ : Addition");
        println!("- : Subtraction");
        println!("* : Multiplication");
        println!("/ : Division");
        println!("^ : Exponentiation");
        println!("& : Logical AND (integer operation)");
        println!("| : Logical OR (integer operation)");
        println!("~ : Logical XOR (integer operation)");
        println!("! : Factorial");
        return; // Exit after displaying the information
    }

    if let Some(expression) = args.expr {
        let trimmed_expression = expression.trim();
        let mut obj = Expression::new(trimmed_expression);

        match obj.process_expression(!args.to_postfix) {
            Ok(_) => {} // In case of success, nothing needs to be done here.
            Err(e) => {
                // Handle different kinds of errors with appropriate messages
                eprintln!("Error processing expression: {}", e.as_str());
            }
        }
    } else if !args.show_ops {
        eprintln!("No expression provided. Use --help for more information or --show-ops to list available operations.")
    }
}
