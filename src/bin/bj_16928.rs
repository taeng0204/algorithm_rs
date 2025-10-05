use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut board: Vec<usize> = (0..=100).collect();
    let mut dist: Vec<usize> = vec![usize::MAX; 101];

    for _ in 0..(n + m) {
        let s: usize = iter.next().unwrap().parse().unwrap();
        let e: usize = iter.next().unwrap().parse().unwrap();
        board[s] = e;
    }

    let mut queue: VecDeque<usize> = VecDeque::new();
    dist[1] = 0;
    queue.push_back(1);

    while let Some(x) = queue.pop_front() {
        for i in 1..=6 {
            if x + i > 100 {
                continue;
            }
            if dist[x] + 1 < dist[board[x + i]] {
                dist[board[x + i]] = dist[x] + 1;
                queue.push_back(board[x + i]);
            }
        }
    }
    println!("{}", dist[100]);
}
