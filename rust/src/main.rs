mod cli;
mod hash;
mod luck;

use clap::Parser;
use cli::Args;

fn main() {
    let args = Args::parse();

    // Check if we can execute
    match args.can_execute() {
        Ok(show_warning) => {
            if show_warning {
                eprintln!("WARNING: Running outside January 1st with --force flag.");
            }
        }
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    }

    println!("Year: {}, User: {}", args.year, args.user);
}
