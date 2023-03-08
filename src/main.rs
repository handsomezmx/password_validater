use regex::Regex;
use std::io;

fn main() {
    // Define a regular expression to match special characters.
    let special_chars = Regex::new(r"[!@#$%^&*()_+-=]").unwrap();

    // Prompt the user to enter a password and read it from standard input.
    println!("Enter a password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    // Calculate the password's score based on its length, use of special characters, numbers, uppercase and lowercase letters.
    let score = calculate_score(&password, &special_chars);

    // Output a strength rating for the password based on its score.
    match score {
        0..=3 => println!("Weak"),
        4..=6 => println!("Medium"),
        _ => println!("Strong"),
    }
}

// Calculate a score for the password based on its length, use of special characters, numbers, uppercase and lowercase letters.
fn calculate_score(password: &str, special_chars: &Regex) -> i32 {
    let mut score = 0;

    if password.len() >= 8 {
        score += 1;
    }

    if special_chars.is_match(password) {
        score += 1;
    }

    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_ascii_uppercase()) {
        score += 1;
    }

    if password.chars().any(|c| c.is_ascii_lowercase()) {
        score += 1;
    }

    score
}