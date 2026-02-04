use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    score: f32,
}

impl Student {
    fn passed(&self) -> bool {
        self.score >= 50.0
    }

    fn display_result(&self) {
        if self.passed() {
            println!("{} passed with a score of {:.1} ", self.name, self.score);
        } else {
            println!("{} failed with a score of {:.1} ", self.name, self.score);
        }
    }
}

fn main() {
    let mut name = String::new();
    let mut input = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Enter student score:");
    io::stdin().read_line(&mut input).expect("Failed to read score");

    let score: f32 = input.trim().parse().expect("Invalid number");

    let student = Student {
        name: name.trim().to_string(),
        score,
    };

    student.display_result();
}