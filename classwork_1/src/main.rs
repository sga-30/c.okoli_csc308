use std::io;

fn main() {
    let mut temp1 = String::new();
    let mut temp2 = String::new();

    println!("\nPlease enter the first temperature in Celsius");
        io::stdin().read_line(&mut temp1).expect("Not a valid string");
        let a:f32 = temp1.trim().parse().expect("Not a valid number");
    
    println!("\nPlease enter the second temperature in Fahrenheit");
        io::stdin().read_line(&mut temp2).expect("Not a valid string");
        let b:f32 = temp2.trim().parse().expect("Not a valid number");

    let new_temp1:f32 = (a * (9.0/5.0)) + 32.0;
    let new_temp2:f32 = (b - 32.0) * (5.0/9.0);

    println!("\nTemperature: {}C", a);
    println!("\nConverted: {}F", new_temp1);
    println!("\nTemperature: {}F", b);
    println!("\nConverted: {}C", new_temp2);
    
}