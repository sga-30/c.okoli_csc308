fn main() {
    let mut name = String::from("Firstname ");
    add_surname_to_firstname(&mut name);
    println!("{}", name);
}

fn add_surname_to_firstname(n: &mut String) {
    n.push_str("Lastname");
}