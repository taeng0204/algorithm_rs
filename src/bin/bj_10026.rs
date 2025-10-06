use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    let n: usize = input.next().unwrap().parse().unwrap();

    let mut matrix: Vec<Vec<char>> = vec![Vec::with_capacity(n); n];
    for i in 0..n {
        let line = input.next().unwrap();
        for c in line.chars() {
            matrix[i].push(c);
        }
    }
    let count = bfs(n, &matrix);

    let cb_matrix: Vec<Vec<char>> = matrix
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|ch| if ch == 'R' { 'G' } else { ch })
                .collect()
        })
        .collect();
    let cb_count = bfs(n, &cb_matrix);

    println!("{} {}", count, cb_count);
}

fn bfs(n: usize, matrix: &Vec<Vec<char>>) -> usize {
    let mut queue = VecDeque::new();
    let dx = [0, 0, 1, -1];
    let dy = [1, -1, 0, 0];
    let mut visited = vec![vec![false; n]; n];
    let mut count = 0;
    queue.push_back((0, 0));

    while let Some((x, y)) = queue.pop_front() {
        if visited[x][y] {
            continue;
        }
        count += 1;
        let color = matrix[x][y];
        visited[x][y] = true;
        let mut q = VecDeque::new();
        q.push_back((x, y));

        while let Some((x, y)) = q.pop_front() {
            for i in 0..4 {
                let nx = x as isize + dx[i];
                let ny = y as isize + dy[i];

                if nx < 0 || nx >= n as isize || ny < 0 || ny >= n as isize {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);

                if visited[nx][ny] {
                    continue;
                }

                if matrix[nx][ny] == color {
                    visited[nx][ny] = true;
                    q.push_front((nx, ny));
                } else {
                    queue.push_front((nx, ny));
                }
            }
        }
    }
    count
}
