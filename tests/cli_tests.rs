use cli_rust::{read_name_from_file, get_greeting};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_read_name_from_file() -> std::io::Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "John Doe")?;
    
    let result = read_name_from_file(temp_file.path().to_str().unwrap())?;
    assert_eq!(result, "John Doe");
    
    Ok(())
}

#[test]
fn test_get_greeting() {
    assert_eq!(get_greeting("en", "Alice"), "Hello, Alice!");
    assert_eq!(get_greeting("es", "Bob"), "Hola, Bob!");
    assert_eq!(get_greeting("bg", "Charlie"), "Здравей, Charlie!");
    
    // Default to English for unsupported languages
    assert_eq!(get_greeting("fr", "David"), "Hello, David!"); 
}

#[test]
fn test_integration() -> std::io::Result<()> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "Eve")?;
    
    let name = read_name_from_file(temp_file.path().to_str().unwrap())?;
    let greeting = get_greeting("es", &name);
    
    assert_eq!(greeting, "Hola, Eve!");
    
    Ok(())
}
