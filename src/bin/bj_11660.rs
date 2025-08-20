use std::fmt::Write as _;
use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut ps = vec![vec![0i64; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            let v: i64 = iter.next().unwrap().parse().unwrap();
            ps[i][j] = v + ps[i][j - 1] + ps[i - 1][j] - ps[i - 1][j - 1];
        }
    }

    let mut out = String::with_capacity(m * 14);
    for _ in 0..m {
        let x1: usize = iter.next().unwrap().parse().unwrap();
        let y1: usize = iter.next().unwrap().parse().unwrap();
        let x2: usize = iter.next().unwrap().parse().unwrap();
        let y2: usize = iter.next().unwrap().parse().unwrap();

        let result = ps[x2][y2] - ps[x1 - 1][y2] - ps[x2][y1 - 1] + ps[x1 - 1][y1 - 1];
        writeln!(&mut out, "{}", result).unwrap();
    }

    let mut stdout = io::BufWriter::new(io::stdout().lock());
    stdout.write_all(out.as_bytes()).unwrap();
}
