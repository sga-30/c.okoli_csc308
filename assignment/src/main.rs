use std::io;

fn main() {
    println!("Enter your energy consumption in kWh: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let consumption: f32 = input.trim().parse().expect("Please enter a valid number");
    let rate:f32;
    
    if consumption > 100.0 && consumption < 200.0 {
        rate = 25.0
    } else if consumption >= 200.0 {
        rate = 30.0
    } else {
        rate = 20.0
    };

    let bill:f32 = consumption * rate;

    println!("Energy Consumed: {} kWh", consumption);
    println!("Rate Applied: ₦{} per unit", rate);
    println!("Total Electricity Bill: ₦{}", bill);
}