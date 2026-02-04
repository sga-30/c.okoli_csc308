use std::io;

fn main() {
    println!("Enter a sentence:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let words: Vec<&str> = input.split_whitespace().collect();

    if words.is_empty() {
        println!("No words found!");
        return;
    }

    let mut shortest = words[0];
    let mut longest = words[0];

    for &word in &words {
        if word.len() < shortest.len() {
            shortest = word;
        }
        if word.len() > longest.len() {
            longest = word;
        }
    }

    println!("Shortest word: {}", shortest);
    println!("Longest word: {}", longest);
}