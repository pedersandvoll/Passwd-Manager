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
                let line_split: String = line
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<String>()
                    .expect("No passwords");
                println!("{}: {}", password_count, line_split);
            }
        }
    }
}
