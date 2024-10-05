mod generate;
mod list;
mod show;
use std::{
    env,
    io::{stdin, stdout, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = dirs::home_dir().expect("no desktop directory");
    let file_path = path.join("password.txt");

    if args.len() > 1 && args[1] == "-ls" {
        list::list::list_available_passwords(file_path);
    } else {
        loop {
            println!("Do you wish to create a new password? (1)");
            println!("Do you wish to gather already created password? (2)");

            let mut s = String::new();
            let _ = stdout().flush();
            stdin().read_line(&mut s).expect("Failed try again");

            match s.trim() {
                "1" => {
                    generate::generate_password::generate_password(file_path);
                    break;
                }
                "2" => {
                    show::gather_password::gather_password(file_path);
                    break;
                }
                _ => println!("{} is not an option", s.trim()),
            }
        }
    }
}
