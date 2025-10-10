use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let mut board: Vec<Vec<u8>> = Vec::with_capacity(n);
    let mut cur = (0, 0);
    for i in 0..n {
        let line: Vec<u8> = iter.next().unwrap().chars().map(|c| c as u8).collect();
        if let Some((j, _)) = line.iter().enumerate().find(|&(_, &c)| c == b'I') {
            cur = (i, j);
        }
        board.push(line);
    }

    let mut visited = vec![vec![false; m]; n];
    let mut queue = VecDeque::new();
    let mut count = 0;
    visited[cur.0][cur.1] = true;
    queue.push_back(cur);

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 {
                continue;
            }

            let nx = nx as usize;
            let ny = ny as usize;

            if visited[nx][ny] {
                continue;
            }

            match board[nx][ny] {
                b'O' => {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
                b'X' => {
                    visited[nx][ny] = true;
                }
                b'P' => {
                    count += 1;
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }

    if count == 0 {
        println!("TT");
        return;
    }
    println!("{}", count);
}
