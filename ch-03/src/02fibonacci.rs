use std::io;

fn fibonacci(n: u32) -> u32 {
    if n ==1 || n==2 {
        1
    } else if n == 0 {
        0
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

fn main() {
    println!("Enter a positive integere");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a positive integer");
    let result = fibonacci(input);
    println!("Fibonacci({}) = {}", input, result);
}