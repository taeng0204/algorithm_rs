use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    if n % 4 != 0 {
        println!("-1");
        return;
    }

    let k = n / 4;

    let r1 = "aaab".repeat(k);
    let r2 = "dabb".repeat(k);
    let r3 = "ddcb".repeat(k);
    let r4 = "dccc".repeat(k);

    let mut result = String::with_capacity(16 * k * k + 4 * k);
    for _ in 0..k {
        for line in [&r1, &r2, &r3, &r4] {
            result.push_str(line);
            result.push('\n');
        }
    }

    print!("{result}");
}
