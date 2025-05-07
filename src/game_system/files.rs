fn is_valid_file_path(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.exists() && path.is_file()
}

fn read_file(file_path: &str) -> std::io::Result<String> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main_menu() {
    loop {
        let file_path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter a file path (or type 'exit' to quit)")
            .validate_with(|input: &String| -> Result<(), &str> {
                if input.to_lowercase() == "exit" || is_valid_file_path(input) {
                    Ok(())
                } else {
                    Err("Invalid file path")
                }
            })
            .interact_text()
            .unwrap();

        if file_path.to_lowercase() == "exit" {
            println!("Exiting the program. Goodbye!");
            break;
        }

        match read_file(&file_path) {
            Ok(contents) => println!("File contents:\n{}", contents),
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }    
}
