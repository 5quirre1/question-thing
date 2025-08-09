use std::io::{self, Write};

pub fn question(message: &str, continue_message: &str) -> String {
	loop {
		println!("{}", message);
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("failed to read line");
		let trimmed = input.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        } else {
            println!("{}", continue_message);
        }
    }
}
