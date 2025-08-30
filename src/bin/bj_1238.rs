use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    let mut adj = vec![vec![]; n + 1];
    let mut rev_adj = vec![vec![]; n + 1];
    for _ in 0..m {
        let start: usize = iter.next().unwrap().parse().unwrap();
        let end: usize = iter.next().unwrap().parse().unwrap();
        let time: usize = iter.next().unwrap().parse().unwrap();
        adj[start].push((end, time));
        rev_adj[end].push((start, time));
    }

    let mut times = vec![usize::MAX; n + 1];
    let mut heap = BinaryHeap::new();
    times[x] = 0;
    heap.push(Reverse((0, x)));
    while let Some(Reverse((time, node))) = heap.pop() {
        if time > times[node] {
            continue;
        }
        for &(next, next_time) in &adj[node] {
            let new_time = time + next_time;
            if new_time < times[next] {
                times[next] = new_time;
                heap.push(Reverse((new_time, next)));
            }
        }
    }

    let mut rev_times = vec![usize::MAX; n + 1];
    rev_times[x] = 0;
    heap.push(Reverse((0, x)));
    while let Some(Reverse((time, node))) = heap.pop() {
        if time > rev_times[node] {
            continue;
        }
        for &(next, next_time) in &rev_adj[node] {
            let new_time = time + next_time;
            if new_time < rev_times[next] {
                rev_times[next] = new_time;
                heap.push(Reverse((new_time, next)));
            }
        }
    }

    let mut max_time = 0;
    for i in 1..=n {
        max_time = max_time.max(times[i] + rev_times[i]);
    }
    println!("{}", max_time);
}
