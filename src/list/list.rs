use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn list_available_passwords(file_path: PathBuf) {
    let file = BufReader::new(File::open(file_path).expect("Cannot open input"));
    let mut password_count = 0;

    for line in file.lines() {
        if let Ok(line) = line {
            if line.contains("password_name") {
                password_count += 1;
                let line_split: Result<String, &str> = line
                    .split(" ")
                    .last()
                    .ok_or("No words found")
                    .and_then(|s| s.parse::<String>().map_err(|_| "Failed to parse string"));

                match line_split {
                    Ok(password) => {
                        println!("{}. {}", password_count, password);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
    }
}
