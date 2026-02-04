fn main() {
    let factorial = |n: u64| -> u64 {
        let mut result = 1;
        for i in 2..=n {
            result *= i;
        }
        result
    };

    let num = 6;
    let result = factorial(num);
    println!("The factorial of {} is: {}", num, result);
}