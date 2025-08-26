use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();
    let mut out = String::with_capacity(t * 8);

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();

        let mut sticker = vec![vec![0i64; n], vec![0i64; n]];
        for r in 0..2 {
            for c in 0..n {
                sticker[r][c] = iter.next().unwrap().parse::<i64>().unwrap();
            }
        }

        if n == 1 {
            let result = sticker[0][0].max(sticker[1][0]);
            out.push_str(&format!("{result}\n"));
            continue;
        }

        let mut dp0 = 0i64;
        let mut dp1 = sticker[0][0];
        let mut dp2 = sticker[1][0];

        for i in 1..n {
            let ndp0 = dp0.max(dp1).max(dp2);
            let ndp1 = dp0.max(dp2) + sticker[0][i];
            let ndp2 = dp0.max(dp1) + sticker[1][i];
            dp0 = ndp0;
            dp1 = ndp1;
            dp2 = ndp2;
        }

        let result = dp0.max(dp1).max(dp2);
        out.push_str(&format!("{result}\n"));
    }

    let mut w = io::BufWriter::new(io::stdout().lock());
    w.write_all(out.as_bytes()).unwrap();
}
