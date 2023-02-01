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
    #[clap(
        version = "1.0",
        author = "Selina Liu",
        about = "draw a random carot card"
    )]
    Tarot {
        // #[clap(short, long)]
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Tarot {}) => {
            let result = mini2::draw();
            println!("{}", result);
        }
        // Some(Commands::Convert { name }) => {
        //     let result = mini2::interpret(&name);
        //     println!("{}", result);
        // }
        None => println!("No subcommand was used"),
    }
}
