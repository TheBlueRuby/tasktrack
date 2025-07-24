use std::io::Read;

pub fn get_yes_no() -> bool {
    println!("Enter response [y/n]:");
    loop {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input);
        match input.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("y/n only please."),
        }
    }
}
