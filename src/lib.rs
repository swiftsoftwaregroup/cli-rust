use std::fs::File;
use std::io::{self, Read};

pub fn read_name_from_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut name = String::new();
    file.read_to_string(&mut name)?;
    Ok(name.trim().to_string())
}

pub fn get_greeting(language: &str, name: &str) -> String {
    match language {
        "es" => format!("Hola, {}!", name),
        "bg" => format!("Здравей, {}!", name),
        _ => format!("Hello, {}!", name),
    }
}