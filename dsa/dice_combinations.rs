use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let sum: usize = input.trim().parse().expect("Not an integer");

    let modulo: usize = 1_000_000_007;
    let mut dp = vec![0; sum + 1];
    dp[0] = 1;

    for i in 1..=sum {
        for j in 1..=6 {
            if i >= j {
                dp[i] = (dp[i] + dp[i - j]) % modulo;
            }
        }
    }

    println!("{}", dp[sum]);
}
