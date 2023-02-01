//A command-line tool to play Marco Polo
use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selina Liu",
    about = "A CLI tool that converts an integer to a Roman numeral"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Selina Liu")]
    Convert {
        #[clap(short, long)]
        num: u32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Convert { num }) => {
            let result = mini1::convert(&num);
            println!("{}", result);
        }
        None => println!("No subcommand was used"),
    }
}
