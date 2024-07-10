use std::io;
use clap::{Parser, Subcommand};
use cli_rust::{read_name_from_file, get_greeting};

#[derive(Parser)]
#[command(name = "cli-rust")]
#[command(version = "1.0")]
#[command(about = "CLI tool for greeting users in different languages")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Greet {
        /// The file containing the name
        file: String,

        /// Sets the greeting language. Available languages: [es, bg, en].
        #[arg(short, long, default_value = "en")]
        language: String,
    },
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Greet { file, language } => {
            let name = read_name_from_file(file)?;
            let greeting = get_greeting(language, &name);
            println!("{}", greeting);
        }
    }

    Ok(())
}
