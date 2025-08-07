use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();
    let mut nums_input = String::new();
    io::stdin().read_line(&mut nums_input).unwrap();
    let nums: Vec<usize> = nums_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut dp = vec![1; n];

    for i in 0..n {
        for j in i..n {
            if nums[i] < nums[j] {
                dp[j] = dp[j].max(dp[i] + 1);
            }
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
