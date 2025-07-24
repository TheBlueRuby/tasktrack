use std::io::Read;

pub fn get_yes_no() -> bool {
    println!("Enter response [y/n]:");
    loop {
        let mut input: [u8; 1] = [0];
        let _ = std::io::stdin().read(&mut input);
        match input[0] as char {
            'y' | 'Y' => return true,
            'n' | 'N' => return false,
            _ => println!("y/n only please."),
        }
    }
}
