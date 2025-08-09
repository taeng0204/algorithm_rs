use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums_input = String::new();
    io::stdin().read_line(&mut nums_input).unwrap();

    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut nums: Vec<i32> = nums_input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    nums.sort();

    let mut visited = vec![false; n];
    let mut selected = vec![0; m];

    backtrack(0, m, &nums, &mut visited, &mut selected);
}

fn backtrack(
    depth: usize,
    m: usize,
    nums: &Vec<i32>,
    visited: &mut Vec<bool>,
    selected: &mut Vec<i32>,
) {
    if depth == m {
        println!(
            "{}",
            selected
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        return;
    }
    let mut prev = 0;
    for i in 0..nums.len() {
        if !visited[i] && nums[i] != prev {
            visited[i] = true;
            selected[depth] = nums[i];
            backtrack(depth + 1, m, nums, visited, selected);
            visited[i] = false;
            prev = nums[i];
        }
    }
}
