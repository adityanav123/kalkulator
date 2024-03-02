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
    // The mathematical expression to be processed.
    #[arg(short, long)]
    expr: String,

    /// Flag to only convert the expression to postfix notation, without evaluating it.
    #[arg(short = 'p', long = "postfix", action = clap::ArgAction::SetTrue)]
    to_postfix: bool,
}

fn main() {
    let args = Args::parse();

    let expression = args.expr.trim();
    let mut obj = Expression::new(expression);

    match obj.process_expression(!args.to_postfix) {
        Ok(_) => {} // In case of success, nothing needs to be done here.
        Err(e) => {
            // Handle different kinds of errors with appropriate messages
            eprintln!("Error processing expression: {}", e.as_str());
        }
    }
}
