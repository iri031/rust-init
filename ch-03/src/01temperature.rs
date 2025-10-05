use std::io;

fn FtoC(temp: f64) -> f64 {
    (temp -32.0) * 5.0/9.0   
}

fn main() {
    println!("Enter Fahrenheit temperature:");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("Failed to read line");
    let temp: f64 = temp.trim().parse().expect("Please type a float!");
    let tempC = FtoC(temp);
    println!("Temperature in Celsius is: {}", tempC);
}