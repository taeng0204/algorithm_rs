use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut nums_input = String::new();
    io::stdin().read_line(&mut nums_input).unwrap();

    let mut input = input.trim().split_whitespace();
    let _: usize = input.next().unwrap().parse().unwrap();
    let m: usize = input.next().unwrap().parse().unwrap();

    let mut nums = nums_input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect::<Vec<usize>>();
    nums.sort();
    nums.dedup();

    let mut result = vec![0; m];
    dfs(0, 0, m, &nums, &mut result);
}

fn dfs(depth: usize, start: usize, m: usize, nums: &[usize], result: &mut [usize]) {
    if depth == m {
        println!(
            "{}",
            result
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        return;
    }

    for i in start..nums.len() {
        result[depth] = nums[i];
        dfs(depth + 1, i, m, nums, result);
    }
}
