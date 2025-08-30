use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let v: usize = iter.next().unwrap().parse().unwrap();
    let e: u32 = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut adj: Vec<Vec<(usize, u32)>> = vec![vec![]; v + 1];
    for _ in 0..e {
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let w: u32 = iter.next().unwrap().parse().unwrap();

        adj[u].push((v, w));
    }

    let mut distance = vec![u32::MAX; v + 1];
    let mut min_queue = BinaryHeap::new();
    distance[k] = 0;
    min_queue.push(Reverse((0, k)));

    while let Some(Reverse((dist, target))) = min_queue.pop() {
        if dist > distance[target] {
            continue;
        }
        for &(v, w) in &adj[target] {
            let new_dist = dist + w;
            if new_dist < distance[v] {
                distance[v] = new_dist;
                min_queue.push(Reverse((new_dist, v)));
            }
        }
    }

    let mut output = String::new();
    for d in &distance[1..] {
        if *d == u32::MAX {
            output.push_str("INF\n");
        } else {
            output.push_str(&format!("{}\n", d));
        }
    }
    io::stdout().write_all(output.as_bytes()).unwrap();
}
