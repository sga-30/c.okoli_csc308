fn main() {
    let s1 = String::from("Hi");
    let s2 = String::from("amazing!");

    let result = longest(&s1, &s2);
    println!("The longer string is: {}", result);
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}