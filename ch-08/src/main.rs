use std::io;
use std::collections::HashMap;

fn main() {
    println!("1. Median and Mode from a list of integers.\n2. String to pig latin. \n3. Allow user to add employee to department.\nEnter the choice of problem (1-3):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    let choice = choice.trim().parse().expect("Please enter a valid number");

    match choice {
        1 => {
            println!("Enter the integer list(space-separated):");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let mut numbers: Vec<i32> = input.trim().split_whitespace()
                .map(|s| s.parse().expect("Please enter valid integers"))
                .collect();

            numbers.sort();

            let len = numbers.len();
            let median = if len % 2 == 0 {
                (numbers[len/2 - 1] + numbers[len/2]) as f32 / 2.0
            } else {
                numbers[len/2] as f32
            };
            println!("Median: {}", median);

            let mut map = HashMap::new();
            for num in &numbers {
                let count = map.entry(num).or_insert(0);
                *count += 1;
            }
            let mut modes = Vec::new();
            if let Some((_, count)) = map.iter().max_by_key(|e| e.1).map(|(&num, &count)| (num,count)) {
                modes = map.iter().filter(|&(_, &v)| v == count).map(|(&k,_)| k).collect();
            }            
            println!("Modes:{:?}", modes);
        }
        2 => {
            println!("Enter the string:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input.");
            let input = input.trim();
            let mut pig_latin = String::new();

            if let Some(first_char) = input.chars().next() {
                if matches!(first_char, 'a' | 'e' | 'i' | 'o' | 'u')  {
                    pig_latin = format!("{}-hay", input);
                } else {
                    pig_latin = format!("{}-{}ay", &input[1..], first_char);
                }
            }
            println!("Pig-latin string: {}", pig_latin);
        }
        3 => {
            let mut map: HashMap<String, Vec<String>> = HashMap::new();

            loop {
                println!("Enter the option (a-c):\na. Add user to dept.\nb. Query users in dept.\nc. Exit:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input.");
                match input.trim().chars().next() {
                    Some('a') => {
                        println!("Enter the user name:");
                        let mut user = String::new();
                        io::stdin().read_line(&mut user).expect("Failed to read user.");
                        let user = user.trim().to_string();

                        println!("Enter the department:");
                        let mut dept = String::new();
                        io::stdin().read_line(&mut dept).expect("Failed to read department.");
                        let dept = dept.trim().to_uppercase();

                        map.entry(dept)
                            .or_insert_with(Vec::new)
                            .push(user);
                        println!("User added.");
                    }
                    Some('b') => {
                        println!("Enter the department:");
                        let mut dept = String::new();
                        io::stdin().read_line(&mut dept).expect("Failed to read department.");
                        let dept = dept.trim().to_uppercase();
                        match map.get(&dept) {
                            Some(users) if !users.is_empty() => {
                                println!("Users in {}: {:?}", dept, users);
                            }
                            _ => {
                                println!("No such department or no users in department.");
                            }
                        }
                    }
                    Some('c') => {
                        println!("Exiting.");
                        break;
                    }
                    _ => {
                        println!("Invalid choice. Please enter a, b, or c.");
                    }
                }
            }
        }
        _ => println!("Invalid choice! Please enter a number between 1 and 3."),
    }
}
