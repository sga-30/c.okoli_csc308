use std::io;

fn last_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}

fn main() {
    let mut input = String::new();
    println!("Enter your string.");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    input = input.trim().to_string();
    let word: &str = last_word(&input);
println!("The last word is: {}", word);
}