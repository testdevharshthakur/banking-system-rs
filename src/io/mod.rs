use std::io::{self, Write};

/// Clears the terminal
pub fn clearScreen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// prompts the user for input and return it as a trimmed string.
pub fn get_input_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

/// Prompts user for a u32 input and parses it
pub fn get_input_u32(prompt: &str) -> Option<u32> {
    let input = get_input_string(prompt);
    input.parse().ok()
}

/// Prompts user for a f64 input and parses it
pub fn get_input_f64(prompt: &str) -> Option<f64> {
    let input = get_input_string(prompt);
    input.parse().ok()
}

/// Pauses the program until user presses Enter
pub fn press_enter_to_continue() {
    println!("Press Enter to continue ...");
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}
