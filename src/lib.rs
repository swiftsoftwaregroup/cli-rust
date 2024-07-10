//! # CLI Rust
//! 
//! `cli_rust` is a command-line interface application that allows users to generate
//! greetings in various languages. It reads a name from a file and can produce
//! greetings in English, Spanish, or Bulgarian.

use std::fs::File;
use std::io::{self, Read};

/// Reads a name from a file.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the file
///
/// # Returns
///
/// This function returns a `Result` which is:
/// * `Ok(String)` containing the name if the file was successfully read
/// * `Err` if there was an error reading the file
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::Write;
/// use cli_rust::read_name_from_file;
///
/// # fn main() -> std::io::Result<()> {
/// let mut file = File::create("name.txt")?;
/// write!(file, "John Doe")?;
///
/// let name = read_name_from_file("name.txt")?;
/// assert_eq!(name, "John Doe");
/// # Ok(())
/// # }
/// ```
pub fn read_name_from_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut name = String::new();
    file.read_to_string(&mut name)?;
    Ok(name.trim().to_string())
}

/// Generates a greeting in the specified language.
///
/// # Arguments
///
/// * `language` - A string slice that holds the language code ("en", "es", or "bg")
/// * `name` - A string slice that holds the name to be greeted
///
/// # Returns
///
/// A `String` containing the greeting in the specified language.
///
/// # Examples
///
/// ```
/// use cli_rust::get_greeting;
///
/// let greeting = get_greeting("es", "Alice");
/// assert_eq!(greeting, "Hola, Alice!");
/// ```
pub fn get_greeting(language: &str, name: &str) -> String {
    match language {
        "es" => format!("Hola, {}!", name),
        "bg" => format!("Здравей, {}!", name),
        _ => format!("Hello, {}!", name),
    }
}
