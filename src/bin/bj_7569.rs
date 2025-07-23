use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let m = iter.next().unwrap().parse::<usize>().unwrap();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let h = iter.next().unwrap().parse::<usize>().unwrap();
    let mut iter = iter.map(|s| s.parse::<i32>().unwrap());

    let mut field = vec![vec![vec![0i32; m]; n]; h];
    let mut queue = VecDeque::new();

    for i in 0..h {
        for j in 0..n {
            for k in 0..m {
                let val = iter.next().unwrap();
                field[i][j][k] = val;
                if val == 1 {
                    queue.push_back((i, j, k, 0));
                }
            }
        }
    }

    let directions = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    let mut max_day = 0;

    while let Some((z, y, x, day)) = queue.pop_front() {
        for (dz, dy, dx) in directions {
            let nz = z as isize + dz;
            let ny = y as isize + dy;
            let nx = x as isize + dx;

            if nz >= 0
                && nz < h as isize
                && ny >= 0
                && ny < n as isize
                && nx >= 0
                && nx < m as isize
            {
                let (nz, ny, nx) = (nz as usize, ny as usize, nx as usize);
                if field[nz][ny][nx] == 0 {
                    field[nz][ny][nx] = 1;
                    queue.push_back((nz, ny, nx, day + 1));
                    max_day = max_day.max(day + 1);
                }
            }
        }
    }

    for z in &field {
        for y in z {
            if y.iter().any(|&x| x == 0) {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", max_day);
}
