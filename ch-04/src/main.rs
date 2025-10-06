use std::io;

fn main() {
    let mut s = String::new(); 
    println!("Please enter a string: ");
    io::stdin().read_line(&mut s).expect("Failed to read line"); 

    for c in s.trim().chars() {
        if c == ' ' {
            println!();
            break;
        } 
        print!("{}", c);
    }
}
