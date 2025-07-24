use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();
    let mut iter = iter.map(|s| s.parse().unwrap());

    let mut vector = vec![vec![-1; m]; n];
    let mut visited = vec![vec![false; m]; n];
    let mut start = (0, 0);

    for i in 0..n {
        for j in 0..m {
            let val = iter.next().unwrap();
            if val == 2 {
                start = (i, j);
                vector[i][j] = 0;
                visited[i][j] = true;
            } else {
                vector[i][j] = val;
            }
        }
    }

    let dir = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut togo = VecDeque::new();
    togo.push_back(start);

    while let Some((y, x)) = togo.pop_front() {
        for (dy, dx) in dir {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if ny >= 0
                && ny < n as isize
                && nx >= 0
                && nx < m as isize
                && !visited[ny as usize][nx as usize]
                && vector[ny as usize][nx as usize] == 1
            {
                let (ny, nx) = (ny as usize, nx as usize);
                visited[ny][nx] = true;
                vector[ny][nx] = vector[y][x] + 1;
                togo.push_back((ny, nx));
            }
        }
    }

    let mut output = String::new();
    for i in 0..n {
        for j in 0..m {
            let val = vector[i][j];
            if val == 1 && !visited[i][j] {
                output.push_str("-1 ");
            } else {
                output.push_str(&format!("{} ", val));
            }
        }
        output.push('\n');
    }
    print!("{}", output);
}
