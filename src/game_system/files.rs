use std::path::Path;

use dialoguer::{theme::ColorfulTheme, Input};

pub fn is_valid_file_path(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.exists() && path.is_file()
}

pub fn read_file(file_path: &str) -> std::io::Result<String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}


pub fn get_file_path() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter a file path (or type 'exit' to quit)")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.to_lowercase() == "exit" || is_valid_file_path(input) {
                Ok(())
            } else {
                Err("Invalid file path")
            }
        })
        .interact_text()
        .unwrap()
}

pub fn create_file(file_path: &str) -> std::io::Result<()> {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(file_path)?;
    file.write_all(b"")?;
    Ok(())
}
