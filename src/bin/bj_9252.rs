use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.lines();
    let first: Vec<char> = iter.next().unwrap().chars().collect();
    let second: Vec<char> = iter.next().unwrap().chars().collect();

    let mut dp = vec![vec![0; second.len() + 1]; first.len() + 1];

    for i in 1..=first.len() {
        for j in 1..=second.len() {
            if first[i - 1] == second[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    if dp[first.len()][second.len()] == 0 {
        println!("0");
    } else {
        let mut result = String::new();
        let mut i = first.len();
        let mut j = second.len();

        while i > 0 && j > 0 {
            if first[i - 1] == second[j - 1] {
                result.push(first[i - 1]);
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                i -= 1;
            } else {
                j -= 1;
            }
        }
        println!("{}", dp[first.len()][second.len()]);
        println!("{}", result.chars().rev().collect::<String>());
    }
}
