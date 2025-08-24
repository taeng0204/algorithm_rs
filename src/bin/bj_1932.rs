use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();

    let count = (1 + n) * n / 2;
    let mut triangle = Vec::with_capacity(count);
    for i in 1..=n {
        for _ in 1..=i {
            let value: i32 = iter.next().unwrap().parse().unwrap();
            triangle.push(value);
        }
    }
    for i in (0..n - 1).rev() {
        let start = (1 + i) * i / 2;
        let next = (1 + i + 1) * (i + 1) / 2;
        for j in 0..=i {
            triangle[start + j] += triangle[next + j].max(triangle[next + j + 1]);
        }
    }
    println!("{}", triangle[0]);
}
