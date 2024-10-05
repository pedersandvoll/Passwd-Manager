use std::{
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

pub fn gather_password(file_path: PathBuf) {
    if !Path::new(&file_path).exists() {
        File::create(&file_path).expect("Failed to create file");
        println!("File created at {:?}", file_path);
    }

    let file_content: Vec<String> =
        BufReader::new(File::open(&file_path).expect("cannot open password.txt"))
            .lines()
            .map(|line| line.expect("Failed to read line"))
            .collect();
    let file = BufReader::new(File::open(file_path).expect("Cannot open input"));

    let mut password_name = String::new();

    loop {
        println!("Enter name of the password you want to gather:");
        let _ = stdout().flush();
        stdin()
            .read_line(&mut password_name)
            .expect("Failed try again");

        let formatted_name = format!("password_name {}", password_name.trim());

        if file_content.contains(&formatted_name) {
            break;
        } else {
            println!("Name not found");
            password_name = "".to_string();
        }
    }

    let formatted_name = format!("password({})", password_name.trim());

    for line in file.lines() {
        if let Ok(line) = line {
            if line.contains(&formatted_name) {
                let password = &line[formatted_name.len()..];
                println!("Password for {} is:{}", password_name.trim(), password);
            }
        }
    }
}
