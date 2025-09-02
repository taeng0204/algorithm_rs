use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let c: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut prices = vec![(0, 0); n];
    for i in 0..n {
        let price: usize = iter.next().unwrap().parse().unwrap();
        let people: usize = iter.next().unwrap().parse().unwrap();
        prices[i] = (price, people);
    }

    let max_c = c + 100;
    let inf = 1_000_000_000;
    let mut dp = vec![inf; max_c + 1];
    dp[0] = 0;

    for &(price, people) in &prices {
        for x in people..=max_c {
            dp[x] = dp[x].min(dp[x - people] + price);
        }
    }

    let result = dp[c..=max_c].iter().min().unwrap();
    println!("{}", result);
}
