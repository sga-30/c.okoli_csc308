use std::io;

fn main() {  
    let mut input = String::new();

    println!("Enter the original bill");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let old_bill:f32 = input.trim().parse().expect("Not a valid number");
    let new_bill:f32;
    let discount:i32;

    if old_bill >= 5000.0 && old_bill < 10000.0{
        new_bill = 0.9 * old_bill;
        discount = 10;
    } else if old_bill >= 10000.0{
        new_bill = 0.85 * old_bill;
        discount = 15;
    } else {
        new_bill = old_bill;
        discount = 0;
    }

    println!("Original Bill: N{}", old_bill);
    println!("Discount Applied: {}%", discount);
    println!("Final Bill: N{}", new_bill);
}