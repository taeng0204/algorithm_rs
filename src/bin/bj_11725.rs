use std::fmt::Write as _;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut parent = vec![0; n + 1];
    let mut adj: Vec<Vec<usize>> = vec![vec![]; n + 1];
    parent[1] = 1;
    for _ in 0..n - 1 {
        let (a, b): (usize, usize) = (
            iter.next().unwrap().parse().unwrap(),
            iter.next().unwrap().parse().unwrap(),
        );
        if parent[a] == 0 && parent[b] == 0 {
            adj[a].push(b);
            adj[b].push(a);
        } else if parent[a] == 0 {
            parent[a] = b;
            set_child(a, &mut adj, &mut parent);
        } else {
            parent[b] = a;
            set_child(b, &mut adj, &mut parent);
        }
    }

    let mut out = String::with_capacity(n * 14);
    for num in &parent[2..] {
        writeln!(out, "{}", num).unwrap();
    }

    let mut stdout = io::BufWriter::new(io::stdout().lock());
    stdout.write_all(out.as_bytes()).unwrap();
}

fn set_child(curr: usize, adj: &mut Vec<Vec<usize>>, parent: &mut Vec<usize>) {
    while let Some(child) = adj[curr].pop() {
        if parent[child] == 0 {
            parent[child] = curr;
            set_child(child, adj, parent);
        }
    }
}
