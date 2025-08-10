use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut iter = input.trim().split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut result = vec![0; m];
    dfs(0, 1, n, m, &mut result);
}

fn dfs(depth: usize, start: usize, n: usize, m: usize, result: &mut [usize]) {
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

    for num in start..=n {
        result[depth] = num;
        dfs(depth + 1, num, n, m, result);
    }
}
