use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let config: Vec<i32> = input.trim().split_whitespace().map(|s| s.parse().expect("Please enter valid integers"))
                .collect();
    let (len, sum) = (config[0], config[1]);

    let input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let mut coins: Vec<i32> = input2.trim().split_whitespace()
                .map(|s| s.parse().expect("Please enter valid integers"))
                .collect();
    let modulo: usize = 1_000_000_007;
    let mut dp = vec![0; sum + 1];
    dp[0] = 1;


    for i in 1..=sum {
        for j in coins {
            if i >= j {
                dp[i] = (dp[i] + dp[i - j]) % modulo;
            }
        }
    }

    println!("{}", dp[sum]);
}
