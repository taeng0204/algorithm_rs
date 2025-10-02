use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut output = String::new();
    for _ in 0..n {
        let t = iter.next().unwrap().parse().unwrap();
        let mut nums: Vec<u32> = (1..=t).collect();
        for curr in 1..nums.len() {
            let prev = curr - 1;
            let sum = nums[prev] + nums[curr];
            if is_square(sum) {
                nums.swap(prev - 1, prev);
            }
        }
        let result: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
        output.push_str(&result.join(" "));
        output.push('\n');
    }
    println!("{}", output);
}

fn is_square(m: u32) -> bool {
    let sqrt = (m as f64).sqrt() as u32;
    sqrt * sqrt == m
}
