use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let a = iter.next().unwrap().parse::<u64>().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();

    println!("{}", get_mod_pow(a % c, b, c));
}

fn get_mod_pow(a: u64, b: u64, c: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let mut half = get_mod_pow(a, b / 2, c);
    half = (half * half) % c;
    if b % 2 == 0 { half } else { (half * a) % c }
}
