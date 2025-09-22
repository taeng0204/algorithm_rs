use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut solutions: Vec<i64> = iter.map(|x| x.parse().unwrap()).collect();
    solutions.sort();

    let mut a = 0;
    let mut b = n - 1;
    let mut best = (solutions[a] + solutions[b], a, b);

    while a < b {
        let sum = solutions[a] + solutions[b];
        if sum == 0 {
            best = (sum, a, b);
            break;
        }

        if sum.abs() < best.0.abs() {
            best = (sum, a, b);
        }

        if sum < 0 {
            a += 1;
        } else {
            b -= 1;
        }
    }
    println!("{} {}", solutions[best.1], solutions[best.2]);
}
