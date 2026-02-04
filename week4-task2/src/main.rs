use std::io;

struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        3.142 * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * 3.142 * self.radius
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the radius of the circle:");

    io::stdin().read_line(&mut input).expect("Failed to read input");

    let radius: f64 = input.trim().parse().expect("Invalid number");

    let circle = Circle { radius };

    println!("Radius: {}", circle.radius);
    println!("Area: {:.2}", circle.area());
    println!("Circumference: {:.2}", circle.circumference());
}