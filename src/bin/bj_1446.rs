use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let d: usize = iter.next().unwrap().parse().unwrap();
    let mut shortcuts: Vec<Vec<(usize, usize)>> = vec![Vec::new(); d + 1];

    for _ in 0..n {
        let s: usize = iter.next().unwrap().parse().unwrap();
        let e: usize = iter.next().unwrap().parse().unwrap();
        let cost: usize = iter.next().unwrap().parse().unwrap();
        if e > d {
            continue;
        }
        if e > s && cost < e - s {
            shortcuts[e].push((s, cost));
        }
    }

    const INF: usize = usize::MAX / 4;
    let mut dp = vec![INF; d + 1];
    dp[0] = 0;

    for i in 1..=d {
        dp[i] = dp[i].min(dp[i - 1] + 1);

        for &(s, cost) in &shortcuts[i] {
            dp[i] = dp[i].min(dp[s] + cost);
        }
    }

    println!("{}", dp[d]);
}
