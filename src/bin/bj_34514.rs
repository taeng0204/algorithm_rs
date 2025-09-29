use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().parse().unwrap();

    let chessboard: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut rook = (0, 0);
    let mut king = (0, 0);
    for i in 0..n {
        for j in 0..n {
            if chessboard[i][j] == 'R' {
                rook = (i, j);
            } else if chessboard[i][j] == 'K' {
                king = (i, j);
            }
        }
    }

    let mut distance = vec![vec![usize::MAX; n]; n];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    distance[rook.0][rook.1] = 0;
    queue.push_back(rook);

    while let Some((x, y)) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let mut stop = false;
            let mut add = 1;
            while !stop {
                let nx = x as isize + dx * add;
                let ny = y as isize + dy * add;
                if nx < 0 || nx >= n as isize || ny < 0 || ny >= n as isize {
                    break;
                }
                let nx = nx as usize;
                let ny = ny as usize;

                if chessboard[nx][ny] == 'W' || chessboard[nx][ny] == 'K' {
                    stop = true;
                } else if chessboard[nx][ny] == 'B' {
                    break;
                }

                if distance[nx][ny] > distance[x][y] + 1 {
                    distance[nx][ny] = distance[x][y] + 1;
                    queue.push_back((nx, ny));
                } else if distance[nx][ny] == distance[x][y] {
                    stop = true;
                }
                add += 1;
            }
        }
    }

    if distance[king.0][king.1] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", distance[king.0][king.1]);
    }
}
