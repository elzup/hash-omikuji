mod cli;
mod hash;
mod luck;
mod output;

use clap::Parser;
use cli::Args;
use hash::HashBits;
use output::OmikujiResult;

fn main() {
    let args = Args::parse();

    // Check if we can execute
    let show_warning = match args.can_execute() {
        Ok(warning) => warning,
        Err(msg) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
    };

    let year = args.get_year();

    if show_warning && !args.json {
        eprintln!("WARNING: Running outside January 1st with --force-year {}.\n", year);
    }

    // Get seed (default: username@hostname)
    let seed = args.get_seed();

    // Generate hash and result
    let hash = HashBits::from_seed(year, &seed);
    let result = OmikujiResult::from_hash(&hash, year, &seed);

    // Output
    if args.json {
        println!("{}", result.format_json());
    } else {
        print!("{}", result.format_text(args.short, args.show_seed));
    }
}
