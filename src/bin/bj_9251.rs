use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let first: Vec<char> = lines.next().unwrap().chars().collect();
    let second: Vec<char> = lines.next().unwrap().chars().collect();

    let mut curr = vec![0; second.len() + 1];
    let mut prev = vec![0; second.len() + 1];

    for i in 1..=first.len() {
        for j in 1..=second.len() {
            if first[i - 1] == second[j - 1] {
                curr[j] = prev[j - 1] + 1;
            } else {
                curr[j] = prev[j].max(curr[j - 1]);
            }
        }
        std::mem::swap(&mut curr, &mut prev);
    }
    println!("{}", prev[second.len()]);
}
