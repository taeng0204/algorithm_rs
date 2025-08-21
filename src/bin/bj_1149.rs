use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut dp = vec![[0i32; 3]; n + 1];

    dp[0][0] = iter.next().unwrap().parse().unwrap();
    dp[0][1] = iter.next().unwrap().parse().unwrap();
    dp[0][2] = iter.next().unwrap().parse().unwrap();

    for i in 1..n {
        let cost: [i32; 3] = [
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        ];

        dp[i][0] = cost[0] + dp[i - 1][1].min(dp[i - 1][2]);
        dp[i][1] = cost[1] + dp[i - 1][0].min(dp[i - 1][2]);
        dp[i][2] = cost[2] + dp[i - 1][0].min(dp[i - 1][1]);
    }

    println!("{}", dp[n - 1][0].min(dp[n - 1][1].min(dp[n - 1][2])));
}
