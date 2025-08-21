use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut items = vec![(0, 0); n];
    for i in 0..n {
        let weight = iter.next().unwrap().parse().unwrap();
        let value = iter.next().unwrap().parse().unwrap();
        items[i] = (weight, value);
    }

    let mut dp = vec![0; k + 1];

    for &(weight, value) in &items {
        for w in (weight..=k).rev() {
            dp[w] = dp[w].max(dp[w - weight] + value);
        }
    }

    println!("{}", dp[k]);
}
