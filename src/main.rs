use std::fs::File;
use std::io::{self, Read};
use clap::{Parser, Subcommand};

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

fn read_name_from_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut name = String::new();
    file.read_to_string(&mut name)?;
    Ok(name.trim().to_string())
}

fn get_greeting(language: &str, name: &str) -> String {
    match language {
        "es" => format!("Hola, {}!", name),
        "bg" => format!("Здравей, {}!", name),
        _ => format!("Hello, {}!", name),
    }
}
