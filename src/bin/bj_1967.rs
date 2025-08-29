use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut adj: Vec<Vec<(usize, usize)>> = vec![vec![]; n + 1];

    for _ in 0..n - 1 {
        let (parent, child, weight): (usize, usize, usize) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        adj[parent].push((child, weight));
        adj[child].push((parent, weight));
    }

    let mut visited = vec![false; n + 1];
    let (_, max_node) = dfs(1, 0, &adj, &mut visited);
    visited.fill(false);
    let (max_weight, _) = dfs(max_node, 0, &adj, &mut visited);
    println!("{}", max_weight);
}

fn dfs(
    root: usize,
    weight: usize,
    adj: &[Vec<(usize, usize)>],
    visited: &mut [bool],
) -> (usize, usize) {
    visited[root] = true;
    let mut max_weight = 0;
    let mut max_node = root;
    for &(child, weight) in adj[root].iter() {
        if visited[child] {
            continue;
        }
        let (sub_max_weight, sub_max_node) = dfs(child, weight, adj, visited);
        if max_weight < sub_max_weight {
            max_weight = sub_max_weight;
            max_node = sub_max_node;
        }
    }
    (max_weight + weight, max_node)
}
