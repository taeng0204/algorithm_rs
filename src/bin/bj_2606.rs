use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];

    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        adj[a].push(b);
        adj[b].push(a);
    }

    let mut visited: Vec<bool> = vec![false; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(1);

    while let Some(u) = queue.pop_front() {
        if !visited[u] {
            visited[u] = true;
            for &v in &adj[u] {
                if !visited[v] {
                    queue.push_back(v);
                }
            }
        }
    }
    let count = visited.iter().filter(|&&x| x).count();
    println!("{}", count - 1);
}
