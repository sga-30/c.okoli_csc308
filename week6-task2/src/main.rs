fn main() {
    let numbers: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let is_even = |n: u32| -> bool {
        n % 2 == 0
    };

    let mut even_numbers: Vec<u32> = Vec::new();
    
    for n in numbers {
        if is_even(n) {
            even_numbers.push(n);
        }
    }
    
    println!("Even Numbers: {:?}", even_numbers);
}