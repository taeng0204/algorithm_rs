use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut adj = vec![vec![]; n + 1];
    for _ in 1..n {
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut tree = vec![vec![]; n + 1];
    let mut stack = vec![(r, 0)];
    while let Some((node, parent)) = stack.pop() {
        for &target in adj[node].iter() {
            if target != parent {
                tree[node].push(target);
                stack.push((target, node));
            }
        }
    }

    let mut node_count = vec![1; n + 1];
    count_subtree(r, &tree, &mut node_count);

    let mut output = String::new();
    for _ in 0..q {
        let root: usize = iter.next().unwrap().parse().unwrap();
        output.push_str(&format!("{}\n", node_count[root]));
    }

    io::stdout().write_all(output.as_bytes()).unwrap();
}

fn count_subtree(root: usize, tree: &Vec<Vec<usize>>, subtree: &mut Vec<usize>) {
    for &child in tree[root].iter() {
        count_subtree(child, tree, subtree);
        subtree[root] += subtree[child];
    }
}
