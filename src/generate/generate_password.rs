use std::{
    fs::{File, OpenOptions},
    io::{stdin, stdout, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

pub fn generate_password(file_path: PathBuf) {
    if !Path::new(&file_path).exists() {
        File::create(&file_path).expect("Failed to create file");
        println!("File created at {:?}", file_path);
    }

    let file_content: Vec<String> =
        BufReader::new(File::open(&file_path).expect("cannot open password.txt"))
            .lines()
            .map(|line| line.expect("Failed to read line"))
            .collect();

    let mut file = OpenOptions::new()
        .append(true)
        .write(true)
        .open(&file_path)
        .expect("cannot append file");

    let mut password_name = String::new();
    let mut password_value = String::new();

    loop {
        loop {
            println!("Enter name of password you want to create:");
            let _ = stdout().flush();
            stdin()
                .read_line(&mut password_name)
                .expect("Failed try again");

            let formatted_name = format!("password_name {}", password_name.trim());

            if file_content.contains(&formatted_name) {
                println!("Name cannot be the same as another saved password name");
                password_name.clear();
            } else {
                break;
            }
        }

        loop {
            println!("Enter password for {}", password_name.trim());
            let _ = stdout().flush();
            stdin()
                .read_line(&mut password_value)
                .expect("Failed try again");

            if password_value.trim().len() < 8 {
                println!("Password cannot be less than 8 characters");
                password_value.clear();
            } else {
                break;
            }
        }

        let formatted_name = format!("password_name {}", password_name.trim());

        let _ = file
            .write_all(formatted_name.as_bytes())
            .expect("Name append failed");
        file.write_all(b"\n").expect("Failed to write newline");
        file.flush().expect("Failed to flush after name append");

        let formatted_password = format!(
            "password({}) {}",
            password_name.trim(),
            password_value.trim()
        );
        let _ = file
            .write_all(formatted_password.as_bytes())
            .expect("Password append failed");
        file.write_all(b"\n").expect("Failed to write newline");
        file.flush().expect("Failed to flush after password append");
        break;
    }
}
