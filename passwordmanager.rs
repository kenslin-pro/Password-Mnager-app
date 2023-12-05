use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let mut password_manager = PasswordManager::new("passwords.txt");

    loop {
        println!("1. Store Password");
        println!("2. Retrieve Password");
        println!("3. Exit");

        let choice = get_user_input("Enter your choice: ").trim().parse::<u32>();

        match choice {
            Ok(1) => {
                let service = get_user_input("Enter service: ");
                let username = get_user_input("Enter username: ");
                let password = get_user_input("Enter password: ");
                password_manager.store_password(service, username, password);
            }
            Ok(2) => {
                let service = get_user_input("Enter service: ");
                match password_manager.retrieve_password(service) {
                    Some(password) => println!("Password: {}", password),
                    None => println!("Service not found."),
                }
            }
            Ok(3) => {
                password_manager.save_to_file();
                println!("Exiting password manager. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a valid option."),
        }
    }
}

struct PasswordManager {
    passwords: HashMap<String, String>,
    file_path: String,
}

impl PasswordManager {
    fn new(file_path: &str) -> PasswordManager {
        let passwords = PasswordManager::load_from_file(file_path);
        PasswordManager {
            passwords,
            file_path: file_path.to_string(),
        }
    }

    fn store_password(&mut self, service: String, username: String, password: String) {
        let entry = format!("{}:{}", username, password);
        self.passwords.insert(service, entry);
        println!("Password stored successfully for {}.", service);
    }

    fn retrieve_password(&self, service: String) -> Option<&String> {
        self.passwords.get(&service)
    }

    fn load_from_file(file_path: &str) -> HashMap<String, String> {
        let path = Path::new(file_path);
        let mut passwords = HashMap::new();

        if path.exists() {
            let file = File::open(path).expect("Failed to open file.");
            let reader = BufReader::new(file);

            for line in reader.lines() {
                if let Ok(entry) = line {
                    let parts: Vec<&str> = entry.split(':').collect();
                    if parts.len() == 2 {
                        passwords.insert(parts[0].to_string(), parts[1].to_string());
                    }
                }
            }
        }

        passwords
    }

    fn save_to_file(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)
            .expect("Failed to open file for writing.");

        for (service, entry) in &self.passwords {
            writeln!(file, "{}:{}", service, entry).expect("Failed to write to file.");
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");

    input.trim().to_string()
}
