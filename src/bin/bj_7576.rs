use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let m = iter.next().unwrap().parse::<usize>().unwrap();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let mut iter = iter.map(|s| s.parse::<i32>().unwrap());

    let mut field = vec![vec![0i32; m]; n];
    let mut queue = VecDeque::new();

    for i in 0..n {
        for j in 0..m {
            let val = iter.next().unwrap();
            field[i][j] = val;
            if val == 1 {
                queue.push_back((i, j, 0));
            }
        }
    }

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut max_day = 0;

    while let Some((y, x, day)) = queue.pop_front() {
        for (dy, dx) in directions {
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if ny >= 0 && ny < n as isize && nx >= 0 && nx < m as isize {
                let (ny, nx) = (ny as usize, nx as usize);
                if field[ny][nx] == 0 {
                    field[ny][nx] = 1;
                    queue.push_back((ny, nx, day + 1));
                    max_day = max_day.max(day + 1);
                }
            }
        }
    }

    for row in &field {
        if row.iter().any(|&x| x == 0) {
            println!("-1");
            return;
        }
    }

    println!("{}", max_day);
}
