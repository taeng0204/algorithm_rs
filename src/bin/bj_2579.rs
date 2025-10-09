use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let stairs: Vec<u32> = iter.map(|x| x.parse().unwrap()).collect();

    if n == 1 {
        println!("{}", stairs[0]);
        return;
    } else if n == 2 {
        println!("{}", stairs[0] + stairs[1]);
        return;
    }

    let mut dp = vec![0; n];
    dp[0] = stairs[0];
    dp[1] = stairs[0] + stairs[1];
    dp[2] = (dp[0] + stairs[2]).max(stairs[1] + stairs[2]);

    for i in 3..n {
        dp[i] = dp[i - 2] + stairs[i];
        dp[i] = dp[i].max(dp[i - 3] + stairs[i - 1] + stairs[i]);
    }

    println!("{}", dp[n - 1]);
}
