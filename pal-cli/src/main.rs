use std::io::{self, Write};

fn is_pallindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let reversed: String = cleaned.chars().rev().collect();
    cleaned.eq_ignore_ascii_case(&reversed)
}

fn main() {
    loop {
        print!("Enter a word or number (or type 'exit' to quit): " );
        io::stdout().flush().unwrap(); //flush out to prompt user input immediatly

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim(); // Remove Whitespaces
        if input.eq_ignore_ascii_case("exit"){
            println!("Goodbye!");
            break;
        }

        if is_pallindrome(input) {
            println!("✅ '{}' is a palindrome!", input);
        }
        else {
            println!("❌ '{}' is NOT a palindrome.", input);
        }

    }
}